//! `obs_volmeter_t`. Opaque in the public header.

use crate::MAX_AUDIO_CHANNELS;

/// `obs_volmeter_updated_t`, with the `void *` param carried as `usize`.
///
/// The three arrays are magnitude, peak, and input peak, in dB, one slot
/// per channel up to [`MAX_AUDIO_CHANNELS`].
pub type MeterCb = fn(
    param: usize,
    magnitude: &[f32; MAX_AUDIO_CHANNELS],
    peak: &[f32; MAX_AUDIO_CHANNELS],
    input_peak: &[f32; MAX_AUDIO_CHANNELS],
);

/// One capture callback's view of `audio_data`, plus the two mute flags
/// the C callback closes over.
///
/// `aligned[i] == false` is the unaligned-plane path: that plane's peak is
/// forced to 1 and its peak history is left alone. Magnitude still runs.
pub struct AudioPacket<'a> {
    pub planes: [Option<&'a [f32]>; MAX_AUDIO_CHANNELS],
    pub frames: usize,
    pub speakers: i32,
    pub aligned: [bool; MAX_AUDIO_CHANNELS],
    pub capture_muted: bool,
    pub user_muted: bool,
}

/// Level meter. The fader type is stored and never read.
pub struct Volmeter {
    _private: (),
}

impl Volmeter {
    /// `obs_volmeter_create`. Any type, including an unknown one, creates.
    pub fn create(ty: i32) -> Self {
        let _ = ty;
        todo!()
    }

    /// `obs_volmeter_attach_source`. Copies `volume` into the current dB
    /// and forgets the previous source's speaker layout.
    pub fn attach_volume(&self, volume: f32) {
        let _ = volume;
        todo!()
    }

    /// `obs_volmeter_detach_source`. Channel count then reads as 0.
    pub fn detach(&self) {
        todo!()
    }

    /// The meter's `volume` signal handler. Updates the current dB only.
    /// Level callbacks are not run. Disconnect is the caller's job.
    pub fn on_volume(&self, mul: f32) {
        let _ = mul;
        todo!()
    }

    /// The meter's `destroy` signal handler. Detaches.
    pub fn on_destroy(&self) {
        todo!()
    }

    /// `obs_volmeter_set_peak_meter_type`. No null check in C.
    pub fn set_peak_meter_type(&self, ty: i32) {
        let _ = ty;
        todo!()
    }

    /// `obs_volmeter_get_nr_channels`.
    ///
    /// Speakers stay unknown until the first packet, so this is 0 after
    /// attach and before any audio. `mix_speakers` is the obs mix layout.
    /// `None` is `obs_get_audio_info` failing, which caps the count at 2.
    /// The count is clamped to that mix. After detach it is 0.
    pub fn nr_channels(&self, mix_speakers: Option<i32>) -> i32 {
        let _ = mix_speakers;
        todo!()
    }

    /// The capture callback: peaks, RMS magnitude, then level callbacks.
    ///
    /// `mul` is 0 when `capture_muted && !user_muted` (a disabled source).
    /// User mute sets both flags, so the meter is not zeroed. Input peak
    /// ignores that multiplier.
    pub fn on_data(&self, packet: &AudioPacket<'_>) {
        let _ = packet;
        todo!()
    }

    /// `obs_volmeter_add_callback`.
    pub fn add_callback(&self, cb: MeterCb, param: usize) {
        let _ = (cb, param);
        todo!()
    }

    /// `obs_volmeter_remove_callback`. One matching `(cb, param)` pair.
    pub fn remove_callback(&self, cb: MeterCb, param: usize) {
        let _ = (cb, param);
        todo!()
    }
}
