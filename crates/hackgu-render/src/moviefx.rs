//! # In-window movie playback
//! Decodes CRI .usm movies through two `ffmpeg` pipes: rawvideo frames, and the audio (5.1 ADX in the
//! game's movies) downmixed to stereo f32 PCM that is played in-process through rodio. The video clock
//! starts when the first audio chunk is queued so picture and sound stay together. (Audio used to go
//! through a separate `ffplay -nodisp` process, which glitched.)

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, TryRecvError};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::video::MovieController;

const MAX_WIDTH: u32 = 1280;
const DEFAULT_FPS: f64 = 29.97;

/// One decoded RGBA frame.
type Frame = (Vec<u8>, u32, u32);

fn hide_window(cmd: &mut Command) -> &mut Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    cmd
}

/// Resolve a movie name or path to a file on disk.
pub fn resolve_path(name_or_path: &str) -> Option<PathBuf> {
    let direct = Path::new(name_or_path);
    if direct.is_file() {
        return Some(direct.to_path_buf());
    }
    if let Some(p) = MovieController::resolve_movie(name_or_path) {
        return Some(p);
    }
    let extra = [
        crate::gamedir::game_path("cmn/movie"),
        std::path::PathBuf::from(r"cmn\movie"),
        crate::gamedir::game_path("cmn/movie/vol1/online"),
        crate::gamedir::game_path("cmn/movie/vol1/news"),
    ];
    extra
        .iter()
        .map(|d| d.join(name_or_path))
        .find(|p| p.is_file())
}

/// Returns (width, height, fps) of the first video stream via ffprobe.
fn probe(path: &Path) -> Option<(u32, u32, f64)> {
    let mut cmd = Command::new("ffprobe");
    hide_window(&mut cmd)
        .args(["-v", "error", "-select_streams", "v:0", "-show_entries"])
        .arg("stream=width,height,r_frame_rate")
        .args(["-of", "csv=p=0"])
        .arg(path)
        .stdin(Stdio::null())
        .stderr(Stdio::null());
    let out = cmd.output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let mut it = text.lines().next()?.split(',');
    let w: u32 = it.next()?.trim().parse().ok()?;
    let h: u32 = it.next()?.trim().parse().ok()?;
    let fps = it
        .next()
        .and_then(|s| {
            let (n, d) = s.trim().split_once('/')?;
            let (n, d): (f64, f64) = (n.parse().ok()?, d.parse().ok()?);
            if d > 0.0 && n > 0.0 {
                Some(n / d)
            } else {
                None
            }
        })
        .unwrap_or(DEFAULT_FPS);
    if w == 0 || h == 0 {
        return None;
    }
    Some((w, h, fps))
}

struct Playback {
    video: Child,
    /// ffmpeg process decoding the audio track to PCM.
    audio: Option<Child>,
    audio_player: Option<Arc<rodio::Player>>,
    audio_thread: Option<JoinHandle<()>>,
    rx: Receiver<Frame>,
    pending: Option<Frame>,
    done: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
    looped: bool,
}

impl Playback {
    fn kill(&mut self) {
        if let Some(p) = self.audio_player.take() {
            p.stop();
        }
        let _ = self.video.kill();
        if let Some(a) = self.audio.as_mut() {
            let _ = a.kill();
        }
        let _ = self.video.wait();
        if let Some(mut a) = self.audio.take() {
            let _ = a.wait();
        }
        if let Some(t) = self.audio_thread.take() {
            let _ = t.join();
        }
        // Dropping the receiver unblocks a reader stuck on a full channel.
        if let Some(t) = self.thread.take() {
            // Drain so the sender cannot block forever.
            while self.rx.try_recv().is_ok() {}
            let _ = t.join();
        }
    }
}

impl Drop for Playback {
    fn drop(&mut self) {
        self.kill();
    }
}

/// Plays movies into RGBA frames that the host uploads to a [`MovieRenderer`].
pub struct MoviePlayer {
    current: Option<Playback>,
    /// Audio output for movie soundtracks (opened on first use).
    sink: Option<rodio::MixerDeviceSink>,
}

impl Default for MoviePlayer {
    fn default() -> Self {
        Self::new()
    }
}

impl MoviePlayer {
    pub fn new() -> Self {
        Self { current: None, sink: None }
    }

