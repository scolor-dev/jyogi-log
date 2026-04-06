pub fn validate_username(username: &str) -> Option<&'static str> {
    let s = username.trim();
    if s.len() < 3 || s.len() > 32 {
        return Some("username must be between 3 and 32 characters");
    }
    if !s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Some("username can only contain alphanumeric characters and underscores");
    }
    None
}

pub fn validate_password(password: &str) -> Option<&'static str> {
    if password.len() < 8 || password.len() > 72 {
        return Some("password must be between 8 and 72 characters");
    }
    if !password.chars().all(|c| c.is_ascii_graphic()) {
        return Some("password must only contain printable ASCII characters");
    }

    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_symbol = password.chars().any(|c| c.is_ascii_punctuation());

    let count = [has_upper, has_lower, has_digit, has_symbol]
        .iter()
        .filter(|&&x| x)
        .count();

    if count < 3 {
        return Some("password must contain at least 3 of: uppercase, lowercase, digits, symbols");
    }

    None
}

pub fn validate_display_name(display_name: &str) -> Option<&'static str> {
    let s = display_name.trim();
    if s.is_empty() || s.len() > 64 {
        return Some("display_name must be between 1 and 64 characters");
    }
    None
}