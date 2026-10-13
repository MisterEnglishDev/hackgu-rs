//! Collision triangle mesh extracted directly from stage CCS models.
//! Performs raycasts, ground height lookups, and wall sliding against authentic game floor geometry.

use glam::{Vec3, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct CollisionTriangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub normal: Vec3,
    pub is_walkable: bool,
}

impl CollisionTriangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Option<Self> {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(edge2);
        let len = normal.length();
        if len < 1e-6 {
            return None;
        }
        let normal = normal / len;
        // In CC2 (+Z is Up): upward facing surfaces have normal.z > 0.5 (angle < 60 degrees)
        let is_walkable = normal.z > 0.5;

        Some(Self {
            v0,
            v1,
            v2,
            normal,
            is_walkable,
        })
    }

    /// Check if a 2D (X, Y) point lies inside this triangle's horizontal projection,
    /// and if so, compute the exact Z height on the triangle surface.
    pub fn height_at(&self, p: Vec2) -> Option<f32> {
        let p0 = Vec2::new(self.v0.x, self.v0.y);
        let p1 = Vec2::new(self.v1.x, self.v1.y);
        let p2 = Vec2::new(self.v2.x, self.v2.y);

        let d00 = (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y);
        if d00.abs() < 1e-6 {
            return None;
        }

        let d10 = (p.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p.y - p0.y);
        let d20 = (p1.x - p0.x) * (p.y - p0.y) - (p.x - p0.x) * (p1.y - p0.y);

        let u = d10 / d00;
        let v = d20 / d00;

        if u >= -0.01 && v >= -0.01 && (u + v) <= 1.01 {
            // Point is inside triangle. Solve plane equation for Z:
            // normal.x * (x - v0.x) + normal.y * (y - v0.y) + normal.z * (z - v0.z) = 0
            if self.normal.z.abs() > 1e-5 {
                let z = self.v0.z - (self.normal.x * (p.x - self.v0.x) + self.normal.y * (p.y - self.v0.y)) / self.normal.z;
                return Some(z);
            }
        }

        None
    }
}

/// Spatial hash grid for fast raycast and collision queries against thousands of stage triangles.
#[derive(Clone, Debug)]
pub struct CollisionMesh {
    pub triangles: Vec<CollisionTriangle>,
    grid_size: f32,
    grid: std::collections::HashMap<(i32, i32), Vec<usize>>,
    /// Vertical wall segments (projected onto XY) from the game's own HitMesh data.
    pub walls: Vec<WallSegment>,
    wall_grid: std::collections::HashMap<(i32, i32), Vec<usize>>,
}

/// A vertical collision wall, projected to the XY plane, with its vertical extent.
#[derive(Clone, Copy, Debug)]
pub struct WallSegment {
    pub a: Vec2,
    pub b: Vec2,
    pub z_min: f32,
    pub z_max: f32,
}

impl CollisionMesh {
    pub fn empty() -> Self {
        Self {
            triangles: Vec::new(),
            grid_size: 4.0,
            grid: std::collections::HashMap::new(),
            walls: Vec::new(),
            wall_grid: std::collections::HashMap::new(),
        }
    }

    /// Build a collision mesh from a list of triangles.
    pub fn from_triangles(triangles: Vec<CollisionTriangle>, grid_size: f32) -> Self {
        let mut grid: std::collections::HashMap<(i32, i32), Vec<usize>> = std::collections::HashMap::new();

        for (idx, tri) in triangles.iter().enumerate() {
            let min_x = (tri.v0.x.min(tri.v1.x).min(tri.v2.x) / grid_size).floor() as i32;
            let max_x = (tri.v0.x.max(tri.v1.x).max(tri.v2.x) / grid_size).floor() as i32;
            let min_y = (tri.v0.y.min(tri.v1.y).min(tri.v2.y) / grid_size).floor() as i32;
            let max_y = (tri.v0.y.max(tri.v1.y).max(tri.v2.y) / grid_size).floor() as i32;

            for gx in min_x..=max_x {
                for gy in min_y..=max_y {
                    grid.entry((gx, gy)).or_default().push(idx);
                }
            }
        }

        Self {
            triangles,
            grid_size,
            grid,
            walls: Vec::new(),
            wall_grid: std::collections::HashMap::new(),
        }
    }

