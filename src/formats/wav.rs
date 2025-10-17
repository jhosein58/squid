use crate::error::{Result, SquidError};

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WavSpec {
    pub audio_format: u16,
    pub num_channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
}

pub struct Wav {
    pub spec: WavSpec,
    pub samples: Vec<f32>,
}

impl Wav {
    pub fn new(spec: WavSpec) -> Self {
        Self {
            spec,
            samples: Vec::new(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut header = [0u8; 44];
        reader.read_exact(&mut header)?;
        verify_header(&header)?;

        let spec = WavSpec::from_header(&header);
        let data_size = u32::from_le_bytes(header[40..44].try_into().unwrap());

        let mut data_bytes = vec![0; data_size as usize];
        reader.read_exact(&mut data_bytes)?;

        let samples = samples_from_bytes(&data_bytes, spec.bits_per_sample)?;

        Ok(Self {
            spec: spec,
            samples: samples,
        })
    }

    pub fn write_to_path<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        const BITS_PER_SAMPLE: u16 = 16;

        let data_bytes = bytes_from_samples(&self.samples, BITS_PER_SAMPLE)?;

        let spec = WavSpec {
            audio_format: 1,
            num_channels: self.spec.num_channels,
            sample_rate: self.spec.sample_rate,
            bits_per_sample: BITS_PER_SAMPLE,
        };

        let header = build_header(&spec, data_bytes.len() as u32);

        writer.write_all(&header)?;
        writer.write_all(&data_bytes)?;

        Ok(())
    }
}

fn verify_header(header: &[u8; 44]) -> Result<()> {
    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" || &header[12..16] != b"fmt " {
        return Err(SquidError::InvalidHeader("Invalid RIFF/WAVE format".into()));
    }
    let audio_format = u16::from_le_bytes(header[20..22].try_into().unwrap());
    if audio_format != 1 {
        return Err(SquidError::UnsupportedFormat(format!(
            "Only PCM (format=1) is supported, found {}",
            audio_format
        )));
    }
    if &header[36..40] != b"data" {
        return Err(SquidError::InvalidHeader(
            "Could not find 'data' chunk".into(),
        ));
    }
    Ok(())
}

impl WavSpec {
    fn from_header(header: &[u8; 44]) -> Self {
        Self {
            audio_format: 1,
            num_channels: u16::from_le_bytes(header[22..24].try_into().unwrap()),
            sample_rate: u32::from_le_bytes(header[24..28].try_into().unwrap()),
            bits_per_sample: u16::from_le_bytes(header[34..36].try_into().unwrap()),
        }
    }
}

fn build_header(spec: &WavSpec, data_size: u32) -> [u8; 44] {
    let mut header = [0u8; 44];
    let block_align = spec.num_channels * (spec.bits_per_sample / 8);
    let byte_rate = spec.sample_rate * block_align as u32;

    header[0..4].copy_from_slice(b"RIFF");
    header[4..8].copy_from_slice(&(36 + data_size).to_le_bytes());
    header[8..12].copy_from_slice(b"WAVE");
    header[12..16].copy_from_slice(b"fmt ");
    header[16..20].copy_from_slice(&16u32.to_le_bytes());
    header[20..22].copy_from_slice(&spec.audio_format.to_le_bytes());
    header[22..24].copy_from_slice(&spec.num_channels.to_le_bytes());
    header[24..28].copy_from_slice(&spec.sample_rate.to_le_bytes());
    header[28..32].copy_from_slice(&byte_rate.to_le_bytes());
    header[32..34].copy_from_slice(&block_align.to_le_bytes());
    header[34..36].copy_from_slice(&spec.bits_per_sample.to_le_bytes());
    header[36..40].copy_from_slice(b"data");
    header[40..44].copy_from_slice(&data_size.to_le_bytes());

    header
}

fn samples_from_bytes(data_bytes: &[u8], bits_per_sample: u16) -> Result<Vec<f32>> {
    let num_samples = data_bytes.len() / (bits_per_sample as usize / 8);
    let mut samples = Vec::with_capacity(num_samples);

    match bits_per_sample {
        8 => data_bytes
            .iter()
            .for_each(|&byte| samples.push((byte as f32 - 128.0) / 128.0)),
        16 => data_bytes.chunks_exact(2).for_each(|chunk| {
            let val = i16::from_le_bytes(chunk.try_into().unwrap());
            samples.push(val as f32 / i16::MAX as f32);
        }),
        24 => data_bytes.chunks_exact(3).for_each(|chunk| {
            let val = i32::from_le_bytes([
                chunk[0],
                chunk[1],
                chunk[2],
                if chunk[2] & 0x80 > 0 { 0xFF } else { 0 },
            ]);
            samples.push(val as f32 / 8_388_607.0);
        }),
        32 => data_bytes.chunks_exact(4).for_each(|chunk| {
            let val = i32::from_le_bytes(chunk.try_into().unwrap());
            samples.push(val as f32 / i32::MAX as f32);
        }),
        _ => {
            return Err(SquidError::UnsupportedFormat(format!(
                "{} bits per sample is not supported.",
                bits_per_sample
            )));
        }
    }
    Ok(samples)
}

fn bytes_from_samples(samples: &[f32], bits_per_sample: u16) -> Result<Vec<u8>> {
    let mut data_bytes = Vec::with_capacity(samples.len() * (bits_per_sample as usize / 8));
    match bits_per_sample {
        16 => {
            for &sample in samples {
                let val = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
                data_bytes.extend_from_slice(&val.to_le_bytes());
            }
        }
        _ => {
            return Err(SquidError::UnsupportedFormat(format!(
                "Writing {} bits per sample is not supported yet.",
                bits_per_sample
            )));
        }
    }
    Ok(data_bytes)
}
