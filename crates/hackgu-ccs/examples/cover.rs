//! cover file.ccs x y : list models (with world bounds via owner node) whose XY bounds contain (x, y) metres, plus decode failures
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let (x, y): (f32, f32) = (a[2].parse().unwrap(), a[3].parse().unwrap());
    let owners = c.model_owner_nodes();
    let clumps = c.clumps(); let nodes = c.scene_nodes();
    let wms: Vec<_> = clumps.iter().map(|cl| cl.compute_world_matrices(&nodes)).collect();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        match DecodedModel::from_payload(name, &m.payload, c.header.raw_version) {
            Err(e) => println!("DECODE FAIL {} {:?} len {}", name, e, m.payload.len()),
            Ok(mut dm) => {
                if dm.is_rigid() { if let Some((ci, n)) = owners.get(&m.object_id) { if let Some(mt) = wms[*ci].get(n) { dm.apply_rigid(mt); } } }
                let (mut lo, mut hi) = ([f32::MAX; 3], [f32::MIN; 3]);
                let mut nv = 0;
                for me in &dm.meshes { for v in &me.vertices { nv += 1; for k in 0..3 { lo[k] = lo[k].min(v.position[k]); hi[k] = hi[k].max(v.position[k]); } } }
                if nv > 0 && lo[0] <= x && x <= hi[0] && lo[1] <= y && y <= hi[1] {
                    println!("{:<26} ty{} draw{:#x} nv{} lo={:.1?} hi={:.1?} owner={}", name, dm.model_type, dm.draw_flags, nv, lo, hi, owners.contains_key(&m.object_id));
                }
            }
        }
    }
}
