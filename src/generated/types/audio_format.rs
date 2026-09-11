#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AudioFormat {
    pub encoding: AudioEncoding,
    ///Constraint: minimum=8000, maximum=96000
    pub sample_rate: i64,
}
