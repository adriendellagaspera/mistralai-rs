impl AudioEncoding {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PcmS16le => "pcm_s16le",
            Self::PcmS32le => "pcm_s32le",
            Self::PcmF16le => "pcm_f16le",
            Self::PcmF32le => "pcm_f32le",
            Self::PcmMulaw => "pcm_mulaw",
            Self::PcmAlaw => "pcm_alaw",
        }
    }
}
