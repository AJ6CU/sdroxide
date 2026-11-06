//! What every radiosonde model decoder has in common, so a lane can run any
//! number of models over the same channel and the track book does not care
//! which one heard a frame.
//!
//! A model decoder turns wideband I/Q into [`SondeFix`]es: the model's own
//! telemetry reduced to the fields the operator sees (serial, frame counter,
//! battery, position). Model-specific detail (RS41 sub-block CRCs, PTU
//! calibration…) stays inside the model's own module.

use sdroxide_dsp::Complex32;
use xng_mode_sonde::{Decoded, GpsPos, SondeChannelDecoder};

/// A radiosonde model this build can decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SondeModel {
    /// Vaisala RS41-SG / -SGP.
    Rs41,
    /// Graw DFM-06 / DFM-09 / DFM-17.
    Dfm,
    /// Meteomodem M10.
    M10,
    /// Meteomodem M20.
    M20,
    /// InterMet iMet-4.
    Imet4,
    /// Meteo-Radiy MRZ.
    Mrz,
}

impl SondeModel {
    /// Every model decoded, in the order a lane tries them.
    pub const ALL: &'static [SondeModel] = &[SondeModel::Rs41, SondeModel::Dfm, SondeModel::M10, SondeModel::M20, SondeModel::Imet4, SondeModel::Mrz];

    /// The type as printed in the track table and sent to SondeHub.
    pub fn name(self) -> &'static str {
        match self {
            SondeModel::Rs41 => "RS41",
            SondeModel::Dfm => "DFM",
            SondeModel::M10 => "M10",
            SondeModel::M20 => "M20",
            SondeModel::Imet4 => "iMet",
            SondeModel::Mrz => "MRZ",
        }
    }
}

/// One telemetry frame, reduced to what every model can tell.
#[derive(Debug, Clone, PartialEq)]
pub struct SondeFix {
    pub model: SondeModel,
    /// The serial number printed on the sonde. Empty when the model's frame
    /// carries none that is read: the track book then names it by channel.
    pub serial: String,
    /// The sonde's own frame counter.
    pub frame_num: u32,
    /// Battery voltage, V, when the model sends it.
    pub battery_v: Option<f32>,
    /// GPS position and velocity, when the frame carries a valid one.
    pub pos: Option<GpsPos>,
}

impl From<&Decoded> for SondeFix {
    fn from(d: &Decoded) -> SondeFix {
        SondeFix {
            model: SondeModel::Rs41,
            serial: d.frame.serial.clone(),
            frame_num: u32::from(d.frame.frame_num),
            battery_v: Some(d.frame.battery_v),
            pos: d.frame.gps_pos.clone(),
        }
    }
}

/// A decoder for one sonde model, fed the lane's I/Q.
pub trait ModelDecoder: Send {
    fn model(&self) -> SondeModel;
    /// Feed I/Q; returns every frame that decoded.
    fn process(&mut self, input: &[Complex32]) -> Vec<SondeFix>;
    /// Smoothed channel power, dBFS.
    fn level_dbfs(&self) -> f32;
}

impl ModelDecoder for SondeChannelDecoder {
    fn model(&self) -> SondeModel {
        SondeModel::Rs41
    }

    fn process(&mut self, input: &[Complex32]) -> Vec<SondeFix> {
        SondeChannelDecoder::process(self, input).iter().map(SondeFix::from).collect()
    }

    fn level_dbfs(&self) -> f32 {
        SondeChannelDecoder::level_dbfs(self)
    }
}

/// A decoder for `model` on a channel `freq_offset_hz` from the capture centre.
pub fn decoder_for(
    model: SondeModel,
    input_rate: f64,
    freq_offset_hz: f64,
) -> Result<Box<dyn ModelDecoder>, String> {
    match model {
        SondeModel::Rs41 => Ok(Box::new(SondeChannelDecoder::new(input_rate, freq_offset_hz)?)),
        SondeModel::Dfm => Ok(Box::new(crate::dfm::DfmDecoder::new(input_rate, freq_offset_hz)?)),
        SondeModel::M10 => Ok(Box::new(crate::m10::M10Decoder::new(input_rate, freq_offset_hz)?)),
        SondeModel::M20 => Ok(Box::new(crate::m20::M20Decoder::new(input_rate, freq_offset_hz)?)),
        SondeModel::Imet4 => Ok(Box::new(crate::imet::Imet4Decoder::new(input_rate, freq_offset_hz)?)),
        SondeModel::Mrz => Ok(Box::new(crate::mrz::MrzDecoder::new(input_rate, freq_offset_hz)?)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_model_has_a_name_and_a_decoder() {
        for &m in SondeModel::ALL {
            assert!(!m.name().is_empty());
            let d = decoder_for(m, xng_mode_sonde::CHANNEL_RATE, 0.0).expect("decoder");
            assert_eq!(d.model(), m);
        }
    }

    #[test]
    fn model_names_are_distinct() {
        let mut n: Vec<_> = SondeModel::ALL.iter().map(|m| m.name()).collect();
        n.sort_unstable();
        n.dedup();
        assert_eq!(n.len(), SondeModel::ALL.len());
    }
}
