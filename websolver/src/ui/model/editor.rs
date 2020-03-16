#[derive(Debug, Copy, Clone)]
pub enum EditorButtonAction {
    SetValue(u8),
    Solve,
    Erase,
    Clear,
    First,
    Prev,
    Next,
    Last,
}

impl std::fmt::Display for EditorButtonAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EditorButtonAction::SetValue(n) => {
                if *n == 0 {
                    "_".to_string()
                } else {
                    format!("{}", *n)
                }
            }
            EditorButtonAction::Solve => "Solve".to_string(),
            EditorButtonAction::Erase => "Erase".to_string(),
            EditorButtonAction::Clear => "Clear".to_string(),
            EditorButtonAction::First => "<<".to_string(),
            EditorButtonAction::Prev => "<".to_string(),
            EditorButtonAction::Next => ">".to_string(),
            EditorButtonAction::Last => ">>".to_string(),
        };
        write!(f, "{}", s)
    }
}
