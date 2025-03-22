use slint::VecModel;
use std::error::Error;
use std::rc::Rc;
slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let todo_model = Rc::new(VecModel::from(vec![]));
    ui.set_todo_model(todo_model.clone().into());

    let todo_model_adding = todo_model.clone();
    // Step 1: Add basic necessary ui in app-window.slint
    // Step 2: using todo_model_adding, a reference-counting (RC) pointer of todo_model,
    // declare the code the add_item callback must execute using ui.on_add_item

    let todo_model_clearing = todo_model.clone();
    // Step 3: using todo_model_clearing declare the code the clear_all callback must execute using ui.on_clear_all

    ui.run()?;

    Ok(())
}