    /// Start a movie (by name such as `OP10000.usm` or by path). Stops any current movie.
    pub fn play(&mut self, name_or_path: &str, looped: bool) -> Result<(), String> {
        self.stop();
        let path = resolve_path(name_or_path)
            .ok_or_else(|| format!("Movie file '{}' not found", name_or_path))?;
        let (sw, sh, fps) = probe(&path).unwrap_or((1280, 720, DEFAULT_FPS));
        let w = sw.min(MAX_WIDTH) & !1;
        let h = ((w as u64 * sh as u64 / sw as u64) as u32) & !1;
        let (w, h) = (w.max(2), h.max(2));

        let mut vcmd = Command::new("ffmpeg");
        hide_window(&mut vcmd).args(["-v", "quiet", "-nostdin"]);
        if looped {
            vcmd.args(["-stream_loop", "-1"]);
        }
        vcmd.arg("-i")
            .arg(&path)
            .args(["-an", "-vf"])
            .arg(format!("scale={}:{}:flags=fast_bilinear", w, h))
            .args(["-f", "rawvideo", "-pix_fmt", "rgba", "-"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        let mut acmd = Command::new("ffmpeg");
        hide_window(&mut acmd).args(["-v", "quiet", "-nostdin"]);
        if looped {
            acmd.args(["-stream_loop", "-1"]);
        }
        acmd.arg("-i")
            .arg(&path)
            .args(["-vn", "-ac", "2", "-ar", "48000", "-af", "volume=0.82", "-f", "f32le", "-"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        let mut video = vcmd
            .spawn()
            .map_err(|e| format!("Failed to launch ffmpeg: {e}"))?;
        let mut audio = acmd.spawn().ok();
        let mut stdout = video.stdout.take().ok_or("ffmpeg stdout unavailable")?;

        // Audio: stream 0.25 s chunks of stereo f32 into a rodio player.
        let audio_started = Arc::new(AtomicBool::new(false));
        if self.sink.is_none() {
            self.sink = rodio::DeviceSinkBuilder::open_default_sink().ok();
        }
        let audio_player = self.sink.as_ref().map(|s| Arc::new(rodio::Player::connect_new(s.mixer())));
        let audio_thread = match (audio.as_mut().and_then(|a| a.stdout.take()), audio_player.clone()) {
            (Some(mut aout), Some(player)) => {
                let started = audio_started.clone();
                Some(std::thread::spawn(move || {
                    let (ch, rate) = (std::num::NonZeroU16::new(2).unwrap(), std::num::NonZeroU32::new(48000).unwrap());
                    let mut buf = vec![0u8; 48000 / 4 * 2 * 4];
                    loop {
                        let mut filled = 0;
                        while filled < buf.len() {
                            match aout.read(&mut buf[filled..]) {
                                Ok(0) | Err(_) => break,
                                Ok(n) => filled += n,
                            }
                        }
                        let usable = filled / 8 * 8;
                        if usable == 0 {
                            break;
                        }
                        let samples: Vec<f32> = buf[..usable].chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
                        player.append(rodio::buffer::SamplesBuffer::new(ch, rate, samples));
                        started.store(true, Ordering::SeqCst);
                        if usable < buf.len() {
                            break;
                        }
                    }
                    started.store(true, Ordering::SeqCst);
                }))
            }
            _ => {
                audio_started.store(true, Ordering::SeqCst);
                None
            }
        };
        let audio_started_v = audio_started.clone();

        let (tx, rx) = sync_channel::<Frame>(3);
        let done = Arc::new(AtomicBool::new(false));
        let done_t = done.clone();
        let frame_len = (w * h * 4) as usize;
        let interval = Duration::from_secs_f64(1.0 / fps);
        let thread = std::thread::spawn(move || {
            let mut start: Option<Instant> = None;
            let mut index: u32 = 0;
            loop {
                let mut buf = vec![0u8; frame_len];
                if stdout.read_exact(&mut buf).is_err() {
                    break;
                }
                if start.is_none() {
                    // Start the picture clock together with the sound (or after 1 s if there is none).
                    let wait = Instant::now();
                    while !audio_started_v.load(Ordering::SeqCst) && wait.elapsed() < Duration::from_secs(1) {
                        std::thread::sleep(Duration::from_millis(2));
                    }
                }
                let t0 = *start.get_or_insert_with(Instant::now);
                let due = t0 + interval * index;
                index += 1;
                let now = Instant::now();
                if due > now {
                    std::thread::sleep(due - now);
                }
                if tx.send((buf, w, h)).is_err() {
                    break;
                }
            }
            done_t.store(true, Ordering::SeqCst);
        });

        self.current = Some(Playback {
            video,
            audio,
            audio_player,
            audio_thread,
            rx,
            pending: None,
            done,
            thread: Some(thread),
            looped,
        });
        Ok(())
    }

    /// Stop playback and kill both child processes.
    pub fn stop(&mut self) {
        if let Some(mut p) = self.current.take() {
            p.kill();
        }
    }

    /// Abort the current movie (same as [`stop`](Self::stop)).
    pub fn skip(&mut self) {
        self.stop();
    }

    pub fn is_playing(&self) -> bool {
        self.current.is_some()
    }

    /// True exactly once when a non-looped movie has ended and all frames were consumed.
    pub fn poll_finished(&mut self) -> bool {
        let Some(p) = self.current.as_mut() else {
            return false;
        };
        if p.pending.is_none() {
            match p.rx.try_recv() {
                Ok(f) => p.pending = Some(f),
                Err(TryRecvError::Empty) => {
                    if !p.done.load(Ordering::SeqCst) {
                        return false;
                    }
                    // Reader finished; re-check for a frame sent just before `done`.
                    if let Ok(f) = p.rx.try_recv() {
                        p.pending = Some(f);
                        return false;
                    }
                }
                Err(TryRecvError::Disconnected) => {}
            }
        }
        if p.pending.is_some() {
            return false;
        }
        // Stream ended (looped movies only end on error).
        let _ = p.looped;
        self.stop();
        true
    }

    /// Newest available frame as `(rgba, width, height)`, or `None` if nothing new.
    pub fn latest_frame(&mut self) -> Option<Frame> {
        let p = self.current.as_mut()?;
        let mut latest = p.pending.take();
        while let Ok(f) = p.rx.try_recv() {
            latest = Some(f);
        }
        latest
    }
}

impl Drop for MoviePlayer {
    fn drop(&mut self) {
        self.stop();
    }
}

const SHADER: &str = r#"
struct Params { scale: vec2<f32>, pad: vec2<f32> };
@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var tex: texture_2d<f32>;
@group(0) @binding(2) var samp: sampler;

struct VOut { @builtin(position) pos: vec4<f32>, @location(0) uv: vec2<f32> };

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VOut {
    var p = array<vec2<f32>, 6>(
        vec2(-1.0, -1.0), vec2(1.0, -1.0), vec2(-1.0, 1.0),
        vec2(-1.0, 1.0), vec2(1.0, -1.0), vec2(1.0, 1.0));
    let q = p[i];
    var o: VOut;
    o.pos = vec4<f32>(q * params.scale, 0.0, 1.0);
    o.uv = vec2<f32>(q.x * 0.5 + 0.5, 0.5 - q.y * 0.5);
    return o;
}

@fragment
fn fs_main(in: VOut) -> @location(0) vec4<f32> {
    return vec4<f32>(textureSample(tex, samp, in.uv).rgb, 1.0);
}
"#;

/// Draws movie frames as an aspect-preserving (letterboxed) fullscreen quad.
pub struct MovieRenderer {
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    uniform: wgpu::Buffer,
    texture: Option<(wgpu::Texture, wgpu::BindGroup, u32, u32)>,
    window: (f32, f32),
    scale: [f32; 2],
}

impl MovieRenderer {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("moviefx shader"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("moviefx bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let pl = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("moviefx pl"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("moviefx pipeline"),
            layout: Some(&pl),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("moviefx sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("moviefx uniform"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            pipeline,
            layout,
            sampler,
            uniform,
            texture: None,
            window: (16.0, 9.0),
            scale: [1.0, 1.0],
        }
    }

    /// Letterbox scale for a video of `vw`x`vh` in a window of `ww`x`wh`.
    fn compute_scale(ww: f32, wh: f32, vw: f32, vh: f32) -> [f32; 2] {
        if ww <= 0.0 || wh <= 0.0 || vw <= 0.0 || vh <= 0.0 {
            return [1.0, 1.0];
        }
        let win_aspect = ww / wh;
        let vid_aspect = vw / vh;
        if vid_aspect > win_aspect {
            [1.0, win_aspect / vid_aspect]
        } else {
            [vid_aspect / win_aspect, 1.0]
        }
    }

    fn write_uniform(&mut self, queue: &wgpu::Queue) {
        let (vw, vh) = self
            .texture
            .as_ref()
            .map(|t| (t.2 as f32, t.3 as f32))
            .unwrap_or((16.0, 9.0));
        self.scale = Self::compute_scale(self.window.0, self.window.1, vw, vh);
        let data = [self.scale[0], self.scale[1], 0.0, 0.0];
        queue.write_buffer(&self.uniform, 0, bytemuck_cast(&data));
    }

    /// Tell the renderer the current window size so the quad is letterboxed.
    pub fn set_target_aspect(&mut self, queue: &wgpu::Queue, window_w: u32, window_h: u32) {
        self.window = (window_w as f32, window_h as f32);
        self.write_uniform(queue);
    }

    /// Upload an RGBA frame (`rgba.len() == w*h*4`); recreates the texture on size change.
    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: &[u8],
        w: u32,
        h: u32,
    ) {
        if w == 0 || h == 0 || rgba.len() < (w * h * 4) as usize {
            return;
        }
        let recreate = !matches!(&self.texture, Some(t) if t.2 == w && t.3 == h);
        if recreate {
            let tex = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("moviefx frame"),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
            let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
            let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("moviefx bg"),
                layout: &self.layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: self.uniform.as_entire_binding() },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&self.sampler),
                    },
                ],
            });
            self.texture = Some((tex, bg, w, h));
            self.write_uniform(queue);
        }
        if let Some((tex, _, _, _)) = &self.texture {
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: tex,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                &rgba[..(w * h * 4) as usize],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(w * 4),
                    rows_per_image: Some(h),
                },
                wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
            );
        }
    }

    /// Drop the current frame so nothing is drawn until the next upload.
    pub fn clear(&mut self) {
        self.texture = None;
    }

    pub fn has_frame(&self) -> bool {
        self.texture.is_some()
    }

    /// Draw the letterboxed frame (no-op if nothing has been uploaded).
    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if let Some((_, bg, _, _)) = &self.texture {
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, bg, &[]);
            pass.draw(0..6, 0..1);
        }
    }
}

