use solver::Cell;

#[derive(Debug)]
pub struct EditorState {
    disabled: bool,
    mode: EditorMode,
    drag_select: Option<Cell>,
    set_sum: Option<Cell>
}

impl EditorState {
    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    pub fn disabled(&self) -> bool {
        self.disabled
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    pub fn set_drag(&mut self, cell: Option<Cell>) {
        self.drag_select = cell;
    }

    pub fn drag(&self) -> Option<Cell> {
        self.drag_select
    }

    pub fn set_sum_target(&mut self, cell: Option<Cell>) {
        self.set_sum = cell;
    }

    pub fn sum_target(&self) -> Option<Cell> {
        self.set_sum
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            disabled: false,
            mode: EditorMode::Default,
            drag_select: None,
            set_sum: None
        }
    }
}


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
    SetMode(EditorMode),
    Dragged(Cell),
    Clicked(Cell),
    CageSum(Cell),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Default,
    Cages,
}

impl Default for EditorMode {
    fn default() -> Self {
        Self::Default
    }
}