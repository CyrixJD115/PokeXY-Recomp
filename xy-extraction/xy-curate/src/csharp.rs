//! C → C# deobfuscation/cleanup pipeline for the Ghidra decompilation corpus.
//!
//! Ghidra's per-function C output is highly mechanical, so most of it converts
//! mechanically into C# `unsafe` code:
//!
//! * All *value* integer types (`int`, `uint`, `undefined1..8`, …) widen to
//!   `long`. Every integer arg/return/assignment combination then converts
//!   implicitly, which is what makes bulk conversion type-check at all. The
//!   original width lives in the corresponding `.c` file. *Pointer* base
//!   types keep their exact C widths (`int*` stays `int*`) because C# pointer
//!   arithmetic scales identically.
//! * `CONCATmn(a, b)` rewrites to `Gh.CONCATmn(a, b)` helpers.
//! * `&stack0xfffffffffffffff8` stack references rewrite to `__stack + (-8)`
//!   over a `stackalloc` scratch buffer (frame-precise aliasing is out of
//!   scope; the decompilation itself only sees slots).
//! * Data labels (`DAT_…`, `PTR_…`, `LAB_…`, `uRam…`) become module-level
//!   fields: value labels as `long` placeholders, pointer labels as `byte*`
//!   pinned to their virtual address. String labels (`s_…`, `u_…`) become
//!   pointer placeholders.
//! * Functions using function pointers (`code`), variadics, or indirect
//!   calls are emitted as stubs with their original C attached, commented.
//! * Calls to `FUN_…` outside the module get throwing stubs in
//!   `Imports.g.cs` so intra-module references still resolve.
//!
//! The output is a *readable reconstruction*, not a verified reimplementation:
//! the original C next to every function stays authoritative.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Per-module conversion statistics.
#[derive(Debug, Default)]
pub struct ModuleStats {
    pub module: String,
    pub total: usize,
    pub converted: usize,
    pub unconverted: usize,
    pub external_stubs: usize,
    pub reasons: BTreeMap<String, usize>,
}

impl ModuleStats {
    /// One-line summary for logs and the curated README.
    pub fn summary_line(&self) -> String {
        format!(
            "{}: {}/{} functions converted, {} stubbed, {} external stubs",
            self.module, self.converted, self.total, self.unconverted, self.external_stubs
        )
    }
}

/// A parsed Ghidra function file (`// FUN_x @ addr` + C89 body).
struct FnSrc {
    name: String,
    addr: String,
    /// Raw signature text (return type + name + parameter list).
    sig: String,
    /// Raw declaration lines (leading `type name;` region inside the body).
    locals: Vec<String>,
    /// Raw body lines (excluding the closing brace).
    body: Vec<String>,
}

// ---------------------------------------------------------------------------
// Type mapping
// ---------------------------------------------------------------------------

/// Ghidra scalar type → C# pointee type for pointer contexts (exact widths).
fn pointee_type(word: &str) -> Option<&'static str> {
    Some(match word {
        "undefined" | "undefined1" | "uchar" | "byte" => "byte",
        "undefined2" | "ushort" => "ushort",
        "undefined4" | "uint" | "dword" => "uint",
        "undefined8" | "ulong" | "ulonglong" => "ulong",
        "char" => "sbyte",
        "short" => "short",
        "int" => "int",
        "long" | "longlong" => "long",
        "float" => "float",
        "double" => "double",
        "bool" => "bool",
        _ => return None,
    })
}

/// Ghidra scalar type → C# value type (integers widen to `long`).
fn value_type(word: &str) -> Option<&'static str> {
    Some(match word {
        "undefined" | "undefined1" | "undefined2" | "undefined4" | "undefined8" | "uchar"
        | "ushort" | "uint" | "ulong" | "ulonglong" | "char" | "short" | "int" | "long"
        | "longlong" | "byte" | "sbyte" | "dword" => "long",
        "float" => "float",
        "double" => "double",
        "bool" => "bool",
        "void" => "void",
        _ => return None,
    })
}

fn is_type_word(word: &str) -> bool {
    pointee_type(word).is_some() || matches!(word, "code" | "wchar_t")
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Ghidra data/label identifiers: `DAT_0029f4a0` (hex-addressed), `PTR_…`,
/// `LAB_…`, `uRam0029f4a0`, and string labels `s_…`/`u_…` (name-addressed).
/// Returns `(kind, address_or_name)`.
fn label_of(word: &str) -> Option<(&str, &str)> {
    if let Some(rest) = word.strip_prefix("uRam") {
        if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Some(("uRam", rest));
        }
        return None;
    }
    let (kind, rest) = word.split_once('_')?;
    match kind {
        "DAT" | "PTR" | "LAB"
            if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_hexdigit()) =>
        {
            Some((kind, rest))
        }
        "s" | "u" if rest.len() > 3 && rest.chars().all(is_ident_continue) => Some((kind, rest)),
        _ => None,
    }
}

fn is_fun_ref(word: &str) -> bool {
    let Some(rest) = word.strip_prefix("FUN_") else {
        return false;
    };
    rest.len() == 8 && rest.bytes().all(|b| b.is_ascii_hexdigit())
}

// ---------------------------------------------------------------------------
// Single-line walking
// ---------------------------------------------------------------------------

#[derive(Default)]
struct WalkOut {
    text: String,
    uses_code: bool,
    uses_stack: bool,
    stack_max: i64,
    labels: BTreeMap<String, bool>, // name -> used as pointer
    funs: BTreeSet<String>,
    concats: BTreeSet<String>,
}

/// Rewrite one line of C: map types, rewrite CONCAT/`&stack0x`, collect
/// label and FUN_ references. `last_star` tracks whether the previously
/// emitted non-space character was a `*` (cast-deref label detection).
fn walk_line(line: &str, out: &mut WalkOut) {
    let bytes = line.as_bytes();
    let mut i = 0usize;
    let mut last_star = false;
    let push_char = |out: &mut WalkOut, last_star: &mut bool, c: char| {
        if !c.is_whitespace() {
            *last_star = c == '*';
        }
        out.text.push(c);
    };
    while i < bytes.len() {
        let c = bytes[i] as char;
        if is_ident_start(c) {
            let start = i;
            while i < bytes.len() && is_ident_continue(bytes[i] as char) {
                i += 1;
            }
            let word = &line[start..i];
            let pointer = bytes
                .get(i..)
                .is_some_and(|rest| rest.iter().find(|b| !b.is_ascii_whitespace()) == Some(&b'*'));
            let next_is_add = bytes.get(i..).is_some_and(|rest| {
                matches!(
                    rest.iter().find(|b| !b.is_ascii_whitespace()),
                    Some(b'+') | Some(b'-')
                )
            });
            if let Some((_kind, _addr)) = label_of(word) {
                let next_is_index = bytes.get(i..).is_some_and(|rest| {
                    matches!(rest.iter().find(|b| !b.is_ascii_whitespace()), Some(b'['))
                });
                let used_as_ptr = pointer || last_star || next_is_add || next_is_index;
                let entry = out.labels.entry(word.to_string()).or_insert(false);
                if used_as_ptr {
                    *entry = true;
                }
                out.text.push_str(word);
            } else if is_fun_ref(word) {
                out.funs.insert(word.to_string());
                out.text.push_str(word);
            } else if let Some(mapped) = gh_type(word, pointer) {
                out.text.push_str(mapped);
            } else if let Some(rest) = word.strip_prefix("CONCAT") {
                if rest.len() == 2 && rest.bytes().all(|b| b.is_ascii_digit()) {
                    out.concats.insert(format!("CONCAT{rest}"));
                    out.text.push_str("Gh.");
                    out.text.push_str(word);
                } else {
                    out.text.push_str(word);
                }
            } else if word == "code" {
                out.uses_code = true;
                out.text.push_str("void*");
            } else if word == "wchar_t" {
                out.text.push_str("ushort");
            } else {
                out.text.push_str(word);
            }
            last_star = false;
        } else if line[i..].starts_with("&stack0x") {
            let hex = &line[i + 8..(i + 24).min(line.len())];
            if hex.len() == 16 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
                let raw = u64::from_str_radix(hex, 16).unwrap_or(0);
                let off = raw as i64;
                out.uses_stack = true;
                out.stack_max = out.stack_max.max(off.abs());
                out.text.push_str(&format!("(__stack + ({off}))"));
                i += 8 + 16;
            } else {
                push_char(&mut *out, &mut last_star, '&');
                i += 1;
            }
        } else {
            push_char(&mut *out, &mut last_star, c);
            i += 1;
        }
    }
}

