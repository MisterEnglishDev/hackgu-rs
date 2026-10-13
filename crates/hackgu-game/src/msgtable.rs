//! Event message table decoder.
//!
//! Event CSV scripts (`ev/PMxxxxx.csv`) may reference dialogue by numeric id
//! (the `TEXT` column of ops 5/68 is a small integer) instead of inline text.
//! The text lives in `ev/msg/msg_*.ccs`, each of which is a CCS container whose
//! `BinaryBlob` objects are named `BIN_<scriptid lowercase>` (e.g. `BIN_pm10105`).
//!
//! ## Script prefix -> msg file
//! `PM`->`msg_pm.ccs`, `PS`->`msg_ps.ccs`, `PP`->`msg_pp.ccs`, `ES`->`msg_es.ccs`,
//! `EM`->`msg_em.ccs`, `EP`->`msg_ep.ccs`, `MM`->`mmtxt.ccs`, `OP`->`msg_op.ccs`,
//! `ED`->`msg_ed.ccs`, `ZZ`->`msg_zz.ccs`. If the preferred file lacks the blob,
//! all `*.ccs` files in `ev/msg` are searched.
//!
//! ## Blob layout (all little-endian u32 words)
//! ```text
//! 0x00  script id, 8 bytes ("PM10105 ")
//! 0x08  u32 (unknown)      0x0C u32 N  = rows per group (max message id + 1)
//! 0x10  u32 1, u32 39      then a grid of 4 groups * N rows, 8 words per row:
//!       row = [a, b, lenA, offA, lenB, offB, c, d]
//!       group 0: (lenA,offA)=voice prefix ("spcName_019_A"), (lenB,offB)=voice id
//!       group 1,2: unused/empty here
//!       group 3: (lenA,offA)=speaker, (lenB,offB)=message text
//! ...   string pool; offsets are absolute file offsets, Shift-JIS (cp932), no separators.
//! ```
//! The row index inside a group *is* the message id used by the CSV.
//! Text is Japanese (Shift-JIS), with inline markup such as `<color76E1FF>`,
//! `<ruby審神者|さにわ>`, `<iconTRIANGLE>`.

use std::collections::HashMap;
use std::path::Path;

use hackgu_ccs::objects::SECTION_BINARYBLOB;
use hackgu_ccs::CcsFile;

/// One decoded message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageEntry {
    pub speaker: String,
    pub text: String,
    /// Voice file id, e.g. `spcName_019_AMainScenario00228` (empty if none).
    pub voice: String,
}

const MSG_FILES: &[(&str, &str)] = &[
    ("PM", "msg_pm.ccs"),
    ("PS", "msg_ps.ccs"),
    ("PP", "msg_pp.ccs"),
    ("ES", "msg_es.ccs"),
    ("EM", "msg_em.ccs"),
    ("EP", "msg_ep.ccs"),
    ("MM", "mmtxt.ccs"),
    ("OP", "msg_op.ccs"),
    ("ED", "msg_ed.ccs"),
    ("ZZ", "msg_zz.ccs"),
];