    /// Extract authentic collision triangles directly from stage CCS container models.
    pub fn from_ccs_stage(ccs: &hackgu_ccs::CcsFile) -> Self {
        let mut triangles = Vec::new();
        let all_clumps = ccs.clumps();
        let scene_nodes = ccs.scene_nodes();

        let mut clump_world_cache = std::collections::HashMap::new();
        for (i, c) in all_clumps.iter().enumerate() {
            clump_world_cache.insert(i, c.compute_world_matrices(&scene_nodes));
        }

        for model_obj in ccs.objects_of_type(hackgu_ccs::objects::SECTION_MODEL) {
            let name = ccs.object_name(model_obj.object_id).unwrap_or("model");
            // Skip non-collision / sky / visual effect models
            let is_visual_only = name.starts_with("MDL_bs")
                || name.starts_with("MDL_sil")
                || name.starts_with("MDL_sun")
                || name.starts_with("MDL_bc")
                || name.to_lowercase().contains("sky")
                || name.to_lowercase().contains("silhouette")
                || name.to_lowercase().contains("fog");

            if is_visual_only {
                continue;
            }

            if let Ok(mut model) = hackgu_ccs::DecodedModel::from_payload(name, &model_obj.payload, ccs.header.raw_version) {
                if model.model_type == 8 {
                    continue;
                }

                // Apply clump world matrix placement
                let matching_clump_idx = if all_clumps.len() == 1 {
                    Some(0)
                } else if all_clumps.len() > 1 {
                    let suffix = name.strip_prefix("MDL_").unwrap_or(name);
                    all_clumps.iter().position(|c| {
                        let cname = ccs.object_name(c.object_id).unwrap_or("");
                        cname == format!("CMP_{}", suffix) || cname.ends_with(suffix)
                    })
                } else {
                    None
                };

                if let Some(ci) = matching_clump_idx {
                    if let Some(wm) = clump_world_cache.get(&ci) {
                        if !wm.is_empty() {
                            let c = &all_clumps[ci];
                            model.apply_skeleton(c, wm);
                        }
                    }
                }

                for mesh in &model.meshes {
                    let v_slice = &mesh.vertices;
                    for tri_indices in mesh.indices.chunks_exact(3) {
                        let i0 = tri_indices[0] as usize;
                        let i1 = tri_indices[1] as usize;
                        let i2 = tri_indices[2] as usize;
                        if i0 < v_slice.len() && i1 < v_slice.len() && i2 < v_slice.len() {
                            let v0 = Vec3::from(v_slice[i0].position);
                            let v1 = Vec3::from(v_slice[i1].position);
                            let v2 = Vec3::from(v_slice[i2].position);

                            if let Some(t) = CollisionTriangle::new(v0, v1, v2) {
                                triangles.push(t);
                            }
                        }
                    }
                }
            }
        }

        Self::from_triangles(triangles, 4.0)
    }

    /// Build collision from the game's own HitMesh (0x0B00) sections.
    ///
    /// `offset_cm` is added to every stored position (district placement, centimetres); the result is in metres.
    /// Floors/slopes become walkable triangles; near-vertical triangles become wall segments.
    pub fn from_hit_meshes(ccs: &hackgu_ccs::CcsFile, offset_cm: [f32; 3]) -> Self {
        let off = Vec3::from(offset_cm) * 0.01;
        Self::from_hit_meshes_placed(ccs, |_| true, |p| p + off, 0.0)
    }

