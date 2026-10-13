use hackgu_ccs::*;
fn main(){let a:Vec<String>=std::env::args().collect();
 let c=CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
 let (x,y,r):(f32,f32,f32)=(a[2].parse().unwrap(),a[3].parse().unwrap(),a[4].parse().unwrap());
 let mut zs=std::collections::BTreeMap::new();
 for h in c.hit_meshes(){ for (ri,rec) in h.records.iter().enumerate(){
   for t in rec.positions.chunks(3){ if t.len()<3{continue;}
     let cx=(t[0][0]+t[1][0]+t[2][0])/3.0; let cy=(t[0][1]+t[1][1]+t[2][1])/3.0;
     if (cx-x).abs()<r&&(cy-y).abs()<r{ let z=((t[0][2]+t[1][2]+t[2][2])/3.0/10.0).round() as i32; *zs.entry((z,ri,rec.flags)).or_insert(0)+=1; }
   }}}
 for (k,v) in zs{println!("z~{}dm rec{} flags{:08x} tris{}",k.0,k.1,k.2,v);}
}
