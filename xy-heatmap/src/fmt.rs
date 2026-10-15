//! Shared number/text formatting helpers.

/// Python `{:,}` — thousands separators.
pub fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    let digits = s.len();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (digits - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// `_pct` — `f"{100.0 * n / d:.1f}%"`, `0.0%` on a zero denominator.
pub fn pct(n: u64, d: u64) -> String {
    if d == 0 {
        "0.0%".to_string()
    } else {
        format!("{:.1}%", 100.0 * n as f64 / d as f64)
    }
}
