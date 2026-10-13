//! CRI ACB (`@UTF` tables) cue lookup: cue index / cue name -> waveform id in the paired AWB, and whether it loops.
//!
//! Event scripts (`BGM操作,123,<slot>,<n>`) give cue indices, not AWB indices: in `bgm.acb` cue 3 "rtown0" (Mac Anu)
//! is AWB 135, cue 62 "ev06" is AWB 53. Chain used: Cue (ReferenceType 3) -> Sequence.TrackIndex[0] -> Synth with the
//! same index -> ReferenceItems (type 1, waveform row) -> Waveform.Id, LoopFlag (1 = once, 2 = loop). For bgm.acb the
//! track and synth tables line up one to one (checked: 155 rows each, synth k references waveform row k).

use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct CueWave {
    pub awb_id: u32,
    pub looping: bool,
}

#[derive(Default)]
pub struct AcbCues {
    pub by_index: HashMap<u32, CueWave>,
    pub by_name: HashMap<String, u32>,
}

#[derive(Clone, Debug)]
enum Val {
    Int(u64),
    Str(String),
    Bytes(Vec<u8>),
    None,
}

impl Val {
    fn int(&self) -> Option<u64> {
        if let Val::Int(v) = self { Some(*v) } else { None }
    }
}

type Row = HashMap<String, Val>;

fn be16(d: &[u8], o: usize) -> Option<u16> {
    Some(u16::from_be_bytes(d.get(o..o + 2)?.try_into().ok()?))
}
fn be32(d: &[u8], o: usize) -> Option<u32> {
    Some(u32::from_be_bytes(d.get(o..o + 4)?.try_into().ok()?))
}

fn read_val(d: &[u8], p: &mut usize, typ: u8, base: usize, strings: usize, data: usize) -> Option<Val> {
    let v = match typ {
        0 | 1 => {
            *p += 1;
            Val::Int(*d.get(*p - 1)? as u64)
        }
        2 | 3 => {
            *p += 2;
            Val::Int(be16(d, *p - 2)? as u64)
        }
        4 | 5 => {
            *p += 4;
            Val::Int(be32(d, *p - 4)? as u64)
        }
        6 | 7 => {
            *p += 8;
            Val::Int(u64::from_be_bytes(d.get(*p - 8..*p)?.try_into().ok()?))
        }
        8 => {
            *p += 4;
            Val::None
        }
        0xA => {
            let o = be32(d, *p)? as usize;
            *p += 4;
            let a = base + strings + o;
            let e = a + d.get(a..)?.iter().position(|&c| c == 0)?;
            Val::Str(String::from_utf8_lossy(&d[a..e]).into_owned())
        }
        0xB => {
            let (o, l) = (be32(d, *p)? as usize, be32(d, *p + 4)? as usize);
            *p += 8;
            let a = base + data + o;
            Val::Bytes(d.get(a..a + l)?.to_vec())
        }
        _ => return None,
    };
    Some(v)
}

/// Parse one `@UTF` table into rows.
fn utf(d: &[u8]) -> Option<Vec<Row>> {
    if d.get(0..4)? != b"@UTF" {
        return None;
    }
    let base = 8;
    let rows_off = be16(d, base + 2)? as usize;
    let strings = be32(d, base + 4)? as usize;
    let data = be32(d, base + 8)? as usize;
    let ncols = be16(d, base + 16)? as usize;
    let row_w = be16(d, base + 18)? as usize;
    let nrows = be32(d, base + 20)? as usize;
    let mut cols = Vec::new();
    let mut p = base + 24;
    for _ in 0..ncols {
        let flags = *d.get(p)?;
        let name_off = be32(d, p + 1)? as usize;
        p += 5;
        let a = base + strings + name_off;
        let e = a + d.get(a..)?.iter().position(|&c| c == 0)?;
        let name = String::from_utf8_lossy(&d[a..e]).into_owned();
        let (storage, typ) = (flags & 0xF0, flags & 0x0F);
        let constant = if storage == 0x30 { Some(read_val(d, &mut p, typ, base, strings, data)?) } else { None };
        cols.push((name, storage, typ, constant));
    }
    let mut rows = Vec::with_capacity(nrows);
    for r in 0..nrows {
        let mut p = base + rows_off + r * row_w;
        let mut row = Row::new();
        for (name, storage, typ, constant) in &cols {
            let v = match storage {
                0x50 => read_val(d, &mut p, *typ, base, strings, data)?,
                0x30 => constant.clone().unwrap_or(Val::None),
                _ => Val::None,
            };
            row.insert(name.clone(), v);
        }
        rows.push(row);
    }
    Some(rows)
}

fn sub(top: &Row, key: &str) -> Vec<Row> {
    match top.get(key) {
        Some(Val::Bytes(b)) => utf(b).unwrap_or_default(),
        _ => Vec::new(),
    }
}

impl AcbCues {
    pub fn parse(d: &[u8]) -> Option<Self> {
        let top = utf(d)?.into_iter().next()?;
        let cues = sub(&top, "CueTable");
        let names = sub(&top, "CueNameTable");
        let seqs = sub(&top, "SequenceTable");
        let synths = sub(&top, "SynthTable");
        let waves = sub(&top, "WaveformTable");
        let mut out = AcbCues::default();
        for n in &names {
            if let (Some(Val::Str(s)), Some(i)) = (n.get("CueName"), n.get("CueIndex").and_then(|v| v.int())) {
                out.by_name.insert(s.clone(), i as u32);
            }
        }
        for (ci, cue) in cues.iter().enumerate() {
            if cue.get("ReferenceType").and_then(|v| v.int()) != Some(3) {
                continue;
            }
            let Some(seq) = cue.get("ReferenceIndex").and_then(|v| v.int()).and_then(|i| seqs.get(i as usize)) else { continue };
            let Some(Val::Bytes(ti)) = seq.get("TrackIndex") else { continue };
            let Some(track) = be16(ti, 0) else { continue };
            let Some(Val::Bytes(items)) = synths.get(track as usize).and_then(|s| s.get("ReferenceItems")) else { continue };
            let (Some(kind), Some(wi)) = (be16(items, 0), be16(items, 2)) else { continue };
            if kind != 1 {
                continue;
            }
            let Some(w) = waves.get(wi as usize) else { continue };
            let (Some(id), lf) = (w.get("Id").and_then(|v| v.int()), w.get("LoopFlag").and_then(|v| v.int())) else { continue };
            out.by_index.insert(ci as u32, CueWave { awb_id: id as u32, looping: lf != Some(1) });
        }
        Some(out)
    }
}
