use hackgu_ccs::CcsFile;
use hackgu_world::CollisionMesh;
fn main(){
    let dir="../target/vol1/data/data/town/";
    let pts:Vec<(f32,f32)>=std::env::args().skip(1).filter_map(|a|{let v:Vec<f32>=a.split(',').filter_map(|x|x.parse().ok()).collect(); if v.len()==2{Some((v[0],v[1]))}else{None}}).collect();
    let town=CcsFile::parse(&std::fs::read(format!("{dir}town01.ccs")).unwrap()).unwrap();
    let loc=town.locators();
    let get=|n:&str|loc.iter().find(|l|l.0==n).map(|l|l.1).unwrap();
    let mut col=CollisionMesh::from_hit_meshes(&town,[0.0;3]);
    for n in 0..=8{
        let off=get(&format!("DMY_town01_{n:02}d"));
        let d=CcsFile::parse(&std::fs::read(format!("{dir}town01_{n:02}h.ccs")).unwrap()).unwrap();
        col.merge(&CollisionMesh::from_hit_meshes(&d,off));
    }
    for (x,y) in pts{
        let mut hs=vec![];
        for z in [-30.0f32,-20.0,-12.0,-5.0,-1.0,0.0,6.0,12.0,13.0,25.0]{ if let Some(h)=col.get_ground_height(x,y,z){ if !hs.iter().any(|v:&f32|(v-h).abs()<0.05){hs.push(h);} } }
        println!("({x},{y}) floors={hs:?}");
    }
    for n in ["DMY_start","DMY_gate","DMY_pc01","DMY_pc02"]{ println!("{n} {:?}",loc.iter().find(|l|l.0==n).map(|l|l.1)); }
}