/// Type mapping used inside expressions and signatures: pointer context uses
/// exact pointee widths, value context widens integers to `long`.
fn gh_type(word: &str, pointer: bool) -> Option<&'static str> {
    if pointer {
        pointee_type(word)
    } else {
        value_type(word)
    }
}

// ---------------------------------------------------------------------------
// Function file parsing
// ---------------------------------------------------------------------------

/// Parse one Ghidra `.c` export into its parts. Returns `None` when the file
/// does not follow the `// FUN_x @ addr` header convention.
fn parse_fn(text: &str) -> Option<FnSrc> {
    let mut lines = text.lines().skip_while(|l| l.trim().is_empty());
    let header = lines.next()?.trim();
    // "// FUN_00100048 @ 00100048"
    let rest = header.strip_prefix("// ")?;
    let name = rest.split_whitespace().next()?.to_string();
    let addr = rest.split("@ ").nth(1)?.trim().to_string();

    let mut sig = String::new();
    let mut in_comment = false;
    for line in lines.by_ref() {
        let t = line.trim();
        // Ghidra drops warning comments (`/* WARNING: Type propagation … */`)
        // between the header and the signature — skip or strip them.
        if in_comment {
            if t.contains("*/") {
                in_comment = false;
            }
            continue;
        }
        if t.starts_with("/*") {
            if !t.contains("*/") {
                in_comment = true;
            }
            continue;
        }
        if t.is_empty() {
            continue;
        }
        let cleaned = strip_block_comments(t);
        sig.push_str(&cleaned);
        sig.push(' ');
        if cleaned.contains(')') {
            break;
        }
    }
    // Consume up to and including the opening brace.
    let mut saw_open = false;
    for line in lines.by_ref() {
        let t = line.trim();
        if t == "{" {
            saw_open = true;
            break;
        }
        if t.contains('{') {
            saw_open = true;
            break;
        }
        if !t.is_empty() {
            break;
        }
    }
    if !saw_open {
        return None;
    }

    // Locals: leading declaration lines (comments pass through).
    let mut locals = Vec::new();
    let mut body_start: Option<usize> = None;
    let raw: Vec<&str> = lines.collect();
    for (idx, line) in raw.iter().enumerate() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        if is_decl_line(t) || t.starts_with("/*") || t.starts_with("//") {
            locals.push(t.to_string());
            continue;
        }
        body_start = Some(idx);
        break;
    }
    let start = match body_start {
        Some(s) => s,
        // Declaration-only function (no body statements).
        None if locals.is_empty()
            || locals
                .iter()
                .all(|l| l.starts_with("/*") || l.starts_with("//")) =>
        {
            return Some(FnSrc {
                name,
                addr,
                sig: sig.trim().to_string(),
                locals,
                body: Vec::new(),
            });
        }
        // Only declarations / comments — body runs to the end (empty).
        None => raw.len(),
    };
    let mut body: Vec<String> = raw[start..]
        .iter()
        .map(|l| l.trim_end().to_string())
        .collect();
    // Drop trailing blank lines and the final closing brace.
    while let Some(last) = body.last() {
        if last.trim().is_empty() {
            body.pop();
        } else {
            if last.trim() == "}" {
                body.pop();
            }
            break;
        }
    }
    Some(FnSrc {
        name,
        addr,
        sig: sig.trim().to_string(),
        locals,
        body,
    })
}