fn bytemuck_cast(data: &[f32; 4]) -> &[u8] {
    // SAFETY: f32 has no padding and any byte pattern is a valid u8.
    unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, 16) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_movies() {
        for n in ["OP10000.usm", "LP1000.usm", "LOGO_H.usm", "MM10145.usm"] {
            let p = resolve_path(n).unwrap_or_else(|| panic!("{n} must resolve"));
            assert!(p.is_file());
        }
    }

    #[test]
    fn letterbox_scale() {
        let s = MovieRenderer::compute_scale(1000.0, 1000.0, 1280.0, 720.0);
        assert!((s[0] - 1.0).abs() < 1e-6 && (s[1] - 0.5625).abs() < 1e-4);
        let s = MovieRenderer::compute_scale(2560.0, 720.0, 1280.0, 720.0);
        assert!((s[1] - 1.0).abs() < 1e-6 && s[0] < 1.0);
    }

    #[test]
    fn plays_frames_and_stops_cleanly() {
        let mut m = MoviePlayer::new();
        m.play("OP10000.usm", false).unwrap();
        assert!(m.is_playing());
        let start = Instant::now();
        let mut count = 0;
        while start.elapsed() < Duration::from_secs(2) {
            if let Some((buf, w, h)) = m.latest_frame() {
                assert_eq!(buf.len(), (w * h * 4) as usize);
                count += 1;
            }
            assert!(!m.poll_finished());
            std::thread::sleep(Duration::from_millis(5));
        }
        assert!(count >= 10, "only {count} frames");
        let audio_id = m.current.as_ref().unwrap().audio.as_ref().map(|a| a.id());
        m.stop();
        assert!(!m.is_playing());
        if let Some(id) = audio_id {
            let out = Command::new("tasklist")
                .args(["/FI", &format!("PID eq {id}"), "/NH"])
                .output()
                .unwrap();
            assert!(!String::from_utf8_lossy(&out.stdout).contains(&id.to_string()));
        }
    }
}
