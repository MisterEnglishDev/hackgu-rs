//! countall root : "path count" of parsed sections for every .ccs under root (regression check)
fn walk(d: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for e in std::fs::read_dir(d).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() { walk(&p, out) } else if p.extension().map(|x| x == "ccs").unwrap_or(false) { out.push(p) }
    }
}
fn main() {
    let mut v = vec![];
    for r in std::env::args().skip(1) { walk(std::path::Path::new(&r), &mut v); }
    v.sort();
    for p in v {
        let Some(c) = std::fs::read(&p).ok().and_then(|d| hackgu_ccs::CcsFile::parse(&d).ok()) else { println!("{} -1", p.display()); continue };
        let (mut ok, mut bad, mut mdl_ok, mut mdl_bad) = (0, 0, 0, 0);
        for t in c.objects_of_type(hackgu_ccs::objects::SECTION_TEXTURE) {
            let clut_id = t.payload.get(0..4).map(|b| i32::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
            let clut = if clut_id > 0 { c.raw_objects.iter().find(|o| o.object_id == clut_id as u32 && o.section_type == hackgu_ccs::objects::SECTION_CLUT).and_then(|o| o.payload.get(16..)) } else { None };
            if hackgu_ccs::DecodedTexture::from_payload_with_clut("t", &t.payload, clut).is_ok() { ok += 1 } else { bad += 1 }
        }
        for m in c.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
            if hackgu_ccs::DecodedModel::from_payload("m", &m.payload, c.header.raw_version).is_ok() { mdl_ok += 1 } else { mdl_bad += 1 }
        }
        println!("{} {} tex {} {} mdl {} {}", p.display(), c.raw_objects.len(), ok, bad, mdl_ok, mdl_bad);
    }
}
