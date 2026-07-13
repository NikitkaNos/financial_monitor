use actix_web::{HttpResponse, web};

pub async fn transaction(form: web::Form<TransactionData>) -> HttpResponse {
    if form.amount <= 0 || form.category.is_empty() || form.type_.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct TransactionData {
    pub amount: i64,
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
}
