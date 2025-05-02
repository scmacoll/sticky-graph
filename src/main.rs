use eframe::{egui, App, Frame, NativeOptions};

/// Our domain state
#[derive(Default)]
struct Model {
    input: String,
    notes: Vec<String>,
}

/// Messages describing user intent
enum Msg {
    SetInput(String),
    AddNote,
    Delete(usize),
}

/// Pure “update” function
fn update(model: &mut Model, msg: Msg) {
    match msg {
        Msg::SetInput(s) => model.input = s,
        Msg::AddNote => {
            let txt = model.input.trim();
            if !txt.is_empty() {
                model.notes.push(txt.to_string());
                model.input.clear();
            }
        }
        Msg::Delete(idx) => {
            // remove the note if the index is valid
            if idx < model.notes.len() {
                model.notes.remove(idx);
            }
        }
    }
}

/// Pure-ish “view” function; no closures buried inside it
/// Pure-ish “view” function; no borrow conflicts and flat layout
fn view(ui: &mut egui::Ui, model: &mut Model) {
    // 1) Input row
    ui.horizontal(|ui| {
        // Clone out the current input so we only update via Msg
        let mut new_input = model.input.clone();

        // Text box — if the user types, we'll get .changed() == true
        let text_edit = ui.text_edit_singleline(&mut new_input);
        if text_edit.changed() {
            // commit the edit into our model
            update(model, Msg::SetInput(new_input.clone()));
        }

        // Add button
        if ui.button("Add").clicked() {
            update(model, Msg::AddNote);
        }
    });

    ui.separator();

    // 2) Notes list + collect delete index
    let mut to_delete: Option<usize> = None;
    for (i, note) in model.notes.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(note);
            // A smaller delete button
            if ui.small_button("×").clicked() {
                to_delete = Some(i);
            }
        });
    }

    // 3) Now that the .iter() borrow is done, actually delete
    if let Some(i) = to_delete {
        update(model, Msg::Delete(i));
    }
}

/// The egui “App” just wires `view` into the egui loop
struct StickyApp {
    model: Model,
}

impl Default for StickyApp {
    fn default() -> Self {
        Self {
            model: Model::default(),
        }
    }
}

impl App for StickyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            view(ui, &mut self.model);
        });
    }
}

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "Gleam-ish Sticky Notes with Delete",
        options,
        Box::new(|_cc| Box::new(StickyApp::default())),
    );
}
