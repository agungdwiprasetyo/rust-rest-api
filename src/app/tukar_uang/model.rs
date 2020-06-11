use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct TukarUangRequest {
    pub pecahan_koin_tersedia: Vec<usize>,
    pub uang: usize,
}

#[derive(Serialize)]
pub struct TukarUangResponse {
    pub total_koin_minimal: usize,
}

impl TukarUangResponse {
    pub fn new(total: usize) -> Self {
        TukarUangResponse {
            total_koin_minimal: total,
        }
    }
}
