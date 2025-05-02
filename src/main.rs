use eframe::{egui, App, Frame, NativeOptions};

/// --- your existing Model & Msg & update stay exactly the same ---
#[derive(Default)]
struct Model {
    input: String,
    notes: Vec<String>,
    editing: Option<(usize, String)>,
}

enum Msg {
    SetInput(String),
    AddNote,
    Delete(usize),
    Edit(usize),
    SetEditInput(String),
    SaveEdit,
    CancelEdit,
}

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
            if idx < model.notes.len() {
                model.notes.remove(idx);
            }
            if let Some((eidx, _)) = model.editing {
                if eidx == idx {
                    model.editing = None;
                }
            }
        }
        Msg::Edit(idx) => {
            if idx < model.notes.len() {
                model.editing = Some((idx, model.notes[idx].clone()));
            }
        }
        Msg::SetEditInput(s) => {
            if let Some((_, buf)) = &mut model.editing {
                *buf = s;
            }
        }
        Msg::SaveEdit => {
            if let Some((idx, buf)) = model.editing.take() {
                let txt = buf.trim();
                if !txt.is_empty() && idx < model.notes.len() {
                    model.notes[idx] = txt.to_string();
                }
            }
        }
        Msg::CancelEdit => model.editing = None,
    }
}

/// --- replacement view fn ---
fn view(ui: &mut egui::Ui, model: &mut Model) {
    // 1) Input row
    ui.horizontal(|ui| {
        let mut new_input = model.input.clone();
        if ui.text_edit_singleline(&mut new_input).changed() {
            model.input = new_input;
        }
        if ui.button("Add").clicked() {
            update(model, Msg::AddNote);
        }
    });

    ui.separator();

    // 2) Record actions here (no mutation of model inside the loop)
    let mut to_delete = None;
    let mut to_edit = None;
    let mut to_save = false;
    let mut to_cancel = false;
    let mut new_edit_buf = None;

    for (i, note) in model.notes.iter().enumerate() {
        ui.horizontal(|ui| {
            // If this note is in edit‐mode, show the edit UI
            if let Some((edit_idx, existing_buf)) = &model.editing {
                if *edit_idx == i {
                    // work on a local copy of the buffer
                    let mut buf = existing_buf.clone();

                    if ui.text_edit_singleline(&mut buf).changed() {
                        new_edit_buf = Some(buf);
                    }
                    if ui.small_button("💾").clicked() {
                        to_save = true;
                    }
                    if ui.small_button("✖").clicked() {
                        to_cancel = true;
                    }
                    return; // skip the display‐mode buttons
                }
            }

            // Normal display mode
            ui.label(note);
            if ui.small_button("✏️").clicked() {
                to_edit = Some(i);
            }
            if ui.small_button("×").clicked() {
                to_delete = Some(i);
            }
        });
    }

    // 3) Now that all borrows are gone, perform the updates
    if let Some(buf) = new_edit_buf {
        update(model, Msg::SetEditInput(buf));
    }
    if let Some(idx) = to_edit {
        update(model, Msg::Edit(idx));
    }
    if to_save {
        update(model, Msg::SaveEdit);
    }
    if to_cancel {
        update(model, Msg::CancelEdit);
    }
    if let Some(idx) = to_delete {
        update(model, Msg::Delete(idx));
    }
}

/// --- small change here to use the imported `App` and `Frame` types ---
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

fn main() -> eframe::Result<()> {
    let opts = NativeOptions::default();
    eframe::run_native(
        "Sticky Notes – Inline Edit",
        opts,
        Box::new(|_cc| Box::new(StickyApp::default())),
    )
}
