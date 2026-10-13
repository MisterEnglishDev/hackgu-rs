use hackgu_ccs::*;
fn main(){let a:Vec<String>=std::env::args().collect();let d=std::fs::read(&a[1]).unwrap();let c=CcsFile::parse(&d).unwrap();
 let mut cnt=std::collections::BTreeMap::new();
 for o in &c.raw_objects{*cnt.entry(o.section_type).or_insert(0)+=1;}
 println!("{:x?}",cnt);
 for o in &c.raw_objects{ if matches!(o.section_type,0x1300|0x1400|0x900|0xB00|0x700){
   let nm=c.object_name(o.object_id).unwrap_or("?");
   let f:Vec<f32>=o.payload.chunks_exact(4).take(6).map(|x|f32::from_le_bytes(x.try_into().unwrap())).collect();
   println!("{:#x} {:<28} len{} {:.1?}",o.section_type,nm,o.payload.len(),if o.section_type>=0x1300{&f[..]}else{&f[..0]});
 }}
 for (n,mn,mx) in c.clumps().iter().map(|c|(c.object_id,0,0)){let _=(n,mn,mx);}
}
