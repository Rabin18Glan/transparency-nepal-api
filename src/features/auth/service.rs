use crate::state::SharedState;
use crate::error::AppError;
use crate::common::utils;
use crate::config::providers::{SmsProvider, WhatsAppProvider};
use super::model::{OtpChannel, OtpResponse, VerifyOtpResponse};
use super::repo::AuthRepository;

pub struct AuthService {
    state: SharedState,
    repo: AuthRepository,
}

impl AuthService {
    pub fn new(state: SharedState) -> Self {
        let repo = AuthRepository::new(state.clone());
        Self { state, repo }
    }

    pub async fn request_otp(&self, phone_number: String, channel: OtpChannel) -> Result<OtpResponse, AppError> {
        tracing::info!("Processing OTP request for {} via {:?}", phone_number, channel);

        let otp = utils::generate_otp();
        self.repo.store_otp(&phone_number, &otp).await?;

        // Debug mode: Skip sending real SMS for dummy number or in debug mode
        if phone_number == "9800000000" || cfg!(debug_assertions) {
            tracing::info!("[DEBUG] SMS skipped for {} — OTP: {}", phone_number, otp);
            return Ok(OtpResponse {
                message: "OTP successfully sent (DEBUG)".to_string(),
                expires_in_seconds: 300,
            });
        }

        match channel {
            OtpChannel::Sms => {
                let config = &self.state.config;
                SmsProvider::new(config.sparrow_sms_token.clone(), config.sparrow_sms_sender.clone())
                    .send_otp(&phone_number, &otp)
                    .await?;
            }
            OtpChannel::Whatsapp => {
                let config = &self.state.config;
                WhatsAppProvider::new(config.whatsapp_access_token.clone(), config.whatsapp_phone_number_id.clone())
                    .send_otp(&phone_number, &otp)
                    .await?;
            }
        }

        Ok(OtpResponse {
            message: "OTP successfully sent".to_string(),
            expires_in_seconds: 300,
        })
    }

    pub async fn verify_otp(&self, phone_number: String, otp: String) -> Result<VerifyOtpResponse, AppError> {
        tracing::info!("Verifying OTP for {}", phone_number);

        let stored = self.repo.get_otp(&phone_number).await?;

        match stored {
            Some(stored_otp) if stored_otp == otp => {
                let token = self.state.paseto.create_token(&phone_number)?;
                Ok(VerifyOtpResponse {
                    verified: true,
                    message: "OTP verified successfully".to_string(),
                    token: Some(token),
                })
            }
            Some(_) => Err(AppError::Unauthorized),
            None => Err(AppError::NotFound("OTP expired or not found".to_string())),
        }
    }
}
