//! animblocks file.ccs anim_name : raw block list (tag, length, target object, first words)
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for o in c.objects_of_type(objects::SECTION_ANIME) {
        let name = c.object_name(o.object_id).unwrap_or("?");
        if name != a[2] { continue; }
        let w: Vec<u32> = o.payload.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        println!("{} frames {} words {} pre {:x?}", name, w[0], w[1], &w[2..5]);
        let mut i = 5;
        while i + 2 < w.len() {
            let (tag, len, id) = (w[i], w[i + 1] as usize, w[i + 2]);
            let body = &w[i + 3..(i + 2 + len).min(w.len())];
            let f: Vec<String> = body.iter().take(12).map(|&x| { let v = f32::from_bits(x); if v.is_finite() && v.abs() > 1e-6 && v.abs() < 1e7 && x > 0x01000000 { format!("{:.2}", v) } else { format!("{:#x}", x) } }).collect();
            println!("  tag {:#010x} len {:<4} id {:<4} {:<22} {}", tag, len, id, c.object_name(id).unwrap_or("?"), f.join(" "));
            i += 2 + len;
        }
    }
}
