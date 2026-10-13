use hackgu_ccs::*;
fn main(){
    let a:Vec<String>=std::env::args().collect();
    let d=std::fs::read(&a[1]).unwrap();let c=CcsFile::parse(&d).unwrap();
    let anims=c.animations();let an=anims.iter().find(|x|x.name.contains(a[2].as_str())).unwrap();
    let cl=&c.clumps()[0];
    println!("anim {} frames {} channels {}",an.name,an.frame_count,an.channels.len());
    for ch in &an.channels{
        let idx=cl.bone_obj_ids.iter().position(|&o|o==ch.bone_id);
        let (bt,br)=idx.map(|i|(cl.transforms[i].translation,cl.transforms[i].rotation_deg)).unwrap_or(([0.;3],[0.;3]));
        println!("{:<22} id{:<4} inclump={} st={:?} tk={} sr={:?} rk={}  bindT={:.3?} bindR={:.1?}",ch.bone_name,ch.bone_id,idx.is_some(),ch.static_translation.map(|t|[(t[0]*1000.).round()/1000.,(t[1]*1000.).round()/1000.,(t[2]*1000.).round()/1000.]),ch.translation_keys.len(),ch.static_rotation,ch.rotation_keys.len(),bt,br);
        if let Some(k)=ch.rotation_keys.first(){println!("      rk0 frame {} quat {:.3?}",k.frame,k.quat)}
        if let Some(k)=ch.translation_keys.first(){println!("      tk0 frame {} t {:.3?}",k.frame,k.translation)}
    }
}
