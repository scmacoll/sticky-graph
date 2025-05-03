use eframe::{run_native, NativeOptions};
use egui::{self, vec2, Layout, Align, Button, Sense, ViewportCommand, ScrollArea, Frame, Margin, TextEdit, Key, CursorIcon};
use egui::viewport::ViewportBuilder;

fn main() -> eframe::Result<()> {
    let min_size = vec2(150.0, 150.0);
    let initial_size = vec2(200.0, 200.0);

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(false)
            .with_inner_size(initial_size)
            .with_min_inner_size(min_size),
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
        // Snapshot input
        let input = ctx.input(|i| i.clone());

        // Change cursor to hand on Cmd+hover, reset otherwise
        ctx.output_mut(|output| {
            if input.modifiers.command && input.pointer.hover_pos().is_some() {
                output.cursor_icon = CursorIcon::PointingHand;
            } else {
                output.cursor_icon = CursorIcon::Default;
            }
        });

        // Cmd+click anywhere to drag
        if input.modifiers.command && input.pointer.primary_pressed() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }

        // Cmd+N to spawn new stickie
        if input.modifiers.command && input.key_pressed(Key::N) {
            if let Ok(exe) = std::env::current_exe() {
                let _ = std::process::Command::new(exe).spawn();
            }
        }

        // Yellow background
        let painter = ctx.layer_painter(egui::LayerId::background());
        painter.rect_filled(ctx.screen_rect(), 0.0, egui::Color32::from_rgb(242, 232, 130));

        // Top draggable bar with close "x"
        egui::TopBottomPanel::top("title_bar").exact_height(24.0).show(ctx, |ui| {
            let full_rect = ui.max_rect();
            let resp = ui.interact(full_rect, ui.id().with("drag_bar"), Sense::drag());
            if resp.dragged() {
                ctx.send_viewport_cmd(ViewportCommand::StartDrag);
            }
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.label("Stickie");
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.add(Button::new("x").frame(false)).clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            });
        });

        // Main content area with padding and auto-scrollbar
        egui::CentralPanel::default().show(ctx, |ui| {
            Frame::NONE
                .inner_margin(Margin { left: 8, right: 8, top: 4, bottom: 8 })
                .show(ui, |ui| {
                    ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            let avail = ui.available_width();
                            ui.add(
                                TextEdit::multiline(&mut self.text)
                                    .frame(false)
                                    .hint_text("Type your note here…")
                                    .desired_rows(10)
                                    .desired_width(avail)
                            );
                        });
                });
        });
    }
}
