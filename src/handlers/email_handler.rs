use crate::{api_response::ApiResponse, email::EmailPayload, error_response::ApiError};
use axum::Json;
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use serde_json::json;
use std::env;

pub async fn send_email_handler(// Json(payload): Json<EmailPayload>
) -> Result<ApiResponse, ApiError> {
    // let body = payload;

    let body = EmailPayload {
        fullname: "sutin injitt".parse().unwrap(),
        email: "poly.devb2018@gmail.com".parse().unwrap(),
        message: "test send email Axum".parse().unwrap(),
    };

    let from_address = "You <sutin17@hotmail.com>".parse().unwrap();
    let to_address = format!("{} <{}>", body.fullname, body.email)
        .parse()
        .unwrap();
    let reply_to = "You <sutin17@hotmail.com>".parse().unwrap();
    let email_subject = "Axum Rust tutorial";

    let email = Message::builder()
        .from(from_address)
        .reply_to(reply_to)
        .to(to_address)
        .subject(email_subject)
        .body(String::from(body.message))
        .unwrap();

    let creds = Credentials::new(
        env::var("SMTP_USERNAME").expect("SMTP Username not specified "),
        env::var("SMTP_PASSWORD").expect("SMTP Password not specified"),
    );

    let mailer = SmtpTransport::relay(&env::var("SMTP_HOST").expect("SMTP Host not specified"))
        .unwrap()
        .credentials(creds)
        .build();

    let response = match mailer.send(&email) {
        Ok(resp) => {
            println!("response: {:?}", resp);
            let json_data = json!({
                "message": "Email sent successfully!"
            });
            serde_json::from_value(json_data).unwrap()
        }
        Err(e) => panic!("Could not send email: {:?}", e),
    };

    Ok(ApiResponse::JsonData(response))
}
