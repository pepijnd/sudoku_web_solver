#[derive(Debug, Copy, Clone)]
pub enum EditorAction {
    SetValue(u8),
    Solve,
    Erase,
    Clear,
    First,
    Prev,
    Next,
    Last,
    None,
}

impl Default for EditorAction {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for EditorAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EditorAction::SetValue(n) => {
                if *n == 0 {
                    "_".to_string()
                } else {
                    format!("{}", *n)
                }
            }
            EditorAction::Solve => "Solve".to_string(),
            EditorAction::Erase => "Erase".to_string(),
            EditorAction::Clear => "Clear".to_string(),
            EditorAction::First => "<<".to_string(),
            EditorAction::Prev => "<".to_string(),
            EditorAction::Next => ">".to_string(),
            EditorAction::Last => ">>".to_string(),

            _ => "N/A".to_string(),
        };
        write!(f, "{}", s)
    }
}
