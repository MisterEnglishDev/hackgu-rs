use hackgu_ccs::*;
fn main(){for p in std::env::args().skip(1){let d=std::fs::read(&p).unwrap();let c=CcsFile::parse(&d).unwrap();
 let raw=c.objects_of_type(objects::SECTION_HITMESH).len();let hm=c.hit_meshes();
 println!("{} hitmesh sections {} parsed {} tris {:?}",p.rsplit('/').next().unwrap(),raw,hm.len(),hm.iter().map(|h|h.triangle_count()).collect::<Vec<_>>());}}
