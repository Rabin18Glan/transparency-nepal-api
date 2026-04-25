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
            Some(stored_otp) if stored_otp == otp => Ok(VerifyOtpResponse {
                verified: true,
                message: "OTP verified successfully".to_string(),
            }),
            Some(_) => Err(AppError::Unauthorized),
            None => Err(AppError::NotFound("OTP expired or not found".to_string())),
        }
    }
}
