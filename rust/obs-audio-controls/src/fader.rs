//! `obs_fader_t`. Opaque in the public header. The safe object is not `repr(C)`.

/// `obs_fader_changed_t`, with the `void *` param carried as `usize`.
pub type FaderCb = fn(param: usize, db: f32);

/// Cubic, IEC, or log fader. An unknown type is not created.
pub struct Fader {
    _private: (),
}

impl Fader {
    /// `obs_fader_create`. `None` for a type other than cubic, IEC, or log.
    pub fn create(ty: i32) -> Option<Self> {
        let _ = ty;
        todo!()
    }

    /// `obs_fader_get_db`.
    pub fn db(&self) -> f32 {
        todo!()
    }

    /// `obs_fader_get_deflection`.
    pub fn deflection(&self) -> f32 {
        todo!()
    }

    /// `obs_fader_get_mul`.
    pub fn mul(&self) -> f32 {
        todo!()
    }

    /// What `obs_fader_db_to_def` returns, applied to `db`.
    pub fn db_to_def(&self, db: f32) -> f32 {
        let _ = db;
        todo!()
    }

    /// `obs_fader_set_db`.
    ///
    /// Clamps to the curve's range. Below `min_db` the stored value is
    /// -infinity, not the floor. Arms `ignore_next_signal`. When a source
    /// volume is attached, that volume becomes `db_to_mul` of the stored
    /// dB. This does not run fader callbacks. `obs_source_set_volume`
    /// signals `volume`, which is [`Self::on_volume`] and clears the flag.
    pub fn set_db(&self, db: f32) -> bool {
        let _ = db;
        todo!()
    }

    /// `obs_fader_set_deflection`.
    pub fn set_deflection(&self, def: f32) -> bool {
        let _ = def;
        todo!()
    }

    /// `obs_fader_set_mul`.
    pub fn set_mul(&self, mul: f32) -> bool {
        let _ = mul;
        todo!()
    }

    /// `obs_fader_attach_source`: drop any current source, then copy `volume`
    /// into the current dB. `ignore_next_signal` stays as it was.
    pub fn attach_volume(&self, volume: f32) {
        let _ = volume;
        todo!()
    }

    /// `obs_fader_detach_source`. The current dB stays.
    pub fn detach(&self) {
        todo!()
    }

    /// The attached source volume, if `attach_volume` is in effect.
    pub fn attached_volume(&self) -> Option<f32> {
        todo!()
    }

    /// The fader's `volume` signal handler.
    ///
    /// A set ignore flag is cleared and the update is dropped. Otherwise
    /// the current dB becomes `mul_to_db(mul)` and callbacks run, last
    /// added first. The handler does not check that a source is still
    /// attached; disconnect is the caller's job.
    pub fn on_volume(&self, mul: f32) {
        let _ = mul;
        todo!()
    }

    /// The fader's `destroy` signal handler. Detaches.
    pub fn on_destroy(&self) {
        todo!()
    }

    /// `obs_fader_add_callback`.
    pub fn add_callback(&self, cb: FaderCb, param: usize) {
        let _ = (cb, param);
        todo!()
    }

    /// `obs_fader_remove_callback`. One matching `(cb, param)` pair.
    pub fn remove_callback(&self, cb: FaderCb, param: usize) {
        let _ = (cb, param);
        todo!()
    }
}
