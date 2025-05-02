use eframe::{run_native, NativeOptions};
use egui::{self, vec2, Sense, Label, Layout, Align, ViewportCommand};
use egui::viewport::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_decorations(false)               // no title bar/borders
            .with_always_on_top()                  // float above other windows
            .with_transparent(false)               // solid background
            .with_inner_size(vec2(200.0, 200.0)),  // initial size
        ..Default::default()
    };

    run_native(
        "Stickie Prototype",
        native_options,
        Box::new(|_cc| Ok(Box::new(StickieApp::default()))),
    )
}

struct StickieApp {
    text: String,
}

impl Default for StickieApp {
    fn default() -> Self {
        Self { text: String::new() }
    }
}

impl eframe::App for StickieApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Yellow background
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(ctx.screen_rect(), 0.0, egui::Color32::from_rgb(242, 232, 130));

        // Top draggable bar with close button
        egui::TopBottomPanel::top("title_bar").exact_height(24.0).show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Draggable title
                let resp = ui.add(Label::new("Stickie").sense(Sense::drag()));
                if resp.dragged() {
                    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
                }
                // Close button aligned right
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("✕").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            });
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.text)
                    .frame(false)
                    .hint_text("Type your note here…")
            );
        });
    }
}
