use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BookResponse {
    pub reservation_id: i64,
    pub resy_token: String,
}
