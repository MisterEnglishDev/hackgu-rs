use hackgu_ccs::*;
fn main(){let a:Vec<String>=std::env::args().collect();let d=std::fs::read(&a[1]).unwrap();let c=CcsFile::parse(&d).unwrap();
 let filt=a.get(2).cloned().unwrap_or_default();
 let nodes=c.scene_nodes();
 for o in &c.raw_objects{ if o.section_type==0x1300||o.section_type==0x1400{
   let nm=c.object_name(o.object_id).unwrap_or("?"); if !nm.contains(&filt){continue;}
   let f:Vec<f32>=o.payload.chunks_exact(4).map(|x|f32::from_le_bytes(x.try_into().unwrap())).collect();
   let u:Vec<u32>=o.payload.chunks_exact(4).map(|x|u32::from_le_bytes(x.try_into().unwrap())).collect();
   println!("{:#x} id{:<4} {:<24} len{} f={:.2?} u0={:?}",o.section_type,o.object_id,nm,o.payload.len(),f,&u[..u.len().min(2)]);
 }}
 let _=nodes;
}
