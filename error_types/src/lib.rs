pub use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        FormError { form_values: (field_name, field_value), date: now, err }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new("password", self.password.clone(), "Password should be at least 8 characters long"));
        }

    let has_letter = self.password.chars().any(char::is_alphabetic);
    let has_number = self.password.chars().any(char::is_numeric);
        let has_symbol = self.password.chars().any(|c| !c.is_alphanumeric());

        if !(has_letter && has_number && has_symbol) {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"))
        }

        Ok(())
    }
}
