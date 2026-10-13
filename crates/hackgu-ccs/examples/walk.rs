use hackgu_ccs::*;
fn main(){
    let p=std::env::args().nth(1).unwrap();
    let d=std::fs::read(&p).unwrap();
    let c=CcsFile::parse(&d).unwrap();
    let last=c.raw_objects.last().unwrap();
    let endpos=last.offset+12+last.size;
    println!("file len {} last parsed obj ends at {} (id {} type {:#x}); objects {}",d.len(),endpos,last.object_id,last.section_type,c.raw_objects.len());
    let t=|o:usize|u32::from_le_bytes(d[o..o+4].try_into().unwrap());
    println!("next 48 bytes: {:02x?}",&d[endpos..(endpos+48).min(d.len())]);
    println!("type at next: {:#x} size {} id {}",t(endpos)&0xffff,t(endpos+4),t(endpos+8));
}
