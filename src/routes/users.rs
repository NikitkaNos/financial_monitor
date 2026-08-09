use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SingupData {
    pub email: String,
    pub password: String,
}

pub async fn signup(form: web::Json<SingupData>, pool: web::Data<PgPool>) -> HttpResponse {
    let user_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new user",
        %user_id,
        user_email = %form.email,
        user_password = %form.password,
    );
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new user details in the DataBase");

    let result = sqlx::query!(
        "INSERT INTO users (id, email, password_hash) VALUES($1, $2, $3) RETURNING id",
        user_id,
        form.email,
        form.password,
    )
    .fetch_one(pool.get_ref())
    .instrument(query_span)
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
