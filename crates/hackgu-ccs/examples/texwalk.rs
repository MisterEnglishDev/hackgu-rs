//! texwalk file.ccs outdir : decode textures by walking btx headers directly (for files whose section chain is broken)
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let d = std::fs::read(&a[1]).unwrap();
    let c = CcsFile::parse(&d).unwrap();
    let mut i = 0;
    while i + 40 < d.len() {
        if u32::from_le_bytes(d[i..i + 4].try_into().unwrap()) & 0xffff == 0x0300 && &d[i + 12 + 24..i + 12 + 28] == b"btx\0" {
            let id = u32::from_le_bytes(d[i + 8..i + 12].try_into().unwrap());
            let plen = 24 + u32::from_le_bytes(d[i + 12 + 20..i + 12 + 24].try_into().unwrap()) as usize;
            let payload = &d[i + 12..(i + 12 + plen).min(d.len())];
            let name = c.object_name(id).unwrap_or("tex").to_string();
            match DecodedTexture::from_payload(&name, payload) {
                Ok(t) => { println!("{} {}x{}", name, t.width, t.height); std::fs::write(format!("{}/{}_{}x{}.rgba", a[2], name, t.width, t.height), &t.rgba).unwrap(); }
                Err(e) => println!("{} decode err {:?}", name, e),
            }
            i += 12 + plen;
        } else {
            i += 4;
        }
    }
}
