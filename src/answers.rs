#[derive(Clone, PartialEq, Eq)]
pub enum Choices {
    A,
    B,
    C,
    D
}

/// Converts to choices enum
pub fn to_choices_enum(s: &str) -> Result<Choices, String> {
    let ss = &s.to_lowercase() as &str;
    match ss.trim() {
        "a" => Ok(Choices::A),
        "b" => Ok(Choices::B),
        "c" => Ok(Choices::C),
        "d" => Ok(Choices::D),
        _ => Err("Conversion to choices enum failed".to_string())
    }
}