    /// Collision from the HitMeshes whose name passes `keep`, each vertex (metres) mapped through `place`;
    /// `yaw` (radians about +Z) rotates the stored normal hints to match. Used for tiled fields where one
    /// library file supplies a piece that is instanced many times.
    pub fn from_hit_meshes_placed(ccs: &hackgu_ccs::CcsFile, keep: impl Fn(&str) -> bool, place: impl Fn(Vec3) -> Vec3, yaw: f32) -> Self {
        let mut tris = Vec::new();
        let mut walls = Vec::new();
        let to_m = |p: [f32; 3]| place(Vec3::new(p[0], p[1], p[2]) * 0.01);
        let (sy, cy) = yaw.sin_cos();
        let rot = |v: Vec3| Vec3::new(cy * v.x - sy * v.y, sy * v.x + cy * v.y, v.z);
        for hm in ccs.hit_meshes() {
            if !keep(ccs.object_name(hm.object_id).unwrap_or("")) {
                continue;
            }
            for rec in &hm.records {
                for (t, n) in rec.positions.chunks_exact(3).zip(rec.normals.chunks_exact(3)) {
                    let (v0, v1, v2) = (to_m(t[0]), to_m(t[1]), to_m(t[2]));
                    let hint = rot(Vec3::from(n[0]) + Vec3::from(n[1]) + Vec3::from(n[2]));
                    let geo = (v1 - v0).cross(v2 - v0);
                    // Steepness from the triangle's own geometry; the stored normal hints are unreliable for that (the
                    // ramp of dungeon2's floor-up hall, ~14 deg, carries near-horizontal hints and became a wall).
                    let vertical = {
                        let h = if geo.length_squared() > 1e-12 { geo.normalize() } else if hint.length_squared() > 1e-6 { hint.normalize() } else { continue };
                        h.z.abs() < 0.3
                    };
                    if vertical {
                        // Project to XY: use the two vertices furthest apart horizontally.
                        let pts = [v0, v1, v2];
                        let (mut best, mut ba, mut bb) = (0.0f32, 0usize, 1usize);
                        for i in 0..3 {
                            for j in (i + 1)..3 {
                                let d = Vec2::new(pts[i].x - pts[j].x, pts[i].y - pts[j].y).length_squared();
                                if d > best {
                                    best = d;
                                    ba = i;
                                    bb = j;
                                }
                            }
                        }
                        if best > 1e-6 {
                            walls.push(WallSegment {
                                a: Vec2::new(pts[ba].x, pts[ba].y),
                                b: Vec2::new(pts[bb].x, pts[bb].y),
                                z_min: v0.z.min(v1.z).min(v2.z),
                                z_max: v0.z.max(v1.z).max(v2.z),
                            });
                        }
                    }
                    // Keep every triangle for floor lookups and camera ray tests; orient the normal with the stored hint.
                    if let Some(mut tri) = CollisionTriangle::new(v0, v1, v2) {
                        if hint.length_squared() > 1e-6 && tri.normal.dot(hint) < 0.0 {
                            tri.normal = -tri.normal;
                        }
                        tri.is_walkable = tri.normal.z > 0.5;
                        tris.push(tri);
                    }
                }
            }
        }
        let mut mesh = Self::from_triangles(tris, 4.0);
        mesh.set_walls(walls);
        mesh
    }

    fn set_walls(&mut self, walls: Vec<WallSegment>) {
        let gs = self.grid_size;
        let mut grid: std::collections::HashMap<(i32, i32), Vec<usize>> = std::collections::HashMap::new();
        for (i, w) in walls.iter().enumerate() {
            let (x0, x1) = ((w.a.x.min(w.b.x) / gs).floor() as i32, (w.a.x.max(w.b.x) / gs).floor() as i32);
            let (y0, y1) = ((w.a.y.min(w.b.y) / gs).floor() as i32, (w.a.y.max(w.b.y) / gs).floor() as i32);
            for gx in x0..=x1 {
                for gy in y0..=y1 {
                    grid.entry((gx, gy)).or_default().push(i);
                }
            }
        }
        self.walls = walls;
        self.wall_grid = grid;
    }

    pub fn has_walls(&self) -> bool {
        !self.walls.is_empty()
    }

