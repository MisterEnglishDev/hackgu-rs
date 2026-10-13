//! CPU software viewer (debug aid): renders CCS stage/character files to a PPM so fixes can be verified without a GPU.
//! usage: soft out.ppm eye=x,y,z target=x,y,z [fov=45] [size=900] file1.ccs [file2.ccs ...]
//!   per-file options as prefix:  anim=nut00  frame=0  root=x,y,z,yawdeg  skip=shadow
use hackgu_ccs::*;
use std::collections::HashMap;

struct Tex{w:usize,h:usize,rgba:Vec<u8>}
struct Tri{p:[[f32;3];3],uv:[[f32;2];3],tex:Option<usize>,n:[f32;3]}

fn load(path:&str, opts:&HashMap<String,String>, tex_store:&mut Vec<Tex>, tris:&mut Vec<Tri>){
    let d=std::fs::read(path).unwrap(); let c=CcsFile::parse(&d).unwrap();
    let mut tex_ids:HashMap<u32,usize>=HashMap::new();
    for t in c.objects_of_type(objects::SECTION_TEXTURE){
        let clut=if t.payload.len()>=4{let id=i32::from_le_bytes(t.payload[0..4].try_into().unwrap()); if id>0{c.raw_objects.iter().find(|o|o.object_id==id as u32&&o.section_type==objects::SECTION_CLUT).and_then(|o|if o.payload.len()>=16{Some(&o.payload[16..])}else{None})}else{None}}else{None};
        if let Ok(dt)=DecodedTexture::from_payload_with_clut("t",&t.payload,clut){tex_ids.insert(t.object_id,tex_store.len());tex_store.push(Tex{w:dt.width,h:dt.height,rgba:dt.rgba});}
    }
    let mats=c.materials();
    let clumps=c.clumps(); let nodes=c.scene_nodes();
    let anim_name=opts.get("anim"); let frame:f32=opts.get("frame").and_then(|s|s.parse().ok()).unwrap_or(0.0);
    let anims=c.animations();
    let anim=anim_name.and_then(|n|anims.iter().find(|a|a.name.contains(n.as_str())));
    let root=opts.get("root").map(|s|{let v:Vec<f32>=s.split(',').map(|x|x.parse().unwrap()).collect();
        Mat4::from_translation([v[0],v[1],v[2]]).mul(&Mat4::from_rot_z(v[3].to_radians()))});
    let cache:Vec<HashMap<u32,Mat4>>=clumps.iter().map(|cl|cl.compute_world_matrices(&nodes)).collect();
    for m in c.objects_of_type(objects::SECTION_MODEL){
        let name=c.object_name(m.object_id).unwrap_or("?");
        let Ok(mut dm)=DecodedModel::from_payload(name,&m.payload,c.header.raw_version) else{continue};
        if dm.model_type==8||name.to_lowercase().contains("shadow"){continue;}
        if let Some(s)=opts.get("skip"){ if name.contains(s.as_str()){continue;} }
        let ci=if clumps.len()==1{Some(0)}else if clumps.len()>1{let suf=name.strip_prefix("MDL_").unwrap_or(name);clumps.iter().position(|cl|{let cn=c.object_name(cl.object_id).unwrap_or("");cn==format!("CMP_{}",suf)||cn.ends_with(suf)})}else{None};
        if let Some(ci)=ci{
            let mut wm=if let (Some(a),0)=(anim,ci){a.compute_world_matrices(&clumps[0],&nodes,&c,frame)}else{cache[ci].clone()};
            if let Some(r)=root{ if ci==0 { for v in wm.values_mut(){*v=r.mul(v);} } }
            if !wm.is_empty(){dm.apply_skeleton(&clumps[ci],&wm);}
        }
        for me in &dm.meshes{
            let tex=mats.get(&me.material_index).and_then(|mt|tex_ids.get(&mt.texture_obj_id)).copied();
            for t in me.indices.chunks(3){ if t.len()<3{continue;}
                let v=[&me.vertices[t[0] as usize],&me.vertices[t[1] as usize],&me.vertices[t[2] as usize]];
                let off:[f32;3]=opts.get("offset").map(|s|{let v:Vec<f32>=s.split(',').map(|x|x.parse().unwrap()).collect();[v[0],v[1],v[2]]}).unwrap_or([0.;3]);
                let ad=|q:[f32;3]|[q[0]+off[0],q[1]+off[1],q[2]+off[2]];
                let p=[ad(v[0].position),ad(v[1].position),ad(v[2].position)];
                let e1=[p[1][0]-p[0][0],p[1][1]-p[0][1],p[1][2]-p[0][2]];let e2=[p[2][0]-p[0][0],p[2][1]-p[0][1],p[2][2]-p[0][2]];
                let mut n=[e1[1]*e2[2]-e1[2]*e2[1],e1[2]*e2[0]-e1[0]*e2[2],e1[0]*e2[1]-e1[1]*e2[0]];let l=(n[0]*n[0]+n[1]*n[1]+n[2]*n[2]).sqrt().max(1e-9);for x in n.iter_mut(){*x/=l;}
                tris.push(Tri{p,uv:[v[0].uv,v[1].uv,v[2].uv],tex,n});
            }
        }
    }
}
fn sub(a:[f32;3],b:[f32;3])->[f32;3]{[a[0]-b[0],a[1]-b[1],a[2]-b[2]]}
fn cross(a:[f32;3],b:[f32;3])->[f32;3]{[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
fn dot(a:[f32;3],b:[f32;3])->f32{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
fn norm(a:[f32;3])->[f32;3]{let l=dot(a,a).sqrt().max(1e-9);[a[0]/l,a[1]/l,a[2]/l]}
fn main(){
    let args:Vec<String>=std::env::args().collect();
    let out=&args[1];
    let mut eye=[0.0,-500.0,200.0];let mut tgt=[0.0,0.0,0.0];let mut fov=45f32;let mut size=900usize;
    let mut tex_store=vec![];let mut tris=vec![];
    let mut pending:HashMap<String,String>=HashMap::new();
    let v3=|s:&str|{let v:Vec<f32>=s.split(',').map(|x|x.parse().unwrap()).collect();[v[0],v[1],v[2]]};
    for a in &args[2..]{
        if let Some(s)=a.strip_prefix("eye="){eye=v3(s)}
        else if let Some(s)=a.strip_prefix("target="){tgt=v3(s)}
        else if let Some(s)=a.strip_prefix("fov="){fov=s.parse().unwrap()}
        else if let Some(s)=a.strip_prefix("size="){size=s.parse().unwrap()}
        else if let Some((k,v))=a.split_once('=').filter(|(k,_)|["anim","frame","root","skip","offset"].contains(k)){pending.insert(k.to_string(),v.to_string());}
        else { load(a,&pending,&mut tex_store,&mut tris); pending.clear(); }
    }
    {let mut mn=[f32::MAX;3];let mut mx=[f32::MIN;3];for t in &tris{for q in &t.p{for i in 0..3{mn[i]=mn[i].min(q[i]);mx[i]=mx[i].max(q[i]);}}} println!("bounds {:?} {:?}",mn,mx);}
    let (w,h)=(size,size);
    let f=norm(sub(tgt,eye));let r=norm(cross(f,[0.0,0.0,1.0]));let u=cross(r,f);
    let tanf=(fov.to_radians()*0.5).tan();
    let mut col=vec![[110u8,140,190];w*h];let mut zb=vec![f32::INFINITY;w*h];
    let proj=|p:[f32;3]|->Option<(f32,f32,f32)>{let d=sub(p,eye);let z=dot(d,f);if z<1.0{return None;}
        Some((w as f32*0.5+dot(d,r)/z/tanf*w as f32*0.5, h as f32*0.5-dot(d,u)/z/tanf*h as f32*0.5, z))};
    for t in &tris{
        let Some(a)=proj(t.p[0])else{continue};let Some(b)=proj(t.p[1])else{continue};let Some(cc)=proj(t.p[2])else{continue};
        let minx=a.0.min(b.0).min(cc.0).floor().max(0.0) as usize;let maxx=(a.0.max(b.0).max(cc.0).ceil() as i64).min(w as i64-1);
        let miny=a.1.min(b.1).min(cc.1).floor().max(0.0) as usize;let maxy=(a.1.max(b.1).max(cc.1).ceil() as i64).min(h as i64-1);
        if maxx<0||maxy<0{continue;}
        let area=(b.0-a.0)*(cc.1-a.1)-(cc.0-a.0)*(b.1-a.1); if area.abs()<1e-6{continue;}
        let lit=0.55+0.45*dot(t.n,norm([0.4,-0.5,0.8])).abs();
        for y in miny..=maxy as usize{for x in minx..=maxx as usize{
            let (px,py)=(x as f32+0.5,y as f32+0.5);
            let w0=((b.0-px)*(cc.1-py)-(cc.0-px)*(b.1-py))/area;let w1=((cc.0-px)*(a.1-py)-(a.0-px)*(cc.1-py))/area;let w2=1.0-w0-w1;
            if w0<0.0||w1<0.0||w2<0.0{continue;}
            let z=w0*a.2+w1*b.2+w2*cc.2; if z>=zb[y*w+x]{continue;}
            let mut rgb=[170u8,170,170];
            if let Some(ti)=t.tex{let tx=&tex_store[ti];
                let u0=w0*t.uv[0][0]+w1*t.uv[1][0]+w2*t.uv[2][0];let v0=1.0-(w0*t.uv[0][1]+w1*t.uv[1][1]+w2*t.uv[2][1]);
                let tu=((u0-u0.floor())*tx.w as f32) as usize%tx.w;let tv=((v0-v0.floor())*tx.h as f32) as usize%tx.h;
                let i=(tv*tx.w+tu)*4; if tx.rgba[i+3]<64{continue;} rgb=[tx.rgba[i],tx.rgba[i+1],tx.rgba[i+2]];}
            zb[y*w+x]=z; col[y*w+x]=[(rgb[0] as f32*lit) as u8,(rgb[1] as f32*lit) as u8,(rgb[2] as f32*lit) as u8];
        }}
    }
    let mut o=format!("P6\n{} {}\n255\n",w,h).into_bytes();for c in col{o.extend_from_slice(&c);}
    std::fs::write(out,o).unwrap(); println!("tris {} tex {}",tris.len(),tex_store.len());
}
