use hackgu_ccs::*;
use std::collections::*;
fn main(){
    for p in std::env::args().skip(1){
        let d=std::fs::read(&p).unwrap();
        let c=match CcsFile::parse(&d){Ok(c)=>c,Err(e)=>{println!("{p}: PARSE ERR {e}");continue}};
        let mats=c.materials();
        let mut texok=HashSet::new(); let mut texbad=vec![];
        for t in c.objects_of_type(objects::SECTION_TEXTURE){
            match DecodedTexture::from_payload_with_clut("t",&t.payload,None){Ok(_)=>{texok.insert(t.object_id);},Err(e)=>texbad.push((t.object_id,c.object_name(t.object_id).unwrap_or("?").to_string(),format!("{e}")))}
        }
        let (mut meshes,mut nomat,mut notex,mut verts,mut mdl_err)=(0,0,0,0,0);
        let mut missing:BTreeMap<String,usize>=BTreeMap::new();
        for m in c.objects_of_type(objects::SECTION_MODEL){
            let nm=c.object_name(m.object_id).unwrap_or("?");
            match DecodedModel::from_payload(nm,&m.payload,c.header.raw_version){
                Ok(dm)=>for me in &dm.meshes{ meshes+=1; verts+=me.vertices.len();
                    match mats.get(&me.material_index){None=>{nomat+=1;*missing.entry(format!("nomat:{}:{}",nm,me.material_index)).or_default()+=1;},Some(mt)=>if !texok.contains(&mt.texture_obj_id){notex+=1;*missing.entry(format!("notex:{}:mat{}->tex{}",nm,me.material_index,mt.texture_obj_id)).or_default()+=1;}}}
                Err(_)=>{mdl_err+=1;println!("  model decode error {nm}")}
            }
        }
        println!("{p}\n  tex ok {} bad {} | meshes {} verts {} nomat {} notex {} model_err {}",texok.len(),texbad.len(),meshes,verts,nomat,notex,mdl_err);
        for b in texbad.iter().take(6){println!("  badtex {:?}",b)}
        for (k,v) in missing.iter().take(12){println!("  {k} x{v}")}
    }
}
