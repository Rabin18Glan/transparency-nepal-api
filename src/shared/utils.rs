use rand::{thread_rng, Rng};

pub fn generate_otp() -> String {
    if cfg!(debug_assertions) {
        return "123456".to_string();
    }
    thread_rng().gen_range(100_000..999_999).to_string()
}

pub fn _format_phone_number(phone: &str) -> String {
    phone.trim().to_string()
}
