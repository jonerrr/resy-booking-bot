use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DetailsResponse {
    pub book_token: BookToken,
    pub cancellation: Cancellation,
    pub user: User,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BookToken {
    pub value: String,
    // should be date
    pub date_expires: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Cancellation {
    pub fee: Option<Fee>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fee {
    pub amount: f32,
    // should be date
    pub date_cut_off: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub payment_methods: Option<Vec<PaymentMethod>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaymentMethod {
    pub id: i32,
    #[serde(rename = "type")]
    pub card_type: String,
    pub display: String,
}
