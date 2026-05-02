use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub server_port: u16,
    pub surreal_url: String,
    pub cache_url: String,
    pub sparrow_sms_token: String,
    pub sparrow_sms_sender: String,
    pub whatsapp_access_token: String,
    pub whatsapp_phone_number_id: String,
    pub paseto_secret: String,
    pub rust_log: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            server_port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("PORT must be a number"),
            surreal_url: env::var("SURREAL_URL").expect("SURREAL_URL must be set"),
            cache_url: env::var("CACHE_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into()),
            sparrow_sms_token: env::var("SPARROW_SMS_TOKEN")
                .expect("SPARROW_SMS_TOKEN must be set"),
            sparrow_sms_sender: env::var("SPARROW_SMS_SENDER")
                .expect("SPARROW_SMS_SENDER must be set"),
            whatsapp_access_token: env::var("WHATSAPP_ACCESS_TOKEN").unwrap_or_else(|_| "".into()),
            whatsapp_phone_number_id: env::var("WHATSAPP_PHONE_NUMBER_ID")
                .unwrap_or_else(|_| "".into()),
            paseto_secret: env::var("PASETO_SECRET").expect("PASETO_SECRET must be set"),
            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        }
    }
}
