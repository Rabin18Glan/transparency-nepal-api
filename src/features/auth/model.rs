use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OtpChannel {
    Sms,
    Whatsapp,
}

impl Default for OtpChannel {
    fn default() -> Self {
        Self::Sms
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct OtpRequest {
    #[validate(length(min = 10, max = 10, message = "Invalid Nepali phone number"))]
    pub phone_number: String,
    
    #[serde(default)]
    pub channel: OtpChannel,
}

#[derive(Debug, Deserialize, Validate)]
pub struct VerifyOtpRequest {
    #[validate(length(min = 10, max = 10, message = "Invalid Nepali phone number"))]
    pub phone_number: String,
    #[validate(length(min = 6, max = 6, message = "OTP must be 6 digits"))]
    pub otp: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyOtpResponse {
    pub verified: bool,
    pub message: String,
}
#[derive(Debug, Serialize)]
pub struct OtpResponse {
    pub message: String,
    pub expires_in_seconds: u64,
}
