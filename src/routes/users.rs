use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SingupData {
    pub email: String,
    pub password: String,
}

pub async fn signup(form: web::Json<SingupData>, pool: web::Data<PgPool>) -> HttpResponse {
    let user_id = Uuid::new_v4();

    let result = sqlx::query!(
        "INSERT INTO users (id, email, password_hash) VALUES($1, $2, $3) RETURNING id",
        user_id,
        form.email,
        form.password,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            println!("✅ User created: {:?}", record.id);
            HttpResponse::Created().json(record.id)
        }
        Err(e) => {
            eprintln!("❌ DB Error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