/// Remove non-nested `/* … */` segments from a line.
fn strip_block_comments(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(open) = rest.find("/*") {
        out.push_str(&rest[..open]);
        match rest[open..].find("*/") {
            Some(close) => rest = &rest[open + close + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

/// `undefined4 uVar1;` / `short *psVar1;` / `undefined1 auStack_8 [4];` /
/// `int iStack_c = 0;` — a Ghidra local declaration.
fn is_decl_line(t: &str) -> bool {
    if !t.ends_with(';') {
        return false;
    }
    let core = t.trim_end_matches(';');
    let core = match core.split_once('=') {
        Some((lhs, _)) => lhs.trim_end(),
        None => core,
    };
    let core = core.trim_end();
    // Strip a trailing [N] array dimension.
    let core = match core.rfind('[') {
        Some(p) => core[..p].trim_end(),
        None => core,
    };
    let mut tokens = core.split_whitespace();
    let Some(first) = tokens.next() else {
        return false;
    };
    if !is_type_word(first.trim_end_matches('*')) {
        return false;
    }
    let Some(name) = tokens.next() else {
        return false; // bare cast or return type — not a declaration
    };
    let name = name.trim_start_matches('*');
    match name.chars().next() {
        Some(c) if is_ident_start(c) && name.chars().all(is_ident_continue) => {}
        _ => return false,
    }
    tokens.next().is_none()
}

// ---------------------------------------------------------------------------
// Signature / type-expression conversion
// ---------------------------------------------------------------------------

/// Convert a type expression like `short *` / `undefined4 **` / `float`.
/// Stars may appear before or after the base word; the base decides exact vs
/// widened mapping (`stars > 0` ⇒ pointee widths).
fn convert_type_expr(expr: &str) -> String {
    let mut base_raw = String::new();
    let mut stars = 0usize;
    for token in expr.split_whitespace() {
        stars += token.matches('*').count();
        let word: String = token.chars().filter(|c| *c != '*').collect();
        if !word.is_empty() && base_raw.is_empty() {
            base_raw = word;
        }
    }
    let base: String = if base_raw == "code" {
        "void*".to_string()
    } else if base_raw == "wchar_t" {
        "ushort".to_string()
    } else if base_raw.is_empty() {
        if stars > 0 {
            return "long*".to_string();
        }
        "long".to_string()
    } else {
        match gh_type(&base_raw, stars > 0) {
            Some(t) => t.to_string(),
            None => base_raw.clone(),
        }
    };
    if base == "void" && stars == 0 {
        return "void".to_string();
    }
    format!("{base}{}", "*".repeat(stars))
}

/// Parse + convert `short * FUN_x(int *a, int b)` → ret `short*`, params
/// [("int*", "a"), ("long", "b")].
fn convert_sig(sig: &str) -> Option<CsSig> {
    let open = sig.find('(')?;
    let close = sig.rfind(')')?;
    if close < open {
        return None;
    }
    let head = sig[..open].trim();
    let args = &sig[open + 1..close];

    // head = "<ret tokens> <name>"; name = the last identifier.
    let mut head_tokens: Vec<&str> = head.split_whitespace().collect();
    let _name_raw = head_tokens.pop()?;
    let ret = convert_type_expr(&head_tokens.join(" "));

    let mut params = Vec::new();
    let mut variadic = false;
    for (idx, arg) in args.split(',').enumerate() {
        let arg = arg.trim();
        if arg.is_empty() || arg == "void" {
            continue;
        }
        if arg == "..." {
            variadic = true;
            continue;
        }
        let mut tokens: Vec<&str> = arg.split_whitespace().collect();
        let last = tokens.pop()?;
        let last_stars = last.matches('*').count();
        let last_word: String = last.chars().filter(|c| *c != '*').collect();
        let (ptype, pname) = if tokens.is_empty() || is_type_word(&last_word) {
            // No name token — synthesize one.
            (convert_type_expr(arg), format!("p{idx}"))
        } else {
            let mut type_raw = tokens.join(" ");
            for _ in 0..last_stars {
                type_raw.push('*');
            }
            (
                convert_type_expr(type_raw.trim()),
                sanitize_ident(&last_word),
            )
        };
        params.push((ptype, pname));
    }
    Some(CsSig {
        ret,
        params,
        variadic,
    })
}

struct CsSig {
    ret: String,
    params: Vec<(String, String)>, // (type, name)
    variadic: bool,
}

fn sanitize_ident(name: &str) -> String {
    let cleaned: String = name
        .chars()
        .map(|c| if is_ident_continue(c) { c } else { '_' })
        .collect();
    if cleaned.is_empty() || !is_ident_start(cleaned.chars().next().unwrap_or('_')) {
        format!("v_{cleaned}")
    } else if is_cs_keyword(&cleaned) {
        format!("@{cleaned}")
    } else {
        cleaned
    }
}

fn is_cs_keyword(w: &str) -> bool {
    matches!(
        w,
        "int"
            | "uint"
            | "long"
            | "ulong"
            | "short"
            | "ushort"
            | "byte"
            | "sbyte"
            | "float"
            | "double"
            | "bool"
            | "void"
            | "char"
            | "object"
            | "string"
            | "params"
            | "base"
            | "this"
            | "new"
            | "return"
            | "switch"
            | "case"
    )
}

// ---------------------------------------------------------------------------
// Module conversion
// ---------------------------------------------------------------------------

struct EmittedFn {
    addr: String,
    name: String,
    comment: String,
    code: String, // full emitted method
}

struct StubFn {
    reason: String,
    code: String,
}

/// Module-wide call information threaded into per-function conversion.
struct FnCtx {
    call_args: BTreeMap<String, usize>,
    result_used: BTreeSet<String>,
    labels_ptr: BTreeMap<String, bool>,
}

/// `iVar1 = FUN_0010abcd(...)` (incl. compound `(x = FUN_(…), …)`) — the
/// function's return value is consumed, so a `void` thunk must widen to long.
fn assigned_fun(line: &str) -> Option<String> {
    let mut from = 0usize;
    while let Some(pos) = line[from..].find('=') {
        let at = from + pos;
        let before = line[..at].chars().last();
        if !matches!(
            before,
            Some('=')
                | Some('!')
                | Some('<')
                | Some('>')
                | Some('/')
                | Some('+')
                | Some('-')
                | Some('*')
                | Some('&')
                | Some('|')
        ) {
            let mut rest = line[at + 1..].trim_start();
            // Skip an interleaved cast: `= (ushort *)FUN_x(...)`.
            if rest.starts_with('(') {
                if let Some(close) = rest.find(')') {
                    rest = rest[close + 1..].trim_start();
                }
            }
            if let Some(name_part) = rest.strip_prefix("FUN_") {
                let end = name_part
                    .char_indices()
                    .find(|(_, c)| !is_ident_continue(*c))
                    .map(|(i, _)| i)
                    .unwrap_or(name_part.len());
                return Some(format!("FUN_{}", &name_part[..end]));
            }
        }
        from = at + 1;
    }
    None
}

/// Ghidra-synthetic helper calls (`VectorUnsignedToFloat(…)`) — CamelCase
/// callees that are neither FUN_ labels nor control keywords.
fn synthetic_call(line: &str) -> Option<(String, usize)> {
    let bytes = line.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c.is_ascii_uppercase() {
            let start = i;
            while i < bytes.len() && is_ident_continue(bytes[i] as char) {
                i += 1;
            }
            let word = &line[start..i];
            if i < bytes.len() && bytes[i] == b'(' {
                let known = matches!(
                    word,
                    "IF" | "DAT" | "PTR" | "LAB" | "CONCAT" | "SUB" | "EXTRACT" | "SEXT" | "ZEXT"
                ) || word.starts_with("CONCAT")
                    || is_fun_ref(word);
                if !known && !is_type_word(word) {
                    let args = count_call_args(line, word).unwrap_or(0);
                    return Some((word.to_string(), args));
                }
            }
        } else {
            i += 1;
        }
    }
    None
}

/// Convert all `*.c` function files under `module_dir` (one Ghidra module,
/// e.g. `code_c` or `cro_c/DllBattle`) into a C# module directory.
///
/// `module_name` is the C# class/namespace name (e.g. `Main`, `DllBag`).
/// `symbols` maps 8-hex function address → real export name (annotation only).
pub fn convert_module(
    module_name: &str,
    module_dir: &Path,
    out_dir: &Path,
    symbols: &BTreeMap<String, String>,
) -> io::Result<ModuleStats> {
    let mut paths: Vec<PathBuf> = fs::read_dir(module_dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "c"))
        .collect();
    paths.sort();
    let mut stats = ModuleStats {
        module: module_name.to_string(),
        ..Default::default()
    };

    // Pass 1: parse everything, collect defined FUN_ names.
    let mut parsed: Vec<FnSrc> = Vec::new();
    for p in &paths {
        let Ok(text) = fs::read_to_string(p) else {
            *stats
                .reasons
                .entry("unreadable file".to_string())
                .or_default() += 1;
            continue;
        };
        match parse_fn(&text) {
            Some(f) => parsed.push(f),
            None => {
                *stats
                    .reasons
                    .entry("unparsed file".to_string())
                    .or_default() += 1
            }
        }
    }
    let defined: BTreeSet<String> = parsed.iter().map(|f| f.name.clone()).collect();

    // Pass 2: module-wide scans (label usage, CONCAT variants, externals).
    let mut labels: BTreeMap<String, bool> = BTreeMap::new();
    let mut concats: BTreeSet<String> = BTreeSet::new();
    let mut externals: BTreeMap<String, usize> = BTreeMap::new(); // name -> max args
    let mut call_args: BTreeMap<String, usize> = BTreeMap::new(); // all FUN_ max args
    let mut result_used: BTreeSet<String> = BTreeSet::new(); // FUN_ whose result is assigned
    let mut synthetics: BTreeMap<String, usize> = BTreeMap::new(); // Ghidra-synthetic helpers
    for f in &parsed {
        for line in &f.body {
            if let Some(name) = assigned_fun(line) {
                result_used.insert(name);
            }
            if let Some((callee, args)) = synthetic_call(line) {
                synthetics
                    .entry(callee)
                    .and_modify(|e| *e = (*e).max(args))
                    .or_insert(args);
            }
        }
        let mut out = WalkOut::default();
        for line in f.locals.iter().chain(f.body.iter()) {
            walk_line(line, &mut out);
        }
        for fun in &out.funs {
            let args = f
                .body
                .iter()
                .find_map(|l| count_call_args(l, fun))
                .unwrap_or(0);
            call_args
                .entry(fun.clone())
                .and_modify(|e| *e = (*e).max(args))
                .or_insert(args);
        }
        for (name, ptr) in out.labels {
            let e = labels.entry(name).or_insert(false);
            if ptr {
                *e = true;
            }
        }
        concats.extend(out.concats);
        for fun in &out.funs {
            if !defined.contains(fun) {
                let args = f
                    .body
                    .iter()
                    .find_map(|l| count_call_args(l, fun))
                    .unwrap_or(0);
                externals
                    .entry(fun.clone())
                    .and_modify(|e| *e = (*e).max(args))
                    .or_insert(args);
            }
        }
    }

    // Pass 3: convert each function.
    let fctx = FnCtx {
        call_args,
        result_used,
        labels_ptr: labels.clone(),
    };
    let mut converted: Vec<EmittedFn> = Vec::new();
    let mut stubs: Vec<StubFn> = Vec::new();
    for f in &parsed {
        stats.total += 1;
        match convert_fn(f, &fctx) {
            Ok(code) => {
                stats.converted += 1;
                let comment = symbols
                    .get(&f.addr)
                    .map(|s| format!("exported as `{s}`"))
                    .unwrap_or_default();
                converted.push(EmittedFn {
                    addr: f.addr.clone(),
                    name: f.name.clone(),
                    comment,
                    code,
                });
            }
            Err(reason) => {
                *stats.reasons.entry(reason.clone()).or_default() += 1;
                stats.unconverted += 1;
                stubs.push(StubFn {
                    reason,
                    code: emit_stub(f, symbols.get(&f.addr), &fctx.call_args, &fctx.result_used),
                });
            }
        }
    }
    stats.external_stubs = externals.len();

    write_module_files(
        module_name,
        out_dir,
        &labels,
        &concats,
        &externals,
        &converted,
        &stubs,
        &stats,
        &synthetics,
    )?;
    Ok(stats)
}

/// Conversion of one function. `Err(reason)` = emit as stub with the original
/// C attached.
fn convert_fn(f: &FnSrc, ctx: &FnCtx) -> Result<String, String> {
    let mut sig = convert_sig(&f.sig).ok_or_else(|| "unparseable signature".to_string())?;
    if sig.variadic {
        return Err("variadic signature".to_string());
    }
    // Ghidra thunk functions often decompile with fewer parameters than
    // call sites pass — widen with a `params` tail so both compile.
    if let Some(max_args) = ctx.call_args.get(&f.name) {
        if *max_args > sig.params.len() {
            sig.params
                .push(("params long[]".to_string(), "a_extra".to_string()));
        }
    }
    // Void thunks whose result is consumed must return something.
    let mut widen_void = false;
    if sig.ret == "void" && ctx.result_used.contains(&f.name) {
        sig.ret = "long".to_string();
        widen_void = true;
    }

    // Variable types (name → pointee base) for deref/indexed assign fixing,
    // plus the set of all pointer-typed identifiers for call-arg conversion.
    let mut var_types: BTreeMap<String, String> = BTreeMap::new();
    let mut ptr_pointee: BTreeMap<String, String> = BTreeMap::new();
    let mut local_types: BTreeMap<String, String> = BTreeMap::new();
    for (t, n) in &sig.params {
        if let Some(base) = t.strip_suffix('*') {
            ptr_pointee.insert(n.clone(), base.to_string());
            if pointee_is_scalar(base) {
                var_types.insert(n.clone(), base.to_string());
            }
        }
        local_types.insert(n.clone(), t.clone());
    }

    // Locals → declarations; collect stack use.
    let mut stack_max = 0i64;
    let mut uses_stack = false;
    let mut local_lines: Vec<String> = Vec::new();
    for raw in &f.locals {
        let t = raw.trim();
        if t.starts_with("/*") || t.starts_with("//") {
            local_lines.push(format!("    {t}"));
            continue;
        }
        if let Some(d) = convert_decl(t, &mut stack_max, &mut uses_stack) {
            local_lines.push(format!("    {}", d.decl));
            if let Some((base, is_ptr)) = d.info {
                let name = d.name;
                if is_ptr {
                    ptr_pointee.insert(name.clone(), base.clone());
                    local_types.insert(name, format!("{base}*"));
                } else {
                    if pointee_is_scalar(&base) {
                        var_types.insert(name.clone(), base.clone());
                    }
                    local_types.insert(name, base);
                }
            }
        }
    }

    // Body conversion.
    let mut body_lines: Vec<String> = Vec::new();
    let mut indirect = false;
    let mut uses_code = false;
    for raw in &f.body {
        let mut out = WalkOut::default();
        walk_line(raw, &mut out);
        uses_code |= out.uses_code;
        if out.uses_stack {
            uses_stack = true;
            stack_max = stack_max.max(out.stack_max);
        }
        let stripped: String = raw.chars().filter(|c| !c.is_whitespace()).collect();
        if stripped.starts_with("(*") || stripped.contains("**)(") {
            indirect = true;
        }
        let mut mapped = out.text.trim_end().to_string();
        mapped = rewrite_char_literals(&mapped);
        mapped = fix_assign(
            &mapped,
            &var_types,
            &ptr_pointee,
            &local_types,
            &ctx.labels_ptr,
        );
        mapped = fix_call_args(&mapped, &ptr_pointee, &ctx.labels_ptr);
        if widen_void && stripped == "return;" {
            mapped = "return 0;".to_string();
        }
        if let Some((stmts, rewritten)) = split_condition_commas(&mapped) {
            for st in &stmts {
                let terminated = if st.ends_with(';') {
                    st.clone()
                } else {
                    format!("{st};")
                };
                body_lines.push(format!("    {terminated}"));
            }
            if !rewritten.is_empty() {
                body_lines.push(format!("    {rewritten}"));
            }
        } else {
            body_lines.push(format!("    {mapped}"));
        }
    }

    if uses_code {
        return Err("function pointers (code)".to_string());
    }
    if indirect {
        return Err("indirect call".to_string());
    }

    let params = sig
        .params
        .iter()
        .map(|(t, n)| format!("{t} {n}"))
        .collect::<Vec<_>>()
        .join(", ");
    let mut code = String::new();
    let _ = writeln!(
        code,
        "    public static unsafe {} {}({}) {{",
        sig.ret, f.name, params
    );
    if uses_stack {
        let size = stack_size(stack_max);
        let _ = writeln!(code, "        byte* __stack = stackalloc byte[{size}];");
    }
    for line in local_lines {
        let _ = writeln!(code, "{line}");
    }
    for line in &body_lines {
        let _ = writeln!(code, "{line}");
    }
    code.push_str("    }\n");
    Ok(code)
}

/// Round a stack frame requirement up to a sane stackalloc size.
fn stack_size(max_abs: i64) -> usize {
    let need = if max_abs <= 0 {
        0x10
    } else {
        max_abs as usize + 8
    };
    need.div_ceil(8) * 8
}

/// One converted local declaration.
struct DeclParts {
    decl: String,
    name: String,
    /// `(base, is_pointer)` — `base` is the C# base type, `is_pointer`
    /// distinguishes `T* x` from `T x`.
    info: Option<(String, bool)>,
}

/// Convert one declaration line to C#. Updates stack-usage info.
fn convert_decl(t: &str, stack_max: &mut i64, uses_stack: &mut bool) -> Option<DeclParts> {
    let core = t.trim_end_matches(';');
    let (lhs, init) = match core.split_once('=') {
        Some((l, r)) => (l.trim_end().to_string(), Some(r.trim().to_string())),
        None => (core.trim().to_string(), None),
    };
    // Array dimension.
    let (lhs, array) = match lhs.rfind('[') {
        Some(p) => {
            let dim = lhs[p..].trim_matches(|c| c == '[' || c == ']');
            (lhs[..p].trim_end().to_string(), Some(dim.to_string()))
        }
        None => (lhs, None),
    };
    // Split type (with stars) from name: the name is the final identifier.
    let mut tokens: Vec<&str> = lhs.split_whitespace().collect();
    let name_raw = tokens.pop()?;
    let mut stars = name_raw.matches('*').count() + type_raw_stars(&tokens.join(" "));
    let name = sanitize_ident(&name_raw.replace('*', ""));
    let base_raw: String = tokens
        .iter()
        .find(|t| !t.trim_matches('*').is_empty())
        .map(|t| t.trim_matches('*').to_string())
        .unwrap_or_default();
    if array.is_some() {
        // A `T x[N]` local decays to a `T*` — the pointee keeps exact width.
        stars += 1;
    }
    let base = if base_raw == "code" {
        "void*".to_string()
    } else {
        match gh_type(&base_raw, stars > 0) {
            Some(b) => b.to_string(),
            None => base_raw.clone(),
        }
    };

    let mut init_walk = WalkOut::default();
    let init_txt = match init {
        Some(expr) => {
            walk_line(&expr, &mut init_walk);
            if init_walk.uses_stack {
                *uses_stack = true;
                *stack_max = (*stack_max).max(init_walk.stack_max);
            }
            format!(" = {}", init_walk.text.trim())
        }
        None => String::new(),
    };

    let is_scalar_ptr = stars > 0 && pointee_is_scalar(&base);
    let _ = is_scalar_ptr;
    if let Some(dim) = array {
        // `undefined1 auStack_8 [4];` → `byte* auStack_8 = stackalloc byte[4];`
        Some(DeclParts {
            decl: format!(
                "{base}{} {name} = stackalloc {base}[{dim}];{init_txt}",
                "*".repeat(stars)
            ),
            name,
            info: Some((base, true)),
        })
    } else if stars > 0 {
        Some(DeclParts {
            decl: format!("{base}{} {name}{init_txt};", "*".repeat(stars)),
            name,
            info: Some((base, true)),
        })
    } else {
        // C# definite assignment: give scalar locals an initial value.
        let decl = if init_txt.is_empty() {
            let zero = match base.as_str() {
                "bool" => " = false".to_string(),
                "float" | "double" | "long" => " = 0".to_string(),
                _ => String::new(),
            };
            format!("{base} {name}{zero};")
        } else {
            format!("{base} {name}{init_txt};")
        };
        Some(DeclParts {
            decl,
            name,
            info: Some((base, false)),
        })
    }
}

fn pointee_is_scalar(base: &str) -> bool {
    matches!(
        base,
        "int"
            | "uint"
            | "short"
            | "ushort"
            | "byte"
            | "sbyte"
            | "long"
            | "ulong"
            | "float"
            | "double"
    )
}

fn type_raw_stars(type_raw: &str) -> usize {
    type_raw.matches('*').count()
}

/// Count the top-level arguments of the first `name(...)` call on a line.
fn count_call_args(line: &str, name: &str) -> Option<usize> {
    let needle = format!("{name}(");
    let start = line.find(&needle)? + needle.len();
    let mut depth = 1usize;
    let mut args = 1usize;
    let mut has_arg = false;
    for c in line[start..].chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(if has_arg { args } else { 0 });
                }
            }
            ',' if depth == 1 => args += 1,
            c if !c.is_whitespace() => has_arg = true,
            _ => {}
        }
    }
    None
}

