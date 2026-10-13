//! near_geo file.ccs x y r : per model (owner-placed), count vertices within r metres (XY) of (x,y) and their z range
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let (x, y, r): (f32, f32, f32) = (a[2].parse().unwrap(), a[3].parse().unwrap(), a[4].parse().unwrap());
    let owners = c.model_owner_nodes(); let clumps = c.clumps(); let nodes = c.scene_nodes();
    let wms: Vec<_> = clumps.iter().map(|cl| cl.compute_world_matrices(&nodes)).collect();
    let mats = c.materials();
    for m in c.objects_of_type(objects::SECTION_MODEL) {
        let name = c.object_name(m.object_id).unwrap_or("?");
        let Ok(mut dm) = DecodedModel::from_payload(name, &m.payload, c.header.raw_version) else { continue };
        if dm.is_rigid() { if let Some((ci, n)) = owners.get(&m.object_id) { if let Some(mt) = wms[*ci].get(n) { dm.apply_rigid(mt); } } }
        for me in &dm.meshes {
            let near: Vec<f32> = me.vertices.iter().filter(|v| ((v.position[0]-x).powi(2)+(v.position[1]-y).powi(2)).sqrt() < r).map(|v| v.position[2]).collect();
            if near.is_empty() { continue; }
            let lo = near.iter().cloned().fold(f32::MAX, f32::min); let hi = near.iter().cloned().fold(f32::MIN, f32::max);
            let tex = mats.get(&me.material_index).and_then(|t| c.object_name(t.texture_obj_id)).unwrap_or("-");
            println!("{:<22} mat{:<5} {:<12} n={:<5} z[{:.2},{:.2}] draw{:#x}", name, me.material_index, tex, near.len(), lo, hi, dm.draw_flags);
        }
    }
}
