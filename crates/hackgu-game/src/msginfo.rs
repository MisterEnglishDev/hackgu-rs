//! `cmn/data/message/<lang>/messageInfo.bin`: the localized message database (eng, jpn, fra, ger).
//!
//! Layout (little-endian):
//! ```text
//! 0x00 u32 version (1001)   0x04 u32 entry count   0x08 u32 8   0x0C u32 0
//! 0x10 entry table, 40 bytes each:
//!      +0x00 u32 id   (CRC-32/MPEG-2 of the message key, e.g. "MainScenario00002")
//!      +0x10 i64 string offset, relative to (entry offset + 0x10); strings are NUL-terminated UTF-8
//! ```
//! The eng and jpn files list the same ids in the same order. Event scripts reach their lines through
//! the voice/scenario key stored in `ev/msg/msg_*.ccs` (`spcName_019_A` + `MainScenario00002`).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// CRC-32/MPEG-2 (poly 0x04C11DB7, init 0xFFFFFFFF, MSB first, final NOT) used for message ids.
pub fn message_id(key: &str) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in key.as_bytes() {
        crc ^= (b as u32) << 24;
        for _ in 0..8 {
            crc = if crc & 0x8000_0000 != 0 { (crc << 1) ^ 0x04C1_1DB7 } else { crc << 1 };
        }
    }
    !crc
}

pub struct MessageInfo {
    pub by_id: HashMap<u32, String>,
}

impl MessageInfo {
    pub fn parse(d: &[u8]) -> Option<MessageInfo> {
        let rd32 = |o: usize| d.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap()));
        let count = rd32(4)? as usize;
        let mut by_id = HashMap::with_capacity(count);
        for i in 0..count {
            let e = 16 + i * 40;
            let id = rd32(e)?;
            let rel = i64::from_le_bytes(d.get(e + 16..e + 24)?.try_into().ok()?);
            let start = (e as i64 + 16 + rel) as usize;
            let s = d.get(start..)?;
            let end = s.iter().position(|&b| b == 0).unwrap_or(s.len());
            by_id.insert(id, String::from_utf8_lossy(&s[..end]).into_owned());
        }
        Some(MessageInfo { by_id })
    }

    /// Load `<root>/cmn/data/message/<lang>/messageInfo.bin`, `root` being the extracted asset dir.
    pub fn load(root: &Path, lang: &str) -> Option<MessageInfo> {
        let p: PathBuf = root.join("cmn/data/message").join(lang).join("messageInfo.bin");
        MessageInfo::parse(&std::fs::read(p).ok()?)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.by_id.get(&message_id(key)).map(|s| s.as_str())
    }
}

/// The scenario key inside a voice id (`spcName_019_AMainScenario00002` -> `MainScenario00002`).
pub fn scenario_key(voice: &str) -> Option<&str> {
    voice.find("MainScenario").map(|p| &voice[p..]).or_else(|| {
        // Other prefixes keep the same "<prefix>_<letter><Key>" shape; take what follows the last "_X".
        let p = voice.rfind('_')?;
        voice.get(p + 2..).filter(|k| !k.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_matches_known_ids() {
        assert_eq!(message_id("MainScenario00001"), 0x31EB_839E);
        assert_eq!(message_id("MainScenario00002"), 0x3CA8_A547);
    }

    #[test]
    fn english_pm10100_lines() {
        for base in [".", "..", "../.."] {
            let root = Path::new(base).join("target");
            if let Some(m) = MessageInfo::load(&root, "eng") {
                // The keys resolve in both languages and give different text (no game lines are kept here).
                let e2 = m.get("MainScenario00002").expect("eng MainScenario00002");
                let e3 = m.get("MainScenario00003").expect("eng MainScenario00003");
                assert!(!e2.is_empty() && !e3.is_empty() && e2 != e3);
                let j = MessageInfo::load(&root, "jpn").unwrap();
                assert_ne!(j.get("MainScenario00002"), Some(e2));
                return;
            }
        }
    }
}