/// Emitted stub for an unconverted function: signature (if parseable) + the
/// original C, line-commented.
fn emit_stub(
    f: &FnSrc,
    symbol: Option<&String>,
    call_args: &BTreeMap<String, usize>,
    result_used: &BTreeSet<String>,
) -> String {
    let mut out = String::new();
    let sym = symbol
        .map(|s| format!(" — export `{s}`"))
        .unwrap_or_default();
    let _ = writeln!(
        out,
        "    // {} @ 0x{} not auto-converted{}",
        f.name, f.addr, sym
    );
    let mut sig = convert_sig(&f.sig);
    if let Some(sig) = sig.as_mut() {
        // Thunk widening, same as converted functions.
        if let Some(max_args) = call_args.get(&f.name) {
            if *max_args > sig.params.len() {
                sig.params
                    .push(("params long[]".to_string(), "a_extra".to_string()));
            }
        }
        if sig.ret == "void" && result_used.contains(&f.name) {
            sig.ret = "long".to_string();
        }
    }
    if let Some(sig) = &sig {
        let params = sig
            .params
            .iter()
            .map(|(t, n)| format!("{t} {n}"))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            out,
            "    public static unsafe {} {}({}) {{",
            sig.ret, f.name, params
        );
    } else {
        let _ = writeln!(out, "    // original signature: {}", f.sig);
    }
    out.push_str("    /* decompiled C:\n");
    for line in &f.body {
        // Neutralize `*/`: inside a block comment it would close it early
        // even on `//`-prefixed lines.
        let safe = line.replace("*/", "* /");
        let _ = writeln!(out, "    // {safe}");
    }
    out.push_str("    // end C — the .c file next to the decomp root is authoritative.\n");
    out.push_str("    */\n");
    if sig.is_some() {
        out.push_str("        throw new NotSupportedException(\"not auto-converted\");\n");
        out.push_str("    }\n");
    }
    out
}

