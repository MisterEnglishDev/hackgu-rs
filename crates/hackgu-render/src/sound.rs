use std::io::Read;
use std::num::{NonZeroU16, NonZeroU32};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;

use rodio::buffer::SamplesBuffer;
use rodio::Source;

/// Header information parsed from a CRI AWB (AFS2) sound bank archive.
#[derive(Debug, Clone)]
pub struct AwbArchive {
    pub path: PathBuf,
    pub count: usize,
    pub align: usize,
    pub offsets: Vec<u32>,
}

impl AwbArchive {
    /// Open and parse an AFS2 / AWB audio archive.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_buf = path.as_ref().to_path_buf();
        let mut file = std::fs::File::open(&path_buf)
            .map_err(|e| format!("Failed to open AWB {}: {}", path_buf.display(), e))?;

        let mut header = [0u8; 16];
        file.read_exact(&mut header)
            .map_err(|e| format!("Failed to read AWB header: {}", e))?;

        if &header[0..4] != b"AFS2" {
            return Err("Invalid AWB magic (expected AFS2)".into());
        }

        let count = u32::from_le_bytes(header[8..12].try_into().unwrap()) as usize;
        let align = u32::from_le_bytes(header[12..16].try_into().unwrap()) as usize;

        // Skip cue IDs (count * 2 bytes) to reach the offsets table
        let offsets_start = 16 + count * 2;
        let mut offsets_bytes = vec![0u8; (count + 1) * 4];
        use std::io::Seek;
        file.seek(std::io::SeekFrom::Start(offsets_start as u64))
            .map_err(|e| format!("Failed to seek to offsets: {}", e))?;
        file.read_exact(&mut offsets_bytes)
            .map_err(|e| format!("Failed to read offset table: {}", e))?;

        let mut offsets = Vec::with_capacity(count + 1);
        for i in 0..=count {
            let off = u32::from_le_bytes(offsets_bytes[i * 4..(i + 1) * 4].try_into().unwrap());
            offsets.push(off);
        }

        Ok(AwbArchive {
            path: path_buf,
            count,
            align,
            offsets,
        })
    }

    /// Extract a sound track / voice cue as raw HCA audio bytes.
    pub fn get_hca_bytes(&self, cue_index: usize) -> Option<Vec<u8>> {
        if cue_index >= self.count || cue_index + 1 >= self.offsets.len() {
            return None;
        }

        let raw_start = self.offsets[cue_index] as usize;
        let aligned_start = (raw_start + (self.align - 1)) & !(self.align - 1);
        let end = self.offsets[cue_index + 1] as usize;

        if end <= aligned_start {
            return None;
        }

        let len = end - aligned_start;
        use std::io::Seek;
        let mut file = std::fs::File::open(&self.path).ok()?;
        file.seek(std::io::SeekFrom::Start(aligned_start as u64)).ok()?;
        let mut data = vec![0u8; len];
        file.read_exact(&mut data).ok()?;
        Some(data)
    }
}

/// Decoded PCM audio ready for playback.
struct Pcm {
    channels: u16,
    rate: u32,
    samples: Vec<f32>,
    /// Loop region in sample frames (start, end) when the HCA carries loop points.
    loop_region: Option<(usize, usize)>,
}

fn decode_hca(bytes: Vec<u8>) -> Option<Pcm> {
    let mut dec = cridecoder::HcaDecoder::from_reader(std::io::Cursor::new(bytes)).ok()?;
    let info = dec.info().clone();
    let samples = dec.decode_all().ok()?;
    let ch = info.channel_count.max(1) as usize;
    let frames = samples.len() / ch;
    let loop_region = if info.loop_enabled {
        let spb = info.samples_per_block;
        let delay = info.encoder_delay as usize;
        let start = (info.loop_start_block as usize * spb + info.loop_start_delay as usize).saturating_sub(delay);
        let end = ((info.loop_end_block as usize + 1) * spb)
            .saturating_sub(info.loop_end_padding as usize)
            .saturating_sub(delay)
            .min(frames);
        if end > start { Some((start, end)) } else { None }
    } else {
        None
    };
    Some(Pcm { channels: info.channel_count as u16, rate: info.sampling_rate, samples, loop_region })
}

