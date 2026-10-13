use hackgu_ccs::*;
fn main() {
    for f in std::env::args().skip(1) {
        let c = CcsFile::parse(&std::fs::read(&f).unwrap()).unwrap();
        let nodes = c.scene_nodes();
        println!("== {}", f);
        for cl in c.clumps() {
            let wm = cl.compute_world_matrices(&nodes);
            for (i, &oid) in cl.bone_obj_ids.iter().enumerate() {
                let Some(n) = nodes.get(&oid) else { continue };
                let Some(m) = wm.get(&oid).map(|m| m.0) else { continue };
                let t = &cl.transforms[i];
                let ident = (0..16).all(|k| (m[k] - Mat4::IDENTITY.0[k]).abs() < 1e-4);
                if !ident || n.model_obj_id != 0 {
                    println!("  {:<16} {:<20} par={:<20} mdl={:<18} T={:.2?} R={:.1?} S={:.2?}{}", c.object_name(cl.object_id).unwrap_or("?"), c.object_name(oid).unwrap_or("?"),
                        c.object_name(n.parent_obj_id).unwrap_or("-"), c.object_name(n.model_obj_id).unwrap_or("-"), t.translation, t.rotation_deg, t.scale, if ident {""} else {"  <NONIDENT>"});
                }
            }
        }
    }
}
