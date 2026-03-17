#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use eframe::egui::{self, Align2, Area, CentralPanel, Color32, Frame, Id, Order, RichText};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Topmost Windows",
        options,
        Box::new(|_| Ok(Box::<TopmostWindowsApp>::default())),
    )
}

struct TopmostWindowsApp {
    show_foreground_overlay: bool,
    show_normal_windows: [bool; 3],
    show_topmost_window: bool,
    overlay_anchor: Align2,
}

impl Default for TopmostWindowsApp {
    fn default() -> Self {
        Self {
            show_foreground_overlay: true,
            show_normal_windows: [true; 3],
            show_topmost_window: true,
            overlay_anchor: Align2::CENTER_CENTER,
        }
    }
}

impl eframe::App for TopmostWindowsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Topmost window layering");
            ui.add_space(8.0);
            ui.label("This example keeps several windows on the normal layer and one on the new topmost layer.");
            ui.label("The highlighted title bar shows the top window within each layer order.");
            ui.label("The Foreground overlay stays above both windows to mimic menus and popups.");
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_normal_windows[0], "Show normal window A");
                ui.checkbox(&mut self.show_normal_windows[1], "Show normal window B");
                ui.checkbox(&mut self.show_normal_windows[2], "Show normal window C");
                ui.checkbox(&mut self.show_topmost_window, "Show topmost window");
                ui.checkbox(&mut self.show_foreground_overlay, "Show Foreground overlay");
            });

            ui.add_space(8.0);
            ui.group(|ui| {
                ui.label(RichText::new("How to verify").strong());
                ui.label("1. Click and drag the normal windows so they overlap each other. They should reorder among themselves.");
                ui.label("2. Bring different normal windows to the front. The topmost window should still stay above all of them.");
                ui.label("3. Drag the topmost window. It should stay above normal windows and keep its highlighted title bar.");
                ui.label("4. Toggle the Foreground overlay. It should cover both normal and topmost windows when visible.");
            });
        });

        for (index, (title, position, color)) in [
            (
                "Normal Window A",
                egui::pos2(70.0, 150.0),
                Color32::from_rgb(90, 135, 220),
            ),
            (
                "Normal Window B",
                egui::pos2(140.0, 220.0),
                Color32::from_rgb(70, 170, 150),
            ),
            (
                "Normal Window C",
                egui::pos2(210.0, 290.0),
                Color32::from_rgb(160, 120, 210),
            ),
        ]
        .into_iter()
        .enumerate()
        {
            if !self.show_normal_windows[index] {
                continue;
            }

            egui::Window::new(title)
                .id(Id::new(title))
                .default_pos(position)
                .default_size([300.0, 200.0])
                .show(ctx, |ui| {
                    ui.colored_label(color, "Middle layer");
                    ui.separator();
                    ui.label("This is a normal window on Order::Middle.");
                    ui.label("Clicking it only reorders it relative to the other normal windows.");
                    ui.label("Even when this becomes the front-most normal window, it should stay behind the topmost window.");
                });
        }

        if self.show_topmost_window {
            egui::Window::new("Topmost Window")
                .default_pos(egui::pos2(360.0, 180.0))
                .default_size([340.0, 220.0])
                .topmost(true)
                .show(ctx, |ui| {
                    ui.colored_label(Color32::from_rgb(220, 120, 70), "Topmost layer");
                    ui.separator();
                    ui.label("This window uses Window::topmost(true).");
                    ui.label(
                        "It stays above Order::Middle windows without entering Order::Foreground.",
                    );
                    ui.label("Menus, popups, and other Foreground content can still cover it.");
                });
        }

        if self.show_foreground_overlay {
            let offset = egui::vec2(0.0, -40.0);
            Area::new(Id::new("foreground_overlay"))
                .order(Order::Foreground)
                .anchor(self.overlay_anchor, offset)
                .show(ctx, |ui| {
                    Frame::popup(&ctx.style()).show(ui, |ui| {
                        ui.set_width(320.0);
                        ui.label(RichText::new("Foreground overlay").strong());
                        ui.label("This block is painted on Order::Foreground.");
                        ui.label("It intentionally overlaps the topmost window.");
                        ui.add_space(6.0);
                        if ui.button("Hide overlay").clicked() {
                            self.show_foreground_overlay = false;
                        }
                    });
                });
        }
    }
}
