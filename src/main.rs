use eframe::{egui, App, CreationContext, Frame, NativeOptions};
use egui::Ui;
use egui_dnd::Dnd;
use serde::{Deserialize, Serialize};

/// --- 1) Model with serde derives ---
#[derive(Serialize, Deserialize, Default)]
struct Model {
    input: String,
    notes: Vec<String>,
    editing: Option<(usize, String)>,
}

/// --- User intents remain unchanged ---
enum Msg {
    SetInput(String),
    AddNote,
    Delete(usize),
    Edit(usize),
    SetEditInput(String),
    SaveEdit,
    CancelEdit,
}

/// --- Pure update function stays the same ---
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

/// --- 2) view function also unchanged from the last version ---
fn view(ui: &mut Ui, model: &mut Model) {
    // 1) Input row
    ui.horizontal(|ui| {
        // clone out current input
        let mut new_input = model.input.clone();

        // if the user types, send Msg::SetInput
        if ui.text_edit_singleline(&mut new_input).changed() {
            update(model, Msg::SetInput(new_input.clone()));
        }

        // Add button
        if ui.button("Add").clicked() {
            update(model, Msg::AddNote);
        }
    });

    ui.separator();

    // Record actions (as before)
    let mut to_delete = None;
    let mut to_edit = None;
    let mut to_save = false;
    let mut to_cancel = false;
    let mut new_edit_buf = None;

    // Reorderable list:
    Dnd::new(ui, "sticky-notes").show_vec(
        &mut model.notes,
        |ui, note: &mut String, handle, state| {
            ui.horizontal(|ui| {
                // 1) draw drag-handle
                handle.ui(ui, |ui| {
                    // You can put any small icon/text here:
                    ui.label("☰");
                });

                // 2) inline‐edit or display
                if let Some((edit_idx, existing)) = &model.editing {
                    if *edit_idx == state.index {
                        let mut buf = existing.clone();
                        if ui.text_edit_singleline(&mut buf).changed() {
                            new_edit_buf = Some(buf);
                        }
                        if ui.small_button("💾").clicked() {
                            to_save = true;
                        }
                        if ui.small_button("✖").clicked() {
                            to_cancel = true;
                        }
                        return;
                    }
                }

                // 3) normal display with edit/delete
                ui.label(&*note);
                if ui.small_button("✏️").clicked() {
                    to_edit = Some(state.index);
                }
                if ui.small_button("×").clicked() {
                    to_delete = Some(state.index);
                }
            });
        },
    );

    // 4) replay actions
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
/// --- 3) The App struct and persistence hooks ---
struct StickyApp {
    model: Model,
}

impl StickyApp {
    /// Load from storage or fallback to Default
    fn new(cc: &CreationContext<'_>) -> Self {
        let model = cc
            .storage
            .and_then(|storage| storage.get_string("sticky_app"))
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default();
        Self { model }
    }
}

impl App for StickyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            view(ui, &mut self.model);
        });
    }

    /// Called on app exit — save our model back to storage
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Ok(json) = serde_json::to_string(&self.model) {
            storage.set_string("sticky_app", json);
        }
    }
}

fn main() -> eframe::Result<()> {
    let opts = NativeOptions::default();
    eframe::run_native(
        "Sticky Notes – Persisted",
        opts,
        Box::new(|cc| {
            // wrap your App in Ok(...) to satisfy the factory signature
            Ok(Box::new(StickyApp::new(cc)))
        }),
    )
}
