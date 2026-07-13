#[allow(unused_imports)]
use crate::configuration::get_configuration;
use crate::run;
use actix_web::HttpResponse;
use sqlx::PgPool;
//use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;

async fn _spawn_app() -> String {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to DB");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, pool).await.expect("Failed to start server");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
#[tokio::test]
async fn health_check_works() {
    let address = _spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
#[tokio::test]
async fn transaction_returns_a_200_for_valid_form_data() {
    let app_address = _spawn_app().await;
    let client = reqwest::Client::new();

    let body = "amount=100&category=Food&type=expense";
    let response = client
        .post(&format!("{}/transactions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(200, response.status().as_u16());
}
// #[tokio::test]
// async fn transaction_returns_a_200_for_valid_form_data() {
//     // Arrange
//     let app_address = spawn_app().await;
//     let configuration = get_configuration().expect("Failed to read configuration_ofc");
//     let connection_string = configuration.database.connection_string();

//     let mut connection = PgConnection::connect(&connection_string)
//         .await
//         .expect("Failed to connect Postgres");
//     let client = reqwest::Client::new();

//     //Act
//     let body = "amount=100&category=Food&type=expense";
//     let response = client
//         .post(&format!("{}/transactions", &app_address))
//         .header("Content-Type", "application/x-www-form-urlencoded")
//         .body(body)
//         .send()
//         .await
//         .expect("Failed to expect request.");

//     //Asseert2
//     assert_eq!(200, response.status().as_u16());
//     let saved = sqlx::query!("SELECT id, amount, user_id FROM transactions")
//         .fetch_optional(&mut connection)
//         .await
//         .expect("Failed to fetch saved transaction.");

//     assert!(saved.is_some());
//     let record = saved.unwrap();
//     assert_eq!(record.amount, 100);
// }
#[tokio::test]
async fn transaction_returns_a_400_when_data_is_missing() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    //Arrange
    let app_address = _spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "Missing the email"),
        ("email=ursula_le_guin%40gmail.com", "Missing the name"),
        ("", "Missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        //Act
        let response = client
            .post(&format!("{}/transactions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was ---> {}.",
            error_message
        );
    }
    Ok(())
}
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
