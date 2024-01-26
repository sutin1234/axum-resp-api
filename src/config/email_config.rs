use std::env;

pub async fn init_email_config() {
    env::var("SMTP_USERNAME").expect("SMTP_USERNAME not specified ");
    env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not specified ");
    env::var("SMTP_HOST").expect("SMTP_HOST Username not specified ");
}