/// Debug trace of channel activity (set HACKGU_AUDIO_LOG=1).
fn alog(msg: &str) {
    use std::sync::OnceLock;
    static T0: OnceLock<std::time::Instant> = OnceLock::new();
    if std::env::var_os("HACKGU_AUDIO_LOG").is_some() {
        let t = T0.get_or_init(std::time::Instant::now).elapsed().as_secs_f32();
        eprintln!("[audio {:8.3}] {}", t, msg);
    }
}

const IDLE: u8 = 0;
const DECODING: u8 = 1;
const QUEUED: u8 = 2;

/// One playback channel (BGM or voice). A generation counter makes a decode that finishes after
/// a stop/replace get discarded instead of played.
struct Channel {
    player: Arc<rodio::Player>,
    generation: Arc<AtomicU64>,
    state: Arc<AtomicU8>,
}

impl Channel {
    fn new(mixer: &rodio::mixer::Mixer) -> Self {
        Self {
            player: Arc::new(rodio::Player::connect_new(mixer)),
            generation: Arc::new(AtomicU64::new(0)),
            state: Arc::new(AtomicU8::new(IDLE)),
        }
    }

    fn stop(&self) {
        alog("stop");
        self.generation.fetch_add(1, Ordering::SeqCst);
        self.player.stop();
        self.state.store(IDLE, Ordering::SeqCst);
    }

    fn is_playing(&self) -> bool {
        match self.state.load(Ordering::SeqCst) {
            DECODING => true,
            QUEUED => !self.player.empty(),
            _ => false,
        }
    }

    /// Decode on a worker thread (BGM tracks are several MB of HCA) and queue the result.
    fn play(&self, hca: Vec<u8>, volume: f32, looped: bool) {
        alog(&format!("play {} bytes looped={}", hca.len(), looped));
        self.stop();
        let gen = self.generation.load(Ordering::SeqCst);
        self.state.store(DECODING, Ordering::SeqCst);
        self.player.set_volume(volume);
        let (player, generation, state) = (self.player.clone(), self.generation.clone(), self.state.clone());
        std::thread::spawn(move || {
            let pcm = decode_hca(hca);
            if generation.load(Ordering::SeqCst) != gen {
                return;
            }
            let Some(pcm) = pcm else {
                state.store(IDLE, Ordering::SeqCst);
                return;
            };
            let (Some(ch), Some(rate)) = (NonZeroU16::new(pcm.channels), NonZeroU32::new(pcm.rate)) else {
                state.store(IDLE, Ordering::SeqCst);
                return;
            };
            let c = ch.get() as usize;
            if looped {
                match pcm.loop_region {
                    Some((s, e)) => {
                        if s > 0 {
                            player.append(SamplesBuffer::new(ch, rate, pcm.samples[..s * c].to_vec()));
                        }
                        player.append(SamplesBuffer::new(ch, rate, pcm.samples[s * c..e * c].to_vec()).repeat_infinite());
                    }
                    None => player.append(SamplesBuffer::new(ch, rate, pcm.samples).repeat_infinite()),
                }
            } else {
                player.append(SamplesBuffer::new(ch, rate, pcm.samples));
            }
            state.store(QUEUED, Ordering::SeqCst);
            alog("queued");
        });
    }
}

/// Voice / BGM audio: HCA from the CRI AWB banks is decoded in-process and played through the
/// default output device (no external player required).
pub struct SoundSystem {
    voice_awb: Option<AwbArchive>,
    bgm_awb: Option<AwbArchive>,
    // Kept alive for the lifetime of the channels.
    _sink: Option<rodio::MixerDeviceSink>,
    bgm: Option<Channel>,
    voice: Option<Channel>,
    current_bgm_track: Option<u32>,
    /// Cue table of bgm.acb (event scripts and areas name cues, not AWB entries).
    bgm_cues: crate::acb::AcbCues,
    /// Fade-out in progress: (seconds left, total).
    bgm_fade: Option<(f32, f32)>,
    pub bgm_volume: f32,
    pub voice_volume: f32,
}

