//! Reading finished curated trees back: `manifest.tsv` (provenance) and
//! `validation.tsv` (post-run verification). Used by the GUI to expose the
//! curated ↔ raw relationships instead of treating files as arbitrary.

use std::fs;
use std::io;
use std::path::Path;

/// One `manifest.tsv` row: source → curated path → action → detail → bytes.
#[derive(Debug, Clone)]
pub struct ManifestRow {
    pub source: String,
    pub path: String,
    pub action: String,
    pub detail: String,
    pub in_bytes: u64,
    pub out_bytes: u64,
    pub status: String,
}

/// One `validation.tsv` row (or the `# totals` line, with `path == "# totals"`).
#[derive(Debug, Clone)]
pub struct ValidationRow {
    pub path: String,
    pub check: String,
    pub result: String,
    pub detail: String,
}

/// Parse `manifest.tsv` (header row skipped).
pub fn read_manifest(path: &Path) -> io::Result<Vec<ManifestRow>> {
    let text = fs::read_to_string(path)?;
    Ok(text
        .lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .filter_map(parse_manifest_line)
        .collect())
}

fn parse_manifest_line(line: &str) -> Option<ManifestRow> {
    let cols: Vec<&str> = line.split('\t').collect();
    if cols.len() < 7 {
        return None;
    }
    Some(ManifestRow {
        source: cols[0].to_string(),
        path: cols[1].to_string(),
        action: cols[2].to_string(),
        detail: cols[3].to_string(),
        in_bytes: cols[4].parse().unwrap_or(0),
        out_bytes: cols[5].parse().unwrap_or(0),
        status: cols[6].to_string(),
    })
}

/// Parse `validation.tsv` (header + rows + trailing `# totals` line).
pub fn read_validation(path: &Path) -> io::Result<Vec<ValidationRow>> {
    let text = fs::read_to_string(path)?;
    Ok(text
        .lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .filter_map(|l| {
            let cols: Vec<&str> = l.split('\t').collect();
            if cols.len() < 4 {
                return None;
            }
            Some(ValidationRow {
                path: cols[0].to_string(),
                check: cols[1].to_string(),
                result: cols[2].to_string(),
                detail: cols[3].to_string(),
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_lines_parse() {
        let row = parse_manifest_line(
            "garc/a/2/1/2/000.bin\tbattle/moves/000.bin\tcopy\tMove data table (pk3DS: move)\t100\t100\tok",
        )
        .unwrap();
        assert_eq!(row.source, "garc/a/2/1/2/000.bin");
        assert_eq!(row.path, "battle/moves/000.bin");
        assert_eq!(row.in_bytes, 100);
        assert_eq!(row.status, "ok");
    }

    #[test]
    fn short_lines_rejected() {
        assert!(parse_manifest_line("only\tthree\tcols").is_none());
        assert!(parse_manifest_line("").is_none());
    }
}
