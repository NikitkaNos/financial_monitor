use actix_web::{HttpResponse, web};
//use chrono::Utc;

pub async fn transaction(_form: web::Form<TransactionData>) -> HttpResponse {
    // Возвращаем 200 ВСЕГДА, даже не смотрим на данные
    HttpResponse::Ok().finish()
}

// pub async fn transaction(
//     form: web::Form<TransactionData>,
//     pool: web::Data<PgPool>,
// ) -> HttpResponse {
//     if form.amount <= 0 || form.category.is_empty() || form.type_.is_empty() {
//         return HttpResponse::BadRequest().finish();
//     }

//     let user_id = uuid::Uuid::new_v4();

//     let result = sqlx::query!(
//         "INSERT INTO transactions (user_id, amount, category, type, date, description)
//          VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
//         user_id,
//         form.amount,
//         form.category,
//         form.type_,
//         chrono::Utc::now().date_naive(),
//         form.description,
//     )
//     .fetch_one(pool.get_ref())
//     .await;

//     match result {
//         Ok(_) => HttpResponse::Ok().finish(),
//         Err(e) => {
//             eprintln!("DB Error: {:?}", e);
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }

#[derive(serde::Deserialize)]
pub struct TransactionData {
    pub amount: i64,
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: Option<String>, // добавил, чтобы совпадало с INSERT
}