    /// Move a cylinder (radius `radius`, from `z_feet` up `height`) from `from` to `to` in the XY plane,
    /// sliding along the HitMesh walls. Steps up to `step` metres tall are ignored. Returns the resolved XY.
    pub fn resolve_walls(&self, from: Vec2, to: Vec2, z_feet: f32, radius: f32, height: f32, step: f32) -> Vec2 {
        if self.walls.is_empty() {
            return to;
        }
        let gs = self.grid_size;
        let relevant = |w: &WallSegment| w.z_max > z_feet + step && w.z_min < z_feet + height;
        let collect = |center: Vec2, reach: f32| -> Vec<usize> {
            let (x0, x1) = (((center.x - reach) / gs).floor() as i32, ((center.x + reach) / gs).floor() as i32);
            let (y0, y1) = (((center.y - reach) / gs).floor() as i32, ((center.y + reach) / gs).floor() as i32);
            let mut v: Vec<usize> = Vec::new();
            for gx in x0..=x1 {
                for gy in y0..=y1 {
                    if let Some(l) = self.wall_grid.get(&(gx, gy)) {
                        v.extend_from_slice(l);
                    }
                }
            }
            v.sort_unstable();
            v.dedup();
            v
        };
        let mid = (from + to) * 0.5;
        let reach = (to - from).length() * 0.5 + radius + gs;
        let cand = collect(mid, reach);

        let mut q = to;
        // 1. Tunnelling guard: if the move crosses a wall, slide along that wall instead.
        for &i in &cand {
            let w = &self.walls[i];
            if !relevant(w) {
                continue;
            }
            if segments_cross(from, q, w.a, w.b) {
                let t = (w.b - w.a).normalize_or_zero();
                q = from + t * (q - from).dot(t);
            }
        }
        // 2. Push out of anything closer than `radius`, staying on the side we started from.
        for _ in 0..4 {
            let mut moved = false;
            for &i in &cand {
                let w = &self.walls[i];
                if !relevant(w) {
                    continue;
                }
                let c = closest_on_segment(w.a, w.b, q);
                let d = q - c;
                let dist = d.length();
                if dist < radius {
                    let ab = w.b - w.a;
                    let side = ab.perp_dot(from - w.a);
                    let n = Vec2::new(-ab.y, ab.x).normalize_or_zero();
                    let n = if side >= 0.0 { -n } else { n }; // perp_dot sign convention: choose normal pointing toward `from`
                    let n = if n.dot(from - c) < 0.0 { -n } else { n };
                    let dir = if dist > 1e-5 && d.dot(n) >= 0.0 { d / dist } else { n };
                    q = c + dir * radius;
                    moved = true;
                }
            }
            if !moved {
                break;
            }
        }
        q
    }

    /// Pull a camera back toward `target` so it never sits behind or inside collision geometry.
    /// Returns the (possibly shortened) eye position.
    /// First triangle hit on the segment target->eye (debugging camera clipping): (distance, vertices).
    pub fn first_hit(&self, target: Vec3, eye: Vec3) -> Option<(f32, [Vec3; 3])> {
        let dir = eye - target;
        let len = dir.length();
        if len < 1e-4 {
            return None;
        }
        let d = dir / len;
        let mut best: Option<(f32, [Vec3; 3])> = None;
        for tri in &self.triangles {
            if let Some(t) = ray_tri(target, d, tri.v0, tri.v1, tri.v2) {
                if t > 0.0 && t < len && best.map(|b| t < b.0).unwrap_or(true) {
                    best = Some((t, [tri.v0, tri.v1, tri.v2]));
                }
            }
        }
        best
    }

    pub fn clip_camera(&self, target: Vec3, eye: Vec3, margin: f32) -> Vec3 {
        let dir = eye - target;
        let len = dir.length();
        if len < 1e-4 || self.triangles.is_empty() {
            return eye;
        }
        let d = dir / len;
        let gs = self.grid_size;
        let mut best = len;
        let mut seen: std::collections::HashSet<(i32, i32)> = std::collections::HashSet::new();
        let steps = (len / (gs * 0.5)).ceil() as i32 + 1;
        for s in 0..=steps {
            let p = target + d * (len * s as f32 / steps as f32);
            let cell = ((p.x / gs).floor() as i32, (p.y / gs).floor() as i32);
            if !seen.insert(cell) {
                continue;
            }
            if let Some(list) = self.grid.get(&cell) {
                for &ti in list {
                    let tri = &self.triangles[ti];
                    if let Some(t) = ray_tri(target, d, tri.v0, tri.v1, tri.v2) {
                        if t > 0.0 && t < best {
                            best = t;
                        }
                    }
                }
            }
        }
        if best < len {
            target + d * (best - margin).max(0.3).min(len)
        } else {
            eye
        }
    }