impl SoundSystem {
    pub fn new() -> Self {
        // Resolve default audio paths
        let sound_root_buf = crate::gamedir::game_path("cmn/data/data/sound/PC");
        let sound_root = sound_root_buf.as_path();

        let jp_voice_path = sound_root.join("JP/v_evVoiceVol1.awb");
        let us_voice_path = sound_root.join("US/v_evVoiceVol1.awb");
        // English (US) voices by default, matching the English text; JP only if the US bank is missing.
        let voice_path = if us_voice_path.exists() { us_voice_path } else { jp_voice_path };
        let voice_awb = if voice_path.exists() { AwbArchive::open(&voice_path).ok() } else { None };

        let bgm_path = sound_root.join("bgm.awb");
        let bgm_awb = if bgm_path.exists() { AwbArchive::open(&bgm_path).ok() } else { None };

        let sink = match rodio::DeviceSinkBuilder::open_default_sink() {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("audio: no output device ({e}); sound disabled");
                None
            }
        };
        let (bgm, voice) = match &sink {
            Some(s) => (Some(Channel::new(s.mixer())), Some(Channel::new(s.mixer()))),
            None => (None, None),
        };

        let bgm_cues = std::fs::read(sound_root.join("bgm.acb")).ok().and_then(|d| crate::acb::AcbCues::parse(&d)).unwrap_or_default();
        Self { voice_awb, bgm_awb, _sink: sink, bgm, voice, current_bgm_track: None, bgm_cues, bgm_fade: None, bgm_volume: 0.75, voice_volume: 0.95 }
    }

    /// Configure voice bank language ("US" or "JP").
    pub fn set_voice_language(&mut self, lang: &str) {
        let sound_root_buf = crate::gamedir::game_path("cmn/data/data/sound/PC");
        let sound_root = sound_root_buf.as_path();
        let voice_path = sound_root.join(format!("{}/v_evVoiceVol1.awb", lang));
        if voice_path.exists() {
            self.voice_awb = AwbArchive::open(&voice_path).ok();
        }
    }

    /// Play a BGM cue of bgm.acb by index (as event scripts give it: `BGM操作,123,<slot>,<cue>`), looping when the
    /// cue's waveform loops.
    pub fn play_bgm(&mut self, cue: u32) {
        alog(&format!("bgm cue {}", cue));
        if self.current_bgm_track == Some(cue) && self.bgm_fade.is_none() && self.is_bgm_playing() {
            return;
        }
        self.stop_bgm();
        let Some(w) = self.bgm_cues.by_index.get(&cue).copied() else {
            eprintln!("audio: bgm cue {} not in bgm.acb", cue);
            return;
        };
        let (Some(awb), Some(ch)) = (&self.bgm_awb, &self.bgm) else { return };
        let Some(hca) = awb.get_hca_bytes(w.awb_id as usize) else { return };
        ch.play(hca, self.bgm_volume, w.looping);
        self.current_bgm_track = Some(cue);
    }

    /// Play a BGM cue by name ("rtown0", "TD_desktop", "btl0"...).
    pub fn play_bgm_named(&mut self, name: &str) {
        if let Some(&i) = self.bgm_cues.by_name.get(name) {
            self.play_bgm(i);
        }
    }

    /// Cue index of a cue name.
    pub fn cue_index(&self, name: &str) -> Option<u32> {
        self.bgm_cues.by_name.get(name).copied()
    }

    /// The cue currently playing (not fading out).
    pub fn current_bgm(&self) -> Option<u32> {
        if self.bgm_fade.is_some() { None } else { self.current_bgm_track }
    }

    /// Fade the music out over `secs`, then stop.
    pub fn fade_out_bgm(&mut self, secs: f32) {
        if self.current_bgm_track.is_some() && self.bgm_fade.is_none() {
            self.bgm_fade = Some((secs.max(0.01), secs.max(0.01)));
        }
    }

    /// Advance a fade-out (call once per frame).
    pub fn tick(&mut self, dt: f32) {
        if let Some((left, total)) = self.bgm_fade {
            let left = left - dt;
            if left <= 0.0 {
                self.stop_bgm();
            } else {
                self.bgm_fade = Some((left, total));
                if let Some(ch) = &self.bgm {
                    ch.player.set_volume(self.bgm_volume * left / total);
                }
            }
        }
    }

    /// Stop currently playing background music.
    pub fn stop_bgm(&mut self) {
        if let Some(ch) = &self.bgm {
            ch.stop();
        }
        self.current_bgm_track = None;
        self.bgm_fade = None;
    }

    /// Check if background music is actively playing (also applies the current BGM volume).
    pub fn is_bgm_playing(&mut self) -> bool {
        let Some(ch) = &self.bgm else { return false };
        if self.bgm_fade.is_none() {
            ch.player.set_volume(self.bgm_volume);
        }
        let playing = ch.is_playing();
        if !playing {
            self.current_bgm_track = None;
        }
        playing
    }

    /// Play character event voice dialogue cue.
    pub fn play_voice_cue(&mut self, cue_index: u32) {
        alog(&format!("voice cue {}", cue_index));
        self.stop_voice();
        let (Some(awb), Some(ch)) = (&self.voice_awb, &self.voice) else { return };
        let Some(hca) = awb.get_hca_bytes(cue_index as usize) else { return };
        ch.play(hca, self.voice_volume, false);
    }

    /// Play dialogue voice for a specific cutscene message row ID (e.g. "PM10100", row_id).
    pub fn play_cutscene_voice(&mut self, script_id: &str, row_id: u32) {
        let cue = match script_id {
            // In PM10100, Row N maps directly to M101_{N-1:03d} (cue index N - 1)
            "PM10100" => row_id.checked_sub(1),
            "PM10105" | "PM10135" => Some(row_id),
            _ => None,
        };
        if let Some(cue_idx) = cue {
            self.play_voice_cue(cue_idx);
        }
    }

    /// Stop current voice line.
    pub fn stop_voice(&mut self) {
        if let Some(ch) = &self.voice {
            ch.stop();
        }
    }

    /// Check if voice line is still speaking.
    pub fn is_voice_playing(&mut self) -> bool {
        self.voice.as_ref().map_or(false, |c| c.is_playing())
    }
}

