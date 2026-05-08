use crate::core::error::AppError;
use crate::core::providers::Provider;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub struct EmailProvider {
    smtp_host: String,
    smtp_port: u16,
    smtp_user: String,
    smtp_pass: String,
}

impl EmailProvider {
    pub fn new(smtp_host: String, smtp_port: u16, smtp_user: String, smtp_pass: String) -> Self {
        Self {
            smtp_host,
            smtp_port,
            smtp_user,
            smtp_pass,
        }
    }
}

impl Provider for EmailProvider {
    async fn send_notification(&self, to: &str, title: &str, body: &str) -> Result<(), AppError> {
        let email = Message::builder()
            .from("No Reply <noreply@example.com>".parse().unwrap())
            .to(to
                .parse()
                .map_err(|_| AppError::ValidationError("Invalid email".into()))?)
            .subject(title)
            .body(body.to_string())
            .map_err(|e| AppError::InternalServerError(format!("Email build error: {}", e)))?;

        let creds = Credentials::new(self.smtp_user.clone(), self.smtp_pass.clone());

        let mailer = SmtpTransport::relay(&self.smtp_host)
            .map_err(|e| AppError::InternalServerError(format!("SMTP relay error: {}", e)))?
            .credentials(creds)
            .port(self.smtp_port)
            .build();

        // Using spawn_blocking because SmtpTransport::send is blocking
        let _res = tokio::task::spawn_blocking(move || mailer.send(&email))
            .await
            .map_err(|e| AppError::InternalServerError(format!("Email task join error: {}", e)))?
            .map_err(|e| AppError::InternalServerError(format!("Email send error: {}", e)))?;

        Ok(())
    }

    async fn send_otp(&self, to: &str, otp: &str) -> Result<(), AppError> {
        self.send_notification(to, "Your OTP", &format!("Your OTP is: {}", otp))
            .await
    }
}