    /// Query the ground surface height directly underneath position (X, Y) within tolerance of `current_z`.
    pub fn get_ground_height(&self, x: f32, y: f32, current_z: f32) -> Option<f32> {
        let gx = (x / self.grid_size).floor() as i32;
        let gy = (y / self.grid_size).floor() as i32;

        let candidate_indices = self.grid.get(&(gx, gy))?;
        let p = Vec2::new(x, y);

        let mut best_height: Option<f32> = None;
        let mut min_dz = f32::INFINITY;

        for &idx in candidate_indices {
            let tri = &self.triangles[idx];
            if !tri.is_walkable {
                continue;
            }

            if let Some(h) = tri.height_at(p) {
                // Must be reasonably near the player's current height (e.g. within 3.5 meters above or below)
                let dz = (h - current_z).abs();
                if (h - current_z) <= 1.5 && (current_z - h) <= 3.5 {
                    if dz < min_dz {
                        min_dz = dz;
                        best_height = Some(h);
                    }
                }
            }
        }

        best_height
    }

    /// Floor for a walking character: the highest walkable floor at (x, y) that is at most `step` above `current_z`
    /// (and at most 3.5 m below it). Where floors overlap (a ramp rising out of the grass, a bridge deck over the
    /// ground) this steps up onto the higher one; `get_ground_height` (nearest floor) would stay underneath.
    pub fn get_walk_height(&self, x: f32, y: f32, current_z: f32, step: f32) -> Option<f32> {
        let gx = (x / self.grid_size).floor() as i32;
        let gy = (y / self.grid_size).floor() as i32;
        let candidate_indices = self.grid.get(&(gx, gy))?;
        let p = Vec2::new(x, y);
        let mut best: Option<f32> = None;
        for &idx in candidate_indices {
            let tri = &self.triangles[idx];
            if !tri.is_walkable {
                continue;
            }
            if let Some(h) = tri.height_at(p) {
                if h - current_z <= step && current_z - h <= 3.5 && best.is_none_or(|b| h > b) {
                    best = Some(h);
                }
            }
        }
        best
    }

    /// Remove the parts of walls that lie inside the axis-aligned rectangle [min, max] (e.g. a bridge deck crossing
    /// the shore's sea wall). Walls are clipped, not dropped, so the rest of the barrier stays.
    pub fn cut_walls_in_rect(&mut self, min: Vec2, max: Vec2) {
        let mut out = Vec::with_capacity(self.walls.len());
        for w in &self.walls {
            // Liang-Barsky: parameter interval [t0, t1] of the segment inside the rectangle.
            let d = w.b - w.a;
            let (mut t0, mut t1) = (0.0f32, 1.0f32);
            let mut inside = true;
            for (p, q) in [(-d.x, w.a.x - min.x), (d.x, max.x - w.a.x), (-d.y, w.a.y - min.y), (d.y, max.y - w.a.y)] {
                if p.abs() < 1e-9 {
                    if q < 0.0 {
                        inside = false;
                        break;
                    }
                } else {
                    let r = q / p;
                    if p < 0.0 {
                        t0 = t0.max(r);
                    } else {
                        t1 = t1.min(r);
                    }
                }
            }
            if !inside || t0 >= t1 {
                out.push(w.clone());
                continue;
            }
            if t0 > 1e-4 {
                out.push(WallSegment { a: w.a, b: w.a + d * t0, z_min: w.z_min, z_max: w.z_max });
            }
            if t1 < 1.0 - 1e-4 {
                out.push(WallSegment { a: w.a + d * t1, b: w.b, z_min: w.z_min, z_max: w.z_max });
            }
        }
        self.set_walls(out);
    }

