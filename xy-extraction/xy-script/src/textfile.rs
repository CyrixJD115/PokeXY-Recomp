//! Gen 6 (XY/ORAS) GameFreak text-bank decoder — port of pk3DS
//! `pk3DS.Core/TextFile.cs` (read side).
//!
//! Bank layout (all little-endian):
//!
//! ```text
//! 0x00  u16 section count (always 1)
//! 0x02  u16 line count
//! 0x04  u32 total length (bytes from 0x10 to end)
//! 0x08  u32 initial key  (always 0)
//! 0x0C  u32 section data offset (always 0x10)
//! 0x10  u32 section length (== total length; validated)
//! 0x14  line info table, one 8-byte entry per line:
//!         +0 i32 offset (relative to section data offset)
//!         +4 i16 length (in u16 code units)
//! ...   line data: XOR-obfuscated u16 units
//! ```
//!
//! Each line is XOR-obfuscated: line *i* starts with key
//! `0x7C89 + 0x2983 * i` (u16 wrapping); after each unit the key rotates
//! left by 3: `key = (key << 3) | (key >> 13)`.
//!
//! Decoded units: `0x0000` terminates a line. `0x0010` introduces a
//! variable `{ u16 count, u16 code, (count-1) × u16 args }`. Formatting
//! controls: `0xBE00` wait+scroll (`\r`), `0xBE01` wait+clear (`\c`),
//! `0xBE02` wait `{ u16 frames }`, `0xBDFF` empty-line marker
//! `{ u16 line }`. Characters `E07F/E08D/E08E/E08F` map to Unicode
//! nbsp/…/♂/♀. Output strings use pk3DS escape conventions so edited
//! text remains re-encodable by the same rules.

use xy_core::error::{Error, Result};
use xy_core::util::{u16_le, u32_le};

const KEY_BASE: u16 = 0x7C89;
const KEY_ADVANCE: u16 = 0x2983;
const KEY_TERMINATOR: u16 = 0x0000;
const KEY_VARIABLE: u16 = 0x0010;
const KEY_TEXTRETURN: u16 = 0xBE00;
const KEY_TEXTCLEAR: u16 = 0xBE01;
const KEY_TEXTWAIT: u16 = 0xBE02;
const KEY_TEXTNULL: u16 = 0xBDFF;

/// XY variable-code table (identical in ORAS/SM per pk3DS source).
const VARIABLES: &[(u16, &str)] = &[
    (0xFF00, "COLOR"),
    (0x0100, "TRNAME"),
    (0x0101, "PKNAME"),
    (0x0102, "PKNICK"),
    (0x0103, "TYPE"),
    (0x0105, "LOCATION"),
    (0x0106, "ABILITY"),
    (0x0107, "MOVE"),
    (0x0108, "ITEM1"),
    (0x0109, "ITEM2"),
    (0x010A, "sTRBAG"),
    (0x010B, "BOX"),
    (0x010D, "EVSTAT"),
    (0x0110, "OPOWER"),
    (0x0127, "RIBBON"),
    (0x0134, "MIINAME"),
    (0x013E, "WEATHER"),
    (0x0189, "TRNICK"),
    (0x018A, "1stchrTR"),
    (0x018B, "SHOUTOUT"),
    (0x018E, "BERRY"),
    (0x018F, "REMFEEL"),
    (0x0190, "REMQUAL"),
    (0x0191, "WEBSITE"),
    (0x019C, "CHOICECOS"),
    (0x01A1, "GSYNCID"),
    (0x0192, "PRVIDSAY"),
    (0x0193, "BTLTEST"),
    (0x0195, "GENLOC"),
    (0x0199, "CHOICEFOOD"),
    (0x019A, "HOTELITEM"),
    (0x019B, "TAXISTOP"),
    (0x019F, "MAISTITLE"),
    (0x1000, "ITEMPLUR0"),
    (0x1001, "ITEMPLUR1"),
    (0x1100, "GENDBR"),
    (0x1101, "NUMBRNCH"),
    (0x1302, "iCOLOR2"),
    (0x1303, "iCOLOR3"),
    (0x0200, "NUM1"),
    (0x0201, "NUM2"),
    (0x0202, "NUM3"),
    (0x0203, "NUM4"),
    (0x0204, "NUM5"),
    (0x0205, "NUM6"),
    (0x0206, "NUM7"),
    (0x0207, "NUM8"),
    (0x0208, "NUM9"),
];

pub fn variable_name(code: u16) -> Option<&'static str> {
    VARIABLES
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(_, name)| *name)
}

fn rol3(key: u16) -> u16 {
    key.rotate_left(3)
}

