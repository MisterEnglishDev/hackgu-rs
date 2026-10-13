//! texdumpall outdir root... : decode every texture >= 128 px (file__TEX.rgba) under the roots
use hackgu_ccs::*;
fn walk(d: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for e in std::fs::read_dir(d).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() { walk(&p, out) } else if p.extension().map(|x| x == "ccs").unwrap_or(false) { out.push(p) }
    }
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut files = vec![];
    for r in &a[2..] { let p = std::path::Path::new(r); if p.is_file() { files.push(p.to_path_buf()) } else { walk(p, &mut files) } }
    let mut n = 0;
    for f in files {
        let Some(c) = std::fs::read(&f).ok().and_then(|d| CcsFile::parse(&d).ok()) else { continue };
        let stem: String = f.to_string_lossy().chars().map(|ch| if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' || ch == '@' { ch } else { '_' }).collect();
        for t in c.objects_of_type(objects::SECTION_TEXTURE) {
            let clut_id = t.payload.get(0..4).map(|b| i32::from_le_bytes(b.try_into().unwrap())).unwrap_or(0);
            let clut = if clut_id > 0 { c.raw_objects.iter().find(|o| o.object_id == clut_id as u32 && o.section_type == objects::SECTION_CLUT).and_then(|o| o.payload.get(16..)) } else { None };
            let name = c.object_name(t.object_id).unwrap_or("t");
            if let Ok(d) = DecodedTexture::from_payload_with_clut(name, &t.payload, clut) {
                if d.width.max(d.height) < 128 { continue; }
                std::fs::write(format!("{}/{}__{}_{}x{}.rgba", a[1], stem, name, d.width, d.height), &d.rgba).unwrap();
                n += 1;
            }
        }
    }
    println!("{} textures", n);
}