// ---------------------------------------------------------------------------
// C#-specific line fixes (post type-walk)
// ---------------------------------------------------------------------------

/// Ghidra emits C character literals (`'a'`, `'\0'`); C# char is 2 bytes, so
/// every literal becomes an explicit `(sbyte)` conversion.
fn rewrite_char_literals(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut out = String::with_capacity(line.len() + 16);
    let mut i = 0usize;
    let mut in_string = false;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if in_string {
            out.push(c);
            if c == '\\' && i + 1 < bytes.len() {
                out.push(bytes[i + 1] as char);
                i += 2;
                continue;
            }
            if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
            continue;
        }
        if c == '\'' {
            // Find the closing quote: one plain char, or a C escape
            // (`\\x` + hex digits, backslash + octal digits, or one char).
            let mut j = i + 1;
            if j < bytes.len() && bytes[j] == b'\\' {
                j += 1;
                if j < bytes.len() && bytes[j] == b'x' {
                    j += 1;
                    while j < bytes.len() && bytes[j].is_ascii_hexdigit() {
                        j += 1;
                    }
                } else if j < bytes.len() && bytes[j].is_ascii_digit() {
                    let start = j;
                    while j < bytes.len() && bytes[j].is_ascii_digit() && j - start < 3 {
                        j += 1;
                    }
                } else if j < bytes.len() {
                    j += 1;
                }
            } else if j < bytes.len() {
                j += 1;
            }
            if j < bytes.len() && bytes[j] == b'\'' {
                out.push_str(&format!("((sbyte){})", &line[i..=j]));
                i = j + 1;
                continue;
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Parse `uint *)` (or `uint **)`) after a leading `*(`. Returns the pointee
/// base and the remainder after the cast's `)`. `None` for multi-star casts
/// (pointer-valued, no wrap needed/possible).
fn parse_pointee_cast(rest: &str) -> Option<(&str, &str)> {
    let word_end = rest
        .char_indices()
        .find(|(_, c)| !is_ident_continue(*c))
        .map(|(i, _)| i)?;
    let word = &rest[..word_end];
    if !pointee_is_scalar(word) {
        return None;
    }
    let after = rest[word_end..].trim_start();
    let star_count = after.chars().take_while(|c| *c == '*').count();
    if star_count != 1 {
        return None;
    }
    let after = after.trim_start_matches('*');
    after.strip_prefix(')').map(|r| (word, r))
}

/// True when `expr` begins with a type cast like `(int)` / `(uint *)`.
fn starts_with_cast(expr: &str) -> bool {
    let Some(rest) = expr.strip_prefix('(') else {
        return false;
    };
    let Some(word_end) = rest
        .char_indices()
        .find(|(_, c)| !is_ident_continue(*c))
        .map(|(i, _)| i)
    else {
        return false;
    };
    let word = &rest[..word_end];
    if !is_type_word(word) && !pointee_is_scalar(word) && word != "long" {
        return false;
    }
    let after = rest[word_end..].trim_start().trim_start_matches('*');
    after.starts_with(')')
}

/// Classify an assignment target: `Deref(base)` for memory slots
/// (`*(T *)addr`, `ptr[i]` — exact-width, so plain long RHS must coerce) or
/// `Local(base)` for variables (long locals take anything implicitly).
enum AssignTarget {
    Deref(String),
    Local(String),
}

/// Rewrite assignment statements so the RHS explicitly converts to the LHS
/// type (C# has no implicit long→pointee / pointer→long conversions).
/// Skips lines it does not recognize.
fn fix_assign(
    line: &str,
    var_types: &BTreeMap<String, String>,
    ptr_pointee: &BTreeMap<String, String>,
    local_types: &BTreeMap<String, String>,
    labels_ptr: &BTreeMap<String, bool>,
) -> String {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];

    let Some((lhs, rhs, target)) =
        classify_lhs(trimmed, var_types, ptr_pointee, local_types, labels_ptr)
    else {
        return line.to_string();
    };
    if rhs.is_empty() {
        return line.to_string();
    }
    let Some(coerced) = coerce_rhs(&rhs, &target, labels_ptr) else {
        return line.to_string();
    };
    if coerced == rhs {
        return line.to_string();
    }
    format!("{indent}{lhs} = {coerced};")
}

/// Determine the assignment target for the statement at `trimmed`,
/// returning `(lhs text, rhs text, target)`.
fn classify_lhs(
    trimmed: &str,
    var_types: &BTreeMap<String, String>,
    ptr_pointee: &BTreeMap<String, String>,
    local_types: &BTreeMap<String, String>,
    labels_ptr: &BTreeMap<String, bool>,
) -> Option<(String, String, AssignTarget)> {
    // Pattern A1: `*(T *)expr = ...`
    if let Some(rest) = trimmed.strip_prefix("*(") {
        let (base, after) = parse_pointee_cast(rest)?;
        let eq = find_toplevel_assign(after)?;
        let rhs = rhs_of(after, eq)?;
        return Some((
            format!("*({base} *){}", after[..eq].trim_end()),
            rhs,
            AssignTarget::Deref(base.to_string()),
        ));
    }
    // Pattern A2: `*ptr = ...` with a known scalar pointer or pointer label.
    if let Some(rest) = trimmed.strip_prefix('*') {
        let end = rest
            .char_indices()
            .find(|(_, c)| !is_ident_continue(*c))
            .map(|(i, _)| i)?;
        let name = &rest[..end];
        let base = var_types.get(name).cloned().or_else(|| {
            if labels_ptr.get(name) == Some(&true) {
                Some("byte".to_string())
            } else {
                None
            }
        });
        if let Some(base) = base {
            let eq = find_toplevel_assign(rest)?;
            let rhs = rhs_of(rest, eq)?;
            return Some((format!("*{name}"), rhs, AssignTarget::Deref(base)));
        }
        return None;
    }
    // Pattern B / C: `IDENT[...] = ...` or `IDENT = ...`
    let end = trimmed
        .char_indices()
        .find(|(_, c)| !is_ident_continue(*c))
        .map(|(i, _)| i)?;
    let name = &trimmed[..end];
    if trimmed[end..].starts_with('[') {
        let base = var_types.get(name).or_else(|| ptr_pointee.get(name))?;
        let eq = find_toplevel_assign(trimmed)?;
        let rhs = rhs_of(trimmed, eq)?;
        return Some((
            trimmed[..eq].trim_end().to_string(),
            rhs,
            AssignTarget::Deref(base.clone()),
        ));
    }
    if trimmed[end..].starts_with(" = ") {
        let eq = find_toplevel_assign(trimmed)?;
        let rhs = rhs_of(trimmed, eq)?;
        if let Some(base) = ptr_pointee.get(name) {
            return Some((
                trimmed[..eq].trim_end().to_string(),
                rhs,
                AssignTarget::Local(format!("{base}*")),
            ));
        }
        let ty = local_types
            .get(name)
            .cloned()
            .unwrap_or_else(|| "long".to_string());
        return Some((
            trimmed[..eq].trim_end().to_string(),
            rhs,
            AssignTarget::Local(ty),
        ));
    }
    None
}

/// RHS text after the `= ` at `eq` (must end with `;`).
fn rhs_of(text: &str, eq: usize) -> Option<String> {
    let raw = text[eq + 2..].trim();
    if !raw.ends_with(';') {
        return None;
    }
    let rhs = raw[..raw.len() - 1].trim();
    if rhs.is_empty() {
        None
    } else {
        Some(rhs.to_string())
    }
}

fn is_int_literal(rhs: &str) -> bool {
    let r = rhs.trim_start_matches('-');
    !r.is_empty()
        && (r.starts_with("0x") || r.chars().next().is_some_and(|c| c.is_ascii_digit()))
        && r.chars()
            .all(|c| c.is_ascii_hexdigit() || c == 'x' || c == 'X')
}

/// Coerce `rhs` to the target type. `None` = leave the statement unchanged.
fn coerce_rhs(
    rhs: &str,
    target: &AssignTarget,
    labels_ptr: &BTreeMap<String, bool>,
) -> Option<String> {
    match target {
        AssignTarget::Deref(base) => {
            // Memory slots are exact-width: wrap unless provably same-type.
            let same = if let Some(rest) = rhs.strip_prefix("*(") {
                parse_pointee_cast(rest).is_some_and(|(b2, _)| b2 == base)
            } else if let Some(word) = leading_scalar_cast(rhs) {
                word == *base
            } else {
                false
            };
            if same {
                None
            } else if is_int_literal(rhs) {
                Some(format!("unchecked(({base})({rhs}))"))
            } else {
                Some(format!("({base})({rhs})"))
            }
        }
        AssignTarget::Local(ty) => {
            if ty.ends_with('*') {
                // Pointer local: label reads and bare exprs need explicit casts.
                let bare: String = rhs.chars().take_while(|c| is_ident_continue(*c)).collect();
                if labels_ptr.get(&bare) == Some(&true) {
                    return Some(format!("({ty})({rhs})"));
                }
                if starts_with_cast(rhs) {
                    return None;
                }
                if rhs
                    .split(|c: char| !is_ident_continue(c))
                    .any(|tok| labels_ptr.get(tok) == Some(&true))
                {
                    return Some(format!("({ty})({rhs})"));
                }
                return None;
            }
            // Scalar local: only pointer-label reads and mismatched casts.
            if ty == "bool" || ty == "float" || ty == "double" {
                return None;
            }
            if let Some(word) = leading_scalar_cast(rhs) {
                if word != *ty {
                    return Some(format!("({ty})({rhs})"));
                }
                return None;
            }
            if starts_with_cast(rhs) || rhs.starts_with('*') {
                return None;
            }
            let contains_ptr_label = rhs
                .split(|c: char| !is_ident_continue(c))
                .any(|tok| labels_ptr.get(tok) == Some(&true));
            if contains_ptr_label {
                return Some(format!("({ty})({rhs})"));
            }
            None
        }
    }
}

/// `(int)`-style leading cast whose target is a scalar type.
fn leading_scalar_cast(expr: &str) -> Option<String> {
    let rest = expr.strip_prefix('(')?;
    let word_end = rest
        .char_indices()
        .find(|(_, c)| !is_ident_continue(*c))
        .map(|(i, _)| i)?;
    let word = &rest[..word_end];
    if pointee_is_scalar(word) {
        let after = rest[word_end..].trim_start().trim_start_matches('*');
        if after.starts_with(')') {
            return Some(word.to_string());
        }
    }
    None
}

/// Rewrite C comma-expressions in `if`/`while` condition headers:
/// `if (a, b) {` → `a;` + `if (b) {`; `while (a, b) {` → `while (true) {` +
/// `a;` + `if (!(b)) break;`. Nested `(assign, expr)` groups are hoisted
/// the same way. Returns `(hoisted statements, rewritten header)`.
fn split_condition_commas(line: &str) -> Option<(Vec<String>, String)> {
    let trimmed = line.trim_start();
    let indent = &line[..line.len() - trimmed.len()];
    let kw = if trimmed.starts_with("if (") {
        "if"
    } else if trimmed.starts_with("while (") {
        "while"
    } else {
        return None;
    };
    if !trimmed.ends_with('{') {
        return None;
    }
    let open = trimmed.find('(')?;
    let close = trimmed.rfind(')')?;
    if close < open {
        return None;
    }
    let mut cond = trimmed[open + 1..close].to_string();

    // Hoist innermost `(assign, expr)` groups until none remain.
    let mut hoisted: Vec<String> = Vec::new();
    while let Some((assign, tail, start, end)) = find_innermost_comma_group(&cond) {
        let looks_like_stmt = assign.contains('=')
            || assign.ends_with(')')
            || assign.starts_with('*')
            || assign.starts_with('!')
            || assign.contains("++")
            || assign.contains("--");
        if !looks_like_stmt {
            return None;
        }
        hoisted.push(assign);
        cond = format!("{}{}{}", &cond[..start], tail, &cond[end..]);
    }

    // Split remaining top-level commas.
    let parts: Vec<String> = split_toplevel(&cond, ',')
        .into_iter()
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect();
    let mut stmts = hoisted;
    if parts.len() > 1 {
        let (head, last) = parts.split_at(parts.len() - 1);
        stmts.extend(head.iter().cloned());
        let last = last[0].clone();
        if kw == "if" {
            return Some((stmts, format!("{indent}if ({last}) {{")));
        }
        // while (a, b) → while (true) { a; if (!(b)) break; <body> }
        let mut out_stmts = vec![format!("{indent}while (true) {{")];
        for st in &stmts {
            out_stmts.push(st.clone());
        }
        out_stmts.push(format!("{indent}    if (!({last})) break;"));
        return Some((out_stmts, String::new()));
    }
    if stmts.is_empty() || !parts.is_empty() && parts[0].is_empty() {
        return None;
    }
    if stmts.is_empty() {
        return None;
    }
    if parts.len() == 1 {
        // Only nested groups were hoisted; keep the single condition.
        if kw == "if" {
            return Some((stmts, format!("{indent}if ({}) {{", parts[0])));
        }
        let mut out_stmts = vec![format!("{indent}while (true) {{")];
        for st in &stmts {
            out_stmts.push(st.clone());
        }
        out_stmts.push(format!("{indent}    if (!({})) break;", parts[0]));
        return Some((out_stmts, String::new()));
    }
    None
}

/// Find the innermost `(...)` group containing a top-level comma. Returns
/// `(before-comma text, after-comma text, group start, group end)` — the
/// group is replaced by `after` (the comma's value position).
fn find_innermost_comma_group(cond: &str) -> Option<(String, String, usize, usize)> {
    let bytes = cond.as_bytes();
    // Scan for innermost groups: rightmost '(' whose match has no '(' inside.
    // Function-call parens (an identifier right before '(') never qualify.
    let mut opens: Vec<usize> = Vec::new();
    let mut best: Option<(usize, usize)> = None;
    for (i, b) in bytes.iter().enumerate() {
        match b {
            b'(' => opens.push(i),
            b')' => {
                if let Some(start) = opens.pop() {
                    let is_call =
                        start > 0 && cond[..start].chars().last().is_some_and(is_ident_continue);
                    let inner = &cond[start + 1..i];
                    if !is_call && !inner.contains('(') && has_toplevel_comma(inner) {
                        best = Some((start, i + 1));
                    }
                }
            }
            _ => {}
        }
    }
    let (start, end) = best?;
    let inner = &cond[start + 1..end - 1];
    let eq = split_toplevel(inner, ',');
    let assign = eq[0].trim().to_string();
    let tail = eq[1..].join(",");
    Some((assign, tail, start, end))
}

fn has_toplevel_comma(text: &str) -> bool {
    let mut depth = 0i32;
    for b in text.bytes() {
        match b {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b',' if depth == 0 => return true,
            _ => {}
        }
    }
    false
}

fn split_toplevel(text: &str, sep: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in text.chars() {
        match c {
            '(' | '[' => {
                depth += 1;
                cur.push(c);
            }
            ')' | ']' => {
                depth -= 1;
                cur.push(c);
            }
            c if c == sep && depth == 0 => {
                parts.push(cur.clone());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    parts.push(cur);
    parts
}

/// Index of the first ` = ` at zero bracket/paren depth (position of `=`).
fn find_toplevel_assign(text: &str) -> Option<usize> {
    let mut depth = 0i32;
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b'=' if depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b' ' => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

/// Wrap pointer-typed arguments of FUN_ calls in `(long)( … )` so they bind
/// to the widened stub/parameter types (C# has no pointer→long implicit).
fn fix_call_args(
    line: &str,
    ptr_pointee: &BTreeMap<String, String>,
    labels_ptr: &BTreeMap<String, bool>,
) -> String {
    let bytes = line.as_bytes();
    let mut out = String::with_capacity(line.len() + 16);
    let mut i = 0usize;
    let mut in_string = false;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if in_string {
            out.push(c);
            if c == '\\' && i + 1 < bytes.len() {
                out.push(bytes[i + 1] as char);
                i += 2;
                continue;
            }
            if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
            continue;
        }
        let is_callee = c.is_ascii_alphabetic() || c == '_';
        if is_callee {
            let name_end = line[i..]
                .char_indices()
                .find(|(_, ch)| !is_ident_continue(*ch))
                .map(|(k, _)| k)
                .unwrap_or(line.len() - i);
            let name = &line[i..i + name_end];
            let is_synth = !is_fun_ref(name)
                && !name.starts_with("CONCAT")
                && name
                    .chars()
                    .next()
                    .is_some_and(|ch| ch.is_ascii_uppercase())
                && !is_type_word(name);
            if name_end > 0
                && bytes.get(i + name_end) == Some(&b'(')
                && (is_fun_ref(name) || is_synth)
            {
                if let Some((args_str, end_idx)) = extract_call_args(&line[i + name_end..]) {
                    let wrapped = wrap_ptr_args(args_str, ptr_pointee, labels_ptr);
                    if is_synth {
                        out.push_str("Gh.");
                    }
                    out.push_str(name);
                    out.push('(');
                    out.push_str(&wrapped);
                    out.push(')');
                    i += name_end + 1 + end_idx;
                    continue;
                }
            }
            out.push_str(name);
            i += name_end;
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

/// Given text starting at `(`, return the top-level arg list and the offset
/// of the matching `)`.
fn extract_call_args(text: &str) -> Option<(&str, usize)> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    for (idx, b) in bytes.iter().enumerate() {
        match b {
            b'(' => depth += 1,
            b')' => {
                depth -= 1;
                if depth == 0 {
                    return Some((&text[1..idx], idx));
                }
            }
            _ => {}
        }
    }
    None
}

fn wrap_ptr_args(
    args: &str,
    ptr_pointee: &BTreeMap<String, String>,
    labels_ptr: &BTreeMap<String, bool>,
) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in args.chars() {
        match c {
            '(' | '[' => {
                depth += 1;
                cur.push(c);
            }
            ')' | ']' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => {
                parts.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        parts.push(cur.trim().to_string());
    }
    parts
        .into_iter()
        .map(|a| {
            let bare: String = a.chars().take_while(|ch| is_ident_continue(*ch)).collect();
            if ptr_pointee.contains_key(&bare)
                || labels_ptr.get(&bare) == Some(&true)
                || a.starts_with('&')
            {
                format!("(long)({a})")
            } else {
                a
            }
        })
        .collect::<Vec<_>>()
        .join(",")
}

// ---------------------------------------------------------------------------
// File emission
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn write_module_files(
    module: &str,
    out_dir: &Path,
    labels: &BTreeMap<String, bool>,
    concats: &BTreeSet<String>,
    externals: &BTreeMap<String, usize>,
    converted: &[EmittedFn],
    stubs: &[StubFn],
    stats: &ModuleStats,
    synthetics: &BTreeMap<String, usize>,
) -> io::Result<()> {
    fs::create_dir_all(out_dir)?;
    let ns = format!("XyDecomp.Code.{module}");

    // Runtime.cs — namespace + CONCAT helpers.
    let mut rt = String::new();
    let _ = writeln!(
        rt,
        "// Generated by `xy curate` (C→C# pipeline). Deterministic output."
    );
    let _ = writeln!(rt, "using System;");
    let _ = writeln!(rt, "namespace {ns};\n");
    let _ = writeln!(
        rt,
        "/// Ghidra decompiler idioms reconstructed as C# helpers."
    );
    let _ = writeln!(rt, "public static class Gh");
    let _ = writeln!(rt, "{{");
    for c in concats {
        let (m, n) = (&c[6..7], &c[7..8]);
        let shift: u32 = n.parse().unwrap_or(4) * 8;
        let _ = writeln!(
            rt,
            "    public static long {c}(long a, long b) => (a << {shift}) | b; // {m}+{n} bytes"
        );
    }
    for name in synthetics.keys() {
        let _ = writeln!(
            rt,
            "    public static unsafe long {name}(params long[] a) => throw new NotSupportedException(\"Ghidra synthetic {name}\");"
        );
    }
    let _ = writeln!(rt, "}}\n");
    fs::write(out_dir.join("Runtime.cs"), rt)?;

    // Labels.g.cs — module-level data labels.
    if !labels.is_empty() {
        let mut lb = String::new();
        let _ = writeln!(
            lb,
            "// Generated by `xy curate`. Data labels referenced by this module."
        );
        let _ = writeln!(lb, "using System;");
        let _ = writeln!(lb, "namespace {ns};\n");
        let _ = writeln!(lb, "public static unsafe partial class {module}");
        let _ = writeln!(lb, "{{");
        for (name, pointer) in labels {
            let Some((_kind, addr)) = label_of(name) else {
                continue;
            };
            let hex_addr = addr.bytes().all(|b| b.is_ascii_hexdigit()) && !addr.is_empty();
            match (hex_addr, pointer) {
                (true, true) => {
                    let _ = writeln!(
                        lb,
                        "    public static unsafe readonly byte* {name} = (byte*)0x{addr}UL; // data @ 0x{addr}"
                    );
                }
                (true, false) => {
                    let _ = writeln!(
                        lb,
                        "    public static unsafe long {name}; // value placeholder for data @ 0x{addr}"
                    );
                }
                (false, _) => {
                    // String labels (s_/u_): the decompiled C carries no address.
                    let _ = writeln!(
                        lb,
                        "    public static unsafe readonly byte* {name}; // string data (address not in decompiled C)"
                    );
                }
            }
        }
        let _ = writeln!(lb, "}}\n");
        fs::write(out_dir.join("Labels.g.cs"), lb)?;
    }

    // Functions_NNN.cs batches.
    const BATCH: usize = 200;
    for (bi, chunk) in converted.chunks(BATCH).enumerate() {
        let mut cs = String::new();
        let _ = writeln!(
            cs,
            "// Generated by `xy curate` (C→C# pipeline). Deterministic output."
        );
        let _ = writeln!(cs, "using System;");
        let _ = writeln!(cs, "namespace {ns};\n");
        let _ = writeln!(cs, "public static unsafe partial class {module}");
        let _ = writeln!(cs, "{{");
        for f in chunk {
            if !f.comment.is_empty() {
                let _ = writeln!(cs, "    // {} @ 0x{} — {}", f.name, f.addr, f.comment);
            }
            cs.push_str(&f.code);
            cs.push('\n');
        }
        let _ = writeln!(cs, "}}");
        fs::write(out_dir.join(format!("Functions_{bi:03}.cs")), cs)?;
    }

    // Unconverted.cs.
    if !stubs.is_empty() {
        let mut cs = String::new();
        let _ = writeln!(
            cs,
            "// Generated by `xy curate`. Functions that did not auto-convert."
        );
        let _ = writeln!(
            cs,
            "// Each keeps its decompiled C as a comment; the .c file is authoritative."
        );
        let _ = writeln!(cs, "using System;");
        let _ = writeln!(cs, "namespace {ns};\n");
        let _ = writeln!(cs, "public static unsafe partial class {module}");
        let _ = writeln!(cs, "{{");
        for f in stubs {
            let _ = writeln!(cs, "    // Reason: {}", f.reason);
            cs.push_str(&f.code);
            cs.push('\n');
        }
        let _ = writeln!(cs, "}}");
        fs::write(out_dir.join("Unconverted.cs"), cs)?;
    }

    // Imports.g.cs — throwing stubs for FUN_ references outside this module.
    if !externals.is_empty() {
        let mut cs = String::new();
        let _ = writeln!(
            cs,
            "// Generated by `xy curate`. Functions referenced from outside this module."
        );
        let _ = writeln!(
            cs,
            "// Long-typed parameters keep intra-module call sites shape-compatible."
        );
        let _ = writeln!(cs, "using System;");
        let _ = writeln!(cs, "namespace {ns};\n");
        let _ = writeln!(cs, "public static unsafe partial class {module}");
        let _ = writeln!(cs, "{{");
        for (name, args) in externals {
            let params = (0..*args)
                .map(|i| format!("long a{i}"))
                .collect::<Vec<_>>()
                .join(", ");
            let _ = writeln!(
                cs,
                "    public static unsafe long {name}({params}) => throw new NotSupportedException(\"external reference {name}\");"
            );
        }
        let _ = writeln!(cs, "}}");
        fs::write(out_dir.join("Imports.g.cs"), cs)?;
    }

    // README.md — stats + method.
    let mut rd = String::new();
    let _ = writeln!(rd, "# C# reconstruction — {module}\n");
    let _ = writeln!(
        rd,
        "Mechanical C→C# conversion of the Ghidra decompilation. \
         Converted {}/{} functions; {} stubbed; {} external stubs.\n",
        stats.converted, stats.total, stats.unconverted, stats.external_stubs
    );
    let _ = writeln!(
        rd,
        "Integer values widen to `long` for mechanical type-checkability \
         (original widths live in the corresponding `.c` files); pointer types \
         keep exact widths. Data labels are placeholders — reads return stale \
         zeros: this is a *readable reconstruction*, not a runnable build."
    );
    if !stats.reasons.is_empty() {
        rd.push_str("\nStub reasons:\n");
        for (reason, n) in &stats.reasons {
            let _ = writeln!(rd, "- {reason}: {n}");
        }
    }
    fs::write(out_dir.join("README.md"), rd)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"// FUN_0010310c @ 0010310c

void FUN_0010310c(int *param_1,int param_2,int param_3)

{
  undefined1 uVar1;
  undefined4 uVar6;
  int *piVar7;
  undefined1 auStack_e014 [4];

  FUN_001002cc(param_2);
  *(undefined1 *)(param_1 + 0x2cc) = 9;
  *(undefined1 *)((int)param_1 + 0xb31) = 0;
  param_1[0x2d0] = 0xffff;
  uVar6 = CONCAT44(0x5,0x100000);
  local_out(&stack0xfffffffffffffff8,8);
  FUN_00abcdef(uVar6,3);
  do {
  } while (true);
}
"#;

    #[test]
    fn parses_and_converts_sample() {
        let f = parse_fn(SAMPLE).expect("parse");
        assert_eq!(f.name, "FUN_0010310c");
        assert_eq!(f.addr, "0010310c");
        assert!(f.locals.iter().any(|l| l.contains("auStack_e014")));
        assert!(f.body.iter().any(|l| l.contains("CONCAT44")));

        let ctx = FnCtx {
            call_args: BTreeMap::new(),
            result_used: BTreeSet::new(),
            labels_ptr: BTreeMap::new(),
        };
        let code = convert_fn(&f, &ctx).expect("converts");
        assert!(code.contains(
            "public static unsafe void FUN_0010310c(int* param_1, long param_2, long param_3)"
        ));
        assert!(code.contains("long uVar1 = 0;"));
        assert!(code.contains("long uVar6 = 0;"));
        assert!(code.contains("int* piVar7;"));
        assert!(code.contains("byte* auStack_e014 = stackalloc byte[4];"));
        assert!(code.contains("*(byte *)(param_1 + 0x2cc) = unchecked((byte)(9));"));
        assert!(code.contains("(long)param_1"));
        assert!(code.contains("Gh.CONCAT44(0x5,0x100000)"));
        assert!(code.contains("(__stack + (-8))"));
        assert!(code.contains("byte* __stack = stackalloc byte[16];"));
        assert!(code.contains("FUN_00abcdef(uVar6,3);"));
    }

    #[test]
    fn code_pointer_marks_unconverted() {
        let src = "// FUN_00100188 @ 00100188\n\nvoid FUN_00100188(void)\n\n{\n  (*DAT_0010018c)();\n  return;\n}\n";
        let f = parse_fn(src).expect("parse");
        let ctx = FnCtx {
            call_args: BTreeMap::new(),
            result_used: BTreeSet::new(),
            labels_ptr: BTreeMap::new(),
        };
        let err = convert_fn(&f, &ctx).unwrap_err();
        assert!(err.contains("function pointer") || err.contains("indirect"));
    }

    #[test]
    fn variadic_marks_unconverted() {
        let src = "// FUN_1 @ 00000001\n\nint FUN_1(int a,...)\n\n{\n  return a;\n}\n";
        let f = parse_fn(src).expect("parse");
        let ctx = FnCtx {
            call_args: BTreeMap::new(),
            result_used: BTreeSet::new(),
            labels_ptr: BTreeMap::new(),
        };
        assert_eq!(convert_fn(&f, &ctx).unwrap_err(), "variadic signature");
    }

    #[test]
    fn decl_detection() {
        assert!(is_decl_line("undefined4 uVar1;"));
        assert!(is_decl_line("short *psVar1;"));
        assert!(is_decl_line("undefined1 auStack_e014 [4];"));
        assert!(is_decl_line("int iStack_e010 = 5;"));
        assert!(is_decl_line("float fVar1;"));
        assert!(is_decl_line("bool bVar1 = false;"));
        assert!(!is_decl_line("FUN_001002cc(param_2);"));
        assert!(!is_decl_line("(undefined4)iVar;"));
        assert!(!is_decl_line("iVar5 = DAT_00103518;"));
    }

    #[test]
    fn type_exprs_convert() {
        assert_eq!(convert_type_expr("short *"), "short*");
        assert_eq!(convert_type_expr("undefined4 **"), "uint**");
        assert_eq!(convert_type_expr("float"), "float");
        assert_eq!(convert_type_expr("void"), "void");
        assert_eq!(convert_type_expr("undefined4"), "long");
        assert_eq!(convert_type_expr("*"), "long*");
    }

    #[test]
    fn label_classification() {
        assert!(label_of("DAT_0029f4a0").is_some());
        assert!(label_of("LAB_0010018c").is_some());
        assert!(label_of("s_the_string").is_some());
        assert!(label_of("uRam0029f4a0").is_some());
        assert!(label_of("uVar1").is_none());
        assert!(label_of("iStack_e010").is_none());
        assert!(label_of("DAT_not_hex").is_none());
    }

    #[test]
    fn call_args_counted() {
        assert_eq!(
            count_call_args("FUN_00abcdef(uVar6,3);", "FUN_00abcdef"),
            Some(2)
        );
        assert_eq!(count_call_args("FUN_00abcdef();", "FUN_00abcdef"), Some(0));
        assert_eq!(count_call_args("x = f(a, g(b), c);", "f"), Some(3));
    }
}