fn unmap_char(val: u16) -> char {
    match val {
        0xE07F => '\u{202F}', // narrow no-break space
        0xE08D => '…',
        0xE08E => '♂',
        0xE08F => '♀',
        _ => char::from_u32(val as u32).unwrap_or('\u{FFFD}'),
    }
}

#[derive(Debug)]
pub struct TextFile<'a> {
    data: &'a [u8],
    line_count: usize,
    /// Section data offset (always 0x10 in practice).
    sdo: usize,
}

impl<'a> TextFile<'a> {
    /// Parse and validate a text bank (validations mirror the pk3DS ctor).
    pub fn parse(data: &'a [u8]) -> Result<Self> {
        if data.len() < 0x14 {
            return Err(Error::truncated(0, 0x14, "textfile header"));
        }
        let sections = u16_le(data, 0)?;
        let line_count = u16_le(data, 2)? as usize;
        let total_length = u32_le(data, 4)? as usize;
        let initial_key = u32_le(data, 8)?;
        let sdo = u32_le(data, 0xC)? as usize;
        if initial_key != 0 {
            return Err(Error::invalid("textfile header", "initial key is not 0"));
        }
        if sections != 1 {
            return Err(Error::invalid(
                "textfile header",
                format!("section count {sections}, expected 1"),
            ));
        }
        if sdo != 0x10 {
            return Err(Error::invalid(
                "textfile header",
                format!("section data offset {sdo:#x}, expected 0x10"),
            ));
        }
        if sdo + total_length != data.len() {
            return Err(Error::invalid(
                "textfile header",
                format!(
                    "total length {total_length:#x} does not reach end of {:#x}-byte bank",
                    data.len()
                ),
            ));
        }
        let section_length = u32_le(data, sdo)? as usize;
        if section_length != total_length {
            return Err(Error::invalid(
                "textfile section",
                format!("section length {section_length:#x} != total {total_length:#x}"),
            ));
        }
        if sdo + 4 + line_count * 8 > data.len() {
            return Err(Error::invalid("textfile", "line info table exceeds bank"));
        }
        Ok(Self {
            data,
            line_count,
            sdo,
        })
    }

    pub fn line_count(&self) -> usize {
        self.line_count
    }

    /// Line-info entry: (byte offset relative to `sdo`, length in u16 units).
    fn line_info(&self, index: usize) -> Result<(usize, usize)> {
        let at = self.sdo + 4 + index * 8;
        let offset = u32_le(self.data, at)? as i32 as usize;
        let length = u16_le(self.data, at + 4)? as i16;
        if length < 0 {
            return Err(Error::invalid(
                "textfile line",
                format!("line {index} has negative length {length}"),
            ));
        }
        Ok((self.sdo + offset, length as usize))
    }

    /// Decrypt one line's u16 units (terminator included).
    pub fn line_units(&self, index: usize) -> Result<Vec<u16>> {
        if index >= self.line_count {
            return Err(Error::invalid("textfile line", format!("no line {index}")));
        }
        let (start, units) = self.line_info(index)?;
        let end = start + units * 2;
        let bytes = self
            .data
            .get(start..end)
            .ok_or_else(|| Error::truncated(start, units * 2, "textfile line data"))?;
        let mut key = KEY_BASE.wrapping_add(KEY_ADVANCE.wrapping_mul(index as u16));
        let mut out = Vec::with_capacity(units);
        for chunk in bytes.as_chunks::<2>().0 {
            let unit = u16::from_le_bytes([chunk[0], chunk[1]]) ^ key;
            key = rol3(key);
            out.push(unit);
        }
        Ok(out)
    }

    /// Decode one line to a pk3DS-style escaped string.
    pub fn line(&self, index: usize) -> Result<String> {
        let units = self.line_units(index)?;
        decode_units(&units)
    }

    /// Decode all lines.
    pub fn lines(&self) -> Result<Vec<String>> {
        (0..self.line_count).map(|i| self.line(i)).collect()
    }
}

