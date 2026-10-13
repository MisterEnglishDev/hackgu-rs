//! hcadump <awb> <cue> <out.wav> : decode one cue with cridecoder and write a 16-bit WAV (for checking the decoder)
use std::io::{Read, Seek, SeekFrom, Write};
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let mut f = std::fs::File::open(&a[1]).unwrap();
    let mut h = [0u8; 16]; f.read_exact(&mut h).unwrap();
    let count = u32::from_le_bytes(h[8..12].try_into().unwrap()) as usize;
    let align = u32::from_le_bytes(h[12..16].try_into().unwrap()) as usize;
    let cue: usize = a[2].parse().unwrap();
    f.seek(SeekFrom::Start((16 + count * 2 + cue * 4) as u64)).unwrap();
    let mut o = [0u8; 8]; f.read_exact(&mut o).unwrap();
    let s = u32::from_le_bytes(o[0..4].try_into().unwrap()) as usize; let e = u32::from_le_bytes(o[4..8].try_into().unwrap()) as usize;
    let s = (s + align - 1) & !(align - 1);
    f.seek(SeekFrom::Start(s as u64)).unwrap();
    let mut d = vec![0u8; e - s]; f.read_exact(&mut d).unwrap();
    let mut dec = cridecoder::HcaDecoder::from_reader(std::io::Cursor::new(d)).unwrap();
    let info = dec.info().clone();
    let smp = dec.decode_all().unwrap();
    let (mn, mx) = smp.iter().fold((f32::MAX, f32::MIN), |(a, b), &x| (a.min(x), b.max(x)));
    println!("ch {} rate {} frames {} spb {} min {} max {} nan {}", info.channel_count, info.sampling_rate, smp.len() / info.channel_count as usize, info.samples_per_block, mn, mx, smp.iter().filter(|x| x.is_nan()).count());
    let mut w = std::fs::File::create(&a[3]).unwrap();
    let ch = info.channel_count as u16; let rate = info.sampling_rate; let n = smp.len() as u32;
    w.write_all(b"RIFF").unwrap(); w.write_all(&(36 + n * 2).to_le_bytes()).unwrap(); w.write_all(b"WAVEfmt ").unwrap();
    w.write_all(&16u32.to_le_bytes()).unwrap(); w.write_all(&1u16.to_le_bytes()).unwrap(); w.write_all(&ch.to_le_bytes()).unwrap();
    w.write_all(&rate.to_le_bytes()).unwrap(); w.write_all(&(rate * ch as u32 * 2).to_le_bytes()).unwrap(); w.write_all(&(ch * 2).to_le_bytes()).unwrap(); w.write_all(&16u16.to_le_bytes()).unwrap();
    w.write_all(b"data").unwrap(); w.write_all(&(n * 2).to_le_bytes()).unwrap();
    for x in smp { w.write_all(&((x.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes()).unwrap(); }
}