    /// Merge another collision mesh into this one.
    pub fn merge(&mut self, other: &Self) {
        let wall_base = self.walls.len();
        self.walls.extend_from_slice(&other.walls);
        for ((gx, gy), indices) in &other.wall_grid {
            let entry = self.wall_grid.entry((*gx, *gy)).or_default();
            for &i in indices {
                entry.push(wall_base + i);
            }
        }
        let base_idx = self.triangles.len();
        self.triangles.extend_from_slice(&other.triangles);
        for ((gx, gy), indices) in &other.grid {
            let entry = self.grid.entry((*gx, *gy)).or_default();
            for &i in indices {
                entry.push(base_idx + i);
            }
        }
    }
}


fn closest_on_segment(a: Vec2, b: Vec2, p: Vec2) -> Vec2 {
    let ab = b - a;
    let l2 = ab.length_squared();
    if l2 < 1e-12 {
        return a;
    }
    a + ab * ((p - a).dot(ab) / l2).clamp(0.0, 1.0)
}

fn segments_cross(p1: Vec2, p2: Vec2, q1: Vec2, q2: Vec2) -> bool {
    let d1 = (p2 - p1).perp_dot(q1 - p1);
    let d2 = (p2 - p1).perp_dot(q2 - p1);
    let d3 = (q2 - q1).perp_dot(p1 - q1);
    let d4 = (q2 - q1).perp_dot(p2 - q1);
    d1 * d2 < 0.0 && d3 * d4 < 0.0
}

/// Moller-Trumbore, two-sided. Returns distance along `dir` (unit) to the hit.
fn ray_tri(o: Vec3, dir: Vec3, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32> {
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let p = dir.cross(e2);
    let det = e1.dot(p);
    if det.abs() < 1e-9 {
        return None;
    }
    let inv = 1.0 / det;
    let t = o - v0;
    let u = t.dot(p) * inv;
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = t.cross(e1);
    let v = dir.dot(q) * inv;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    Some(e2.dot(q) * inv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collision_triangle_walkable() {
        // Flat triangle facing straight up (+Z)
        let t = CollisionTriangle::new(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::new(10.0, 0.0, 5.0),
            Vec3::new(0.0, 10.0, 5.0),
        ).unwrap();

        assert!(t.is_walkable);
        assert_eq!(t.height_at(Vec2::new(2.0, 2.0)), Some(5.0));
        assert_eq!(t.height_at(Vec2::new(15.0, 15.0)), None);
    }

    #[test]
    fn test_stage_ground_height_mac_anu() {
        let path = std::path::Path::new("../../target/vol1/data/data/town/town01_00h.ccs");
        if path.exists() {
            let data = std::fs::read(path).unwrap();
            let ccs = hackgu_ccs::CcsFile::parse(&data).unwrap();
            let col = CollisionMesh::from_ccs_stage(&ccs);

            assert!(!col.triangles.is_empty(), "Collision mesh should contain triangles");
            println!("Mac Anu Central Plaza collision triangles: {}", col.triangles.len());

            let mut nearest_tri = None;
            let mut nearest_dist = f32::INFINITY;
            for t in &col.triangles {
                let center_2d = glam::Vec2::new((t.v0.x + t.v1.x + t.v2.x) / 3.0, (t.v0.y + t.v1.y + t.v2.y) / 3.0);
                let d = center_2d.distance(glam::Vec2::new(0.0, 25.0));
                if d < nearest_dist {
                    nearest_dist = d;
                    nearest_tri = Some(*t);
                }
            }
            if let Some(t) = nearest_tri {
                println!("Nearest triangle to (0, 25): dist={:.2}, norm={:?}, z=[{:.2}, {:.2}, {:.2}]",
                    nearest_dist, t.normal, t.v0.z, t.v1.z, t.v2.z);
            }

            // Query height near spawn point (0.0, 25.0) with expected floor level ~12.0
            let h = col.get_ground_height(0.0, 25.0, 12.0);
            println!("Ground height at spawn (0, 25) with current_z=12: {:?}", h);
        }
    }
}