fn rd(d: &[u8], off: usize) -> Option<u32> {
    d.get(off..off + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn decode_sjis(b: &[u8]) -> String {
    let b = match b.iter().position(|&c| c == 0) {
        Some(p) => &b[..p],
        None => b,
    };
    let (cow, _, _) = encoding_rs::SHIFT_JIS.decode(b);
    cow.into_owned()
}

fn read_blob(ev_dir: &Path, script_id: &str) -> Option<Vec<u8>> {
    let upper = script_id.to_ascii_uppercase();
    let want = format!("BIN_{}", script_id.to_ascii_lowercase());
    let msg_dir = ev_dir.join("msg");
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for (pfx, f) in MSG_FILES {
        if upper.starts_with(pfx) {
            files.push(msg_dir.join(f));
        }
    }
    if let Ok(rd) = std::fs::read_dir(&msg_dir) {
        let mut rest: Vec<_> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().map(|e| e.eq_ignore_ascii_case("ccs")).unwrap_or(false) && !files.contains(p))
            .collect();
        rest.sort();
        files.extend(rest);
    }
    for path in files {
        let Ok(data) = std::fs::read(&path) else { continue };
        let Ok(ccs) = CcsFile::parse(&data) else { continue };
        for o in &ccs.raw_objects {
            if o.section_type == SECTION_BINARYBLOB && ccs.object_name(o.object_id) == Some(want.as_str()) {
                return Some(o.payload.clone());
            }
        }
    }
    None
}

fn pool_str(d: &[u8], len: u32, off: u32) -> String {
    if len == 0 {
        return String::new();
    }
    d.get(off as usize..(off as usize).saturating_add(len as usize)).map(decode_sjis).unwrap_or_default()
}

fn parse_blob(d: &[u8]) -> HashMap<u32, MessageEntry> {
    let mut map = HashMap::new();
    let Some(n) = rd(d, 12) else { return map };
    let n = n as usize;
    if n == 0 || n > 4096 {
        return map;
    }
    let row = |group: usize, id: usize| -> Option<(u32, u32, u32, u32)> {
        let base = 16 + 4 * (2 + 8 * (group * n + id));
        Some((rd(d, base + 8)?, rd(d, base + 12)?, rd(d, base + 16)?, rd(d, base + 20)?))
    };
    for id in 0..n {
        let Some((sl, so, tl, to)) = row(3, id) else { break };
        if tl == 0 {
            continue;
        }
        let (vl, vo, il, io) = row(0, id).unwrap_or((0, 0, 0, 0));
        let voice = format!("{}{}", pool_str(d, vl, vo), pool_str(d, il, io));
        map.insert(
            id as u32,
            MessageEntry { speaker: pool_str(d, sl, so), text: pool_str(d, tl, to), voice },
        );
    }
    map
}

/// Load full (speaker, text, voice) entries keyed by message id for `script_id`
/// (e.g. `"PM10105"`). `ev_dir` is the `ev` directory containing `msg/`.
pub fn load_message_entries(ev_dir: &Path, script_id: &str) -> Option<HashMap<u32, MessageEntry>> {
    let blob = read_blob(ev_dir, script_id)?;
    let m = parse_blob(&blob);
    if m.is_empty() { None } else { Some(m) }
}

/// Load `message id -> text` (UTF-8, markup preserved) for `script_id`.
pub fn load_message_table(ev_dir: &Path, script_id: &str) -> Option<HashMap<u32, String>> {
    load_message_entries(ev_dir, script_id).map(|m| m.into_iter().map(|(k, v)| (k, v.text)).collect())
}

/// Remove inline markup: `<colorRRGGBB>` dropped, `<rubyBase|reading>` -> `Base`,
/// `<iconNAME>` -> `[NAME]`, any other `<tag>` dropped.
pub fn strip_markup(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(p) = rest.find('<') {
        out.push_str(&rest[..p]);
        let after = &rest[p + 1..];
        match after.find('>') {
            Some(e) => {
                let tag = &after[..e];
                if let Some(r) = tag.strip_prefix("ruby") {
                    out.push_str(r.split('|').next().unwrap_or(""));
                } else if let Some(i) = tag.strip_prefix("icon") {
                    out.push('[');
                    out.push_str(i);
                    out.push(']');
                }
                rest = &after[e + 1..];
            }
            None => {
                out.push_str(&rest[p..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markup() {
        assert_eq!(strip_markup("<color76E1FF>あ<ruby獣神殿|じゅうじんでん>を<iconSELECT>"), "あ獣神殿を[SELECT]");
    }

    #[test]
    fn pm10105_pm10135() {
        let ev = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/vol1/data/data/ev");
        if !ev.join("msg").exists() {
            return;
        }
        let mut out = String::new();
        for (sid, ids) in [
            ("PM10105", vec![0u32, 5, 6]),
            ("PM10135", vec![0u32, 1, 2]),
            ("PM10140", vec![0u32, 1, 2, 3, 4, 5, 6]),
        ] {
            let m = load_message_entries(&ev, sid).expect(sid);
            for id in ids {
                let e = m.get(&id).unwrap_or_else(|| panic!("{} id {}", sid, id));
                let line = format!("{} #{} [{}] {} :: {}", sid, id, e.voice, e.speaker, e.text);
                println!("{}", line);
                out.push_str(&line);
                out.push('\n');
            }
        }
        let rep = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../decomp/reports/ev_messages_PM10105_PM10135.txt");
        let _ = std::fs::write(rep, out);
        assert!(load_message_table(&ev, "PM10135").unwrap()[&0].contains("よーし"));
        assert!(load_message_table(&ev, "PM10140").unwrap()[&0].contains("フォルセト"));
    }
}

/// Every message entry of every `BIN_*` table in `ev/msg/*.ccs` (used to learn speaker names).
pub fn all_entries(ev_dir: &Path) -> Vec<MessageEntry> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(ev_dir.join("msg")) else { return out };
    for e in rd.flatten() {
        let path = e.path();
        if !path.extension().map(|x| x.eq_ignore_ascii_case("ccs")).unwrap_or(false) {
            continue;
        }
        let Ok(data) = std::fs::read(&path) else { continue };
        let Ok(ccs) = CcsFile::parse(&data) else { continue };
        for o in &ccs.raw_objects {
            if o.section_type == SECTION_BINARYBLOB && ccs.object_name(o.object_id).is_some_and(|n| n.starts_with("BIN_")) {
                out.extend(parse_blob(&o.payload).into_values());
            }
        }
    }
    out
}

/// Full-width ASCII (U+FF01..U+FF5E) and the ideographic space to plain ASCII ("Ｉｙｏｔｅｎ" -> "Iyoten").
pub fn halfwidth(s: &str) -> String {
    s.chars()
        .map(|c| match c as u32 {
            0xFF01..=0xFF5E => char::from_u32(c as u32 - 0xFEE0).unwrap_or(c),
            0x3000 => ' ',
            _ => c,
        })
        .collect()
}
