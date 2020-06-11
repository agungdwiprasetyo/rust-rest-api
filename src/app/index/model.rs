use serde::Serialize;

#[derive(Serialize)]
pub struct IndexResponse {
    pub message: String,
}

impl IndexResponse {
    fn new(message: String) -> Self {
        IndexResponse{
            message: message,
        }
    }
}

pub fn construct(message: String) -> IndexResponse {
    let output = IndexResponse::new(message);
    output
}