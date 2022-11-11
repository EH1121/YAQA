#[derive(Clone, PartialEq, Eq)]
pub enum Choices {
    A,
    B,
    C,
    D
}


/// Converts to choices enum
pub fn to_choices_enum(s: &str) -> Result<Choices, ()> {
    let ss = &s.to_lowercase() as &str;
    match ss {
        "a" => Ok(Choices::A),
        "b" => Ok(Choices::B),
        "c" => Ok(Choices::C),
        "d" => Ok(Choices::D),
        _ => Err(())
    }
}

/// Converts back to str
pub fn from_choices_enum(s: Choices) -> String {
    match s {
        Choices::A => "a".to_string(),
        Choices::B => "b".to_string(),
        Choices::C => "c".to_string(),
        Choices::D => "d".to_string(),
    }
}