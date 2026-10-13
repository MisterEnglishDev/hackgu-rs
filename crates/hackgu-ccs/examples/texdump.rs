//! texdump file.ccs outdir [filter] -> raw RGBA dumps (name_WxH.rgba) for inspection
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let filt = a.get(3).cloned().unwrap_or_default();
    for t in c.objects_of_type(objects::SECTION_TEXTURE) {
        let name = c.object_name(t.object_id).unwrap_or("?");
        if !name.contains(&filt) { continue; }
        let clut_id = i32::from_le_bytes(t.payload[0..4].try_into().unwrap());
        let clut = if clut_id > 0 { c.raw_objects.iter().find(|o| o.object_id == clut_id as u32 && o.section_type == objects::SECTION_CLUT).map(|o| &o.payload[16..]) } else { None };
        if let Ok(d) = DecodedTexture::from_payload_with_clut(name, &t.payload, clut) {
            let amin = d.rgba.chunks(4).map(|p| p[3]).min().unwrap(); let amax = d.rgba.chunks(4).map(|p| p[3]).max().unwrap();
            println!("{} {}x{} fmt{} alpha[{},{}]", name, d.width, d.height, d.format, amin, amax);
            std::fs::write(format!("{}/{}_{}x{}.rgba", a[2], name, d.width, d.height), &d.rgba).unwrap();
        }
    }
}
