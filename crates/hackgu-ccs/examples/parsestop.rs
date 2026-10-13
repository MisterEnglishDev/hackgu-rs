use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let d = std::fs::read(&a[1]).unwrap();
    let c = CcsFile::parse(&d).unwrap();
    println!("objects in index: {}, parsed: {}", c.objects.len(), c.raw_objects.len());
    for o in c.raw_objects.iter().rev().take(3) { println!("type {:#x} id {} {:?} off {:#x} size {}", o.section_type, o.object_id, c.object_name(o.object_id), o.offset, o.size); }
    let last = c.raw_objects.last().unwrap();
    let next = last.offset + 12 + last.size;
    println!("next at {:#x}: {:02x?}", next, &d[next..(next+48).min(d.len())]);
}
