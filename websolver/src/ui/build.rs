#![cfg(feature = "webui")]


use webelements::Result;

use crate::ui::{
    app::AppController,
    editor::EditorController,
    info::InfoController,
    sudoku::{SudokuController, SudokuStateModel},
    {controllers, models, SudokuInfo},
};

use solver::Sudoku;

pub fn build_ui() -> Result<()> {
    let app = controllers().get::<AppController>("app")?;
    let sudoku = controllers().get::<SudokuController>("sudoku")?;
    let editor = controllers().get::<EditorController>("editor")?;
    let info = controllers().get::<InfoController>("info")?;
    if let Some(element) = &app.element() {
        app.
        element.sdk().build(&sudoku)?;
        element.main().build(&editor)?;
        element.main().build(&info)?;
        element.set_as_body();
    }
    Ok(())
}

pub fn init_ui() -> Result<()> {
    models()
        .init::<SudokuStateModel>("sudoku")
        .set_start(Sudoku::from(
            // "...6..8....35.4...65..217...6..............5..7138..2...7.1.6.4.1.......9....3..7",
            "....3.76.5....91.29.........49..53.......327...52..........75.4..1.4.....6.......",
        ));
    models().init::<SudokuInfo>("info").update_properties();
    controllers().build::<AppController>("app")?;
    controllers().build::<SudokuController>("sudoku")?;
    controllers().build::<EditorController>("editor")?;
    controllers().build::<InfoController>("info")?;
    Ok(())
}
