//! footspeed model.ccs ANM_name : world position of the foot bones over an in-place cycle; while a foot is planted
//! it slides backward at the speed the character must move (metres per frame of the 30 fps animation).
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    let clump = c.clumps().into_iter().next().unwrap();
    let nodes = c.scene_nodes();
    let an = c.animations().into_iter().find(|x| x.name == a[2]).unwrap();
    let feet: Vec<u32> = clump.bone_obj_ids.iter().copied().filter(|&id| c.object_name(id).is_some_and(|n| n.ends_with(" foot") || n.ends_with(" head"))).collect();
    for &f in &feet { print!("{:>26}", c.object_name(f).unwrap()); }
    println!();
    for fr in 0..an.frame_count {
        let w = an.compute_world_matrices(&clump, &nodes, &c, fr as f32);
        print!("f{:3}", fr);
        for &f in &feet {
            let m = w[&f];
            let p = m.transform_point([0.0; 3]);
            print!("  y{:8.4} z{:8.4}", p[1], p[2]);
        }
        println!();
    }
}
