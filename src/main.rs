use eframe::{run_native, NativeOptions};
use egui::{self, vec2};
use egui::viewport::ViewportBuilder;

fn main() -> eframe::Result<()> {
    // Build our always-on-top, borderless, fixed-size window:
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_decorations(false)               // no title bar or borders
            .with_always_on_top()                  // float above other windows
            .with_transparent(false)               // solid background
            .with_inner_size(vec2(200.0, 200.0)),  // initial size
        ..Default::default()
    };

    run_native(
        "Stickie Prototype",
        native_options,
        Box::new(|_cc| Ok(Box::new(StickieApp::default()))),  // wrap in Ok
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
        // Yellow background for the sticky-note look:
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(ctx.screen_rect(), 0.0, egui::Color32::from_rgb(242, 232, 130));

        // Editable text area:
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::TextEdit::multiline(&mut self.text)
                .frame(false)
                .hint_text("Type your note here…"));
        });
    }
}
