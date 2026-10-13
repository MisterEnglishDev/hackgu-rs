//! hitimg file.ccs outdir : top-down max-height raster (64 px per 16 m, +y up) of every HitMesh, raw f32 per pixel
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for h in c.hit_meshes() {
        let name = c.object_name(h.object_id).unwrap_or("?");
        let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
        for r in &h.records { for v in &r.positions { for k in 0..2 { lo[k] = lo[k].min(v[k] * 0.01); hi[k] = hi[k].max(v[k] * 0.01); } } }
        let ppm = 4.0f32;
        let (w, hh) = (((hi[0] - lo[0]) * ppm).ceil() as usize + 1, ((hi[1] - lo[1]) * ppm).ceil() as usize + 1);
        let mut img = vec![f32::NAN; w * hh];
        for r in &h.records {
            for t in r.positions.chunks_exact(3) {
                let p: Vec<[f32; 3]> = t.iter().map(|v| [(v[0] * 0.01 - lo[0]) * ppm, (hi[1] - v[1] * 0.01) * ppm, v[2] * 0.01]).collect();
                let (minx, maxx) = (p.iter().map(|q| q[0]).fold(f32::MAX, f32::min) as i32, p.iter().map(|q| q[0]).fold(f32::MIN, f32::max).ceil() as i32);
                let (miny, maxy) = (p.iter().map(|q| q[1]).fold(f32::MAX, f32::min) as i32, p.iter().map(|q| q[1]).fold(f32::MIN, f32::max).ceil() as i32);
                for y in miny.max(0)..=maxy.min(hh as i32 - 1) { for x in minx.max(0)..=maxx.min(w as i32 - 1) {
                    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                    let d = (p[1][1] - p[2][1]) * (p[0][0] - p[2][0]) + (p[2][0] - p[1][0]) * (p[0][1] - p[2][1]);
                    if d.abs() < 1e-6 { // vertical triangle: mark its footprint line with max z
                        continue;
                    }
                    let l1 = ((p[1][1] - p[2][1]) * (px - p[2][0]) + (p[2][0] - p[1][0]) * (py - p[2][1])) / d;
                    let l2 = ((p[2][1] - p[0][1]) * (px - p[2][0]) + (p[0][0] - p[2][0]) * (py - p[2][1])) / d;
                    let l3 = 1.0 - l1 - l2;
                    if l1 < -0.01 || l2 < -0.01 || l3 < -0.01 { continue; }
                    let z = l1 * p[0][2] + l2 * p[1][2] + l3 * p[2][2];
                    let i = y as usize * w + x as usize;
                    if img[i].is_nan() || z > img[i] { img[i] = z; }
                } }
            }
        }
        let bytes: Vec<u8> = img.iter().flat_map(|v| v.to_le_bytes()).collect();
        std::fs::write(format!("{}/{}_{}x{}_{:.1}_{:.1}_{:.1}_{:.1}.f32", a[2], name, w, hh, lo[0], lo[1], hi[0], hi[1]), bytes).unwrap();
    }
}
