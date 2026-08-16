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
    let user_email = form.email.clone();
    let user_password = form.password.clone();

    let request_span = tracing::info_span!(
        "ADDING A NEW USER--->",
        user_id = %user_id,
        user_email = %user_email,
        user_password = %user_password,
    );
    let _request_span_guard = request_span.enter();

    tracing::info!("[ADDING A NEW USER - START]--->");

    // Для запроса используем отдельный Span без enter()
    let query_span = tracing::info_span!(
        "SAVING NEW USER DETAILS IN THE DATABASE--->",
        user_id = %user_id,
        user_email = %user_email,
        user_password = %user_password,
    );

    // Логируем старт внутри Span через событие
    tracing::info!("[SAVING NEW USER DETAILS IN THE DATABASE - START]--->");

    match sqlx::query!(
        "INSERT INTO users (id, email, password_hash) VALUES($1, $2, $3) RETURNING id",
        user_id,
        user_email,
        user_password,
    )
    .fetch_one(pool.get_ref())
    .instrument(query_span) // <-- перемещаем Span, а не заимствуем
    .await
    {
        Ok(_) => {
            tracing::info!("[<---SAVING NEW USER DETAILS IN THE DATABASE - END]");
            tracing::info!("[<---ADDING A NEW USER - EVENT] User created successfully");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