/// Decode decrypted code units to the pk3DS escape-convention string.
pub fn decode_units(units: &[u16]) -> Result<String> {
    let mut s = String::new();
    let mut i = 0usize;
    while i < units.len() {
        let val = units[i];
        if val == KEY_TERMINATOR {
            break;
        }
        i += 1;
        match val {
            KEY_VARIABLE => {
                let count = *units
                    .get(i)
                    .ok_or_else(|| Error::truncated(i, 1, "textfile variable count"))?;
                let code = *units
                    .get(i + 1)
                    .ok_or_else(|| Error::truncated(i + 1, 1, "textfile variable code"))?;
                i += 2;
                match code {
                    KEY_TEXTRETURN => s.push_str("\\r"),
                    KEY_TEXTCLEAR => s.push_str("\\c"),
                    KEY_TEXTWAIT => {
                        let time = *units
                            .get(i)
                            .ok_or_else(|| Error::truncated(i, 1, "textfile wait time"))?;
                        i += 1;
                        s.push_str(&format!("[WAIT {time}]"));
                    }
                    KEY_TEXTNULL => {
                        let line = *units
                            .get(i)
                            .ok_or_else(|| Error::truncated(i, 1, "textfile null-line index"))?;
                        i += 1;
                        s.push_str(&format!("[~ {line}]"));
                    }
                    _ => {
                        let name = variable_name(code).unwrap_or("UNKNOWN");
                        s.push_str("[VAR ");
                        s.push_str(name);
                        if count > 1 {
                            s.push('(');
                            for k in 1..count as usize {
                                let arg = units.get(i + k - 1).copied().ok_or_else(|| {
                                    Error::truncated(i + k - 1, 1, "textfile variable arg")
                                })?;
                                if k > 1 {
                                    s.push(',');
                                }
                                s.push_str(&format!("{arg:04X}"));
                            }
                            i += count as usize - 1;
                            s.push(')');
                        }
                        s.push(']');
                    }
                }
            }
            0x000A => s.push_str("\\n"),
            0x005C => s.push_str("\\\\"),
            0x005B => s.push_str("\\["),
            _ => s.push(unmap_char(val)),
        }
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a one-line bank the way pk3DS's writer would (crypt applied).
    fn bank(lines: &[&str]) -> Vec<u8> {
        let encode_line = |text: &str, index: usize| -> Vec<u8> {
            let mut units: Vec<u16> = text.chars().map(|c| c as u16).collect();
            units.push(KEY_TERMINATOR);
            if units.len() % 2 == 1 {
                units.push(0); // pad to 4-byte alignment like the writer
            }
            let mut key = KEY_BASE.wrapping_add(KEY_ADVANCE.wrapping_mul(index as u16));
            let mut bytes = Vec::with_capacity(units.len() * 2);
            for unit in units {
                let enc = unit ^ key;
                key = rol3(key);
                bytes.extend_from_slice(&enc.to_le_bytes());
            }
            bytes
        };

        let mut payloads = Vec::new();
        let mut infos = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            let payload = encode_line(line, i);
            infos.push((4 + lines.len() * 8 + payloads.len(), payload.len() / 2));
            payloads.extend_from_slice(&payload);
        }
        let section_length = 4 + lines.len() * 8 + payloads.len();
        let mut data = Vec::with_capacity(0x10 + section_length);
        data.extend_from_slice(&1u16.to_le_bytes());
        data.extend_from_slice(&(lines.len() as u16).to_le_bytes());
        data.extend_from_slice(&(section_length as u32).to_le_bytes());
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(&0x10u32.to_le_bytes());
        data.extend_from_slice(&(section_length as u32).to_le_bytes());
        for (off, len) in infos {
            data.extend_from_slice(&(off as u32).to_le_bytes());
            data.extend_from_slice(&(len as u16).to_le_bytes());
            data.extend_from_slice(&0u16.to_le_bytes());
        }
        data.extend_from_slice(&payloads);
        data
    }

    #[test]
    fn round_trips_synthetic_bank() {
        // '[' would decode to the pk3DS escape "\["; plain words only here
        // (escapes and variables are covered by decode tests below).
        let data = bank(&["Bulbasaur", "Ivysaur", "Venusaur"]);
        let tf = TextFile::parse(&data).expect("parse");
        assert_eq!(tf.line_count(), 3);
        let lines = tf.lines().expect("lines");
        assert_eq!(lines[0], "Bulbasaur");
        assert_eq!(lines[1], "Ivysaur");
        assert_eq!(lines[2], "Venusaur");
    }

    #[test]
    fn decodes_variables_and_escapes() {
        // 0x0010, count=2, code=TRNAME, arg=0x0003 ; then literal 'x'; \n.
        let units = [
            KEY_VARIABLE,
            2,
            0x0100,
            0x0003,
            'x' as u16,
            0x000A,
            KEY_TERMINATOR,
        ];
        assert_eq!(decode_units(&units).unwrap(), "[VAR TRNAME(0003)]x\\n");
        let wait = [KEY_VARIABLE, 1, KEY_TEXTWAIT, 60, KEY_TERMINATOR];
        assert_eq!(decode_units(&wait).unwrap(), "[WAIT 60]");
    }

    #[test]
    fn rejects_bad_header() {
        assert!(TextFile::parse(&[0u8; 8]).is_err());
        let mut data = bank(&["hi"]);
        data[0] = 2; // sections != 1
        assert!(TextFile::parse(&data).is_err());
    }
}
