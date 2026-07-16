//use actix_web::HttpResponse;
// use actix_web::web;
// use sqlx::PgPool;
// //use chrono::Utc;

//TODO The function needs a complete redesign
//pub async fn transaction(// form: web::Form<TransactionData>,
// pool: web::Data<PgPool>,
//) -> HttpResponse {
// let user_id = uuid::Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();

// let result = sqlx::query!(
//     "INSERT INTO transactions (user_id, amount, category, type, date, description)
//     VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
//     user_id,
//     form.category,
//     form.type_,
//     chrono::Utc::now().date_naive(),
//     form.description,
// )
// .fetch_one(pool.get_ref())
// .await;

// match result {
//     Ok(record) => {
//         println!("✅ Inserted transaction with id: {:?}", record.id);
//         HttpResponse::Ok().finish()
//     }
//     Err(e) => {
//         eprintln!("❌ DB Error: {:?}", e);
//         HttpResponse::InternalServerError().finish()
//     }
// }
// HttpResponse::Ok().finish() //Возвращаем 200 ВСЕГДА, даже не смотрим на данные
//}
//
// pub async fn transaction(
//     form: web::Form<TransactionData>,
//     pool: web::Data<PgPool>,
// ) -> HttpResponse {
//     if form.amount <= 0 || form.category.is_empty() || form.type_.is_empty() {
//         return HttpResponse::BadRequest().finish();
//     }

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