impl Drop for SoundSystem {
    fn drop(&mut self) {
        self.stop_bgm();
        self.stop_voice();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_awb_header_parsing() {
        let bgm_path = crate::gamedir::game_path("cmn/data/data/sound/PC/bgm.awb");
        if bgm_path.exists() {
            let awb = AwbArchive::open(bgm_path).expect("failed to open bgm.awb");
            assert!(awb.count > 100);
            assert_eq!(awb.align, 32);
            assert!(awb.offsets.len() > awb.count);
            
            // Track 3 is Mac Anu plaza BGM
            let track3 = awb.get_hca_bytes(3).expect("track 3 should exist");
            assert!(track3.len() > 1_000_000);
            assert_eq!(&track3[0..3], b"HCA");
        }
    }

    #[test]
    fn test_voice_cue_extraction() {
        let voice_path = crate::gamedir::game_path("cmn/data/data/sound/PC/US/v_evVoiceVol1.awb");
        if voice_path.exists() {
            let awb = AwbArchive::open(voice_path).expect("failed to open voice awb");
            assert_eq!(awb.count, 4400);

            // Cue 0 is Haseo's opening line "...waa..."
            let voice0 = awb.get_hca_bytes(0).expect("cue 0 should exist");
            assert!(voice0.len() > 1000);
            assert_eq!(&voice0[0..3], b"HCA");
        }
    }
}
