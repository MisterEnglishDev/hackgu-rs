//! blobs file.ccs [type_hex] : list sections of a type (default 0x2400 BinaryBlob) with name, size and a hex/ascii preview
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let ty = a.get(2).map(|s| u32::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()).unwrap_or(0x2400);
    for o in c.objects_of_type(ty) {
        let p = &o.payload;
        let asc: String = p.iter().take(96).map(|&b| if (32..127).contains(&b) { b as char } else { '.' }).collect();
        println!("{} len {}\n  {:02x?}\n  {}", c.object_name(o.object_id).unwrap_or("?"), p.len(), &p[..p.len().min(64)], asc);
    }
}
