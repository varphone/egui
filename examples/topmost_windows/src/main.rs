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
        Box::new(|cc| {
            let mut visuals = egui::Visuals::light();
            visuals.widgets.open.weak_bg_fill = Color32::from_rgb(220, 120, 70);
            cc.egui_ctx.set_visuals_of(egui::Theme::Light, visuals);
            Ok(Box::<TopmostWindowsApp>::default())
        }),
    )
}

struct TopmostWindowsApp {
    show_foreground_overlay: bool,
    show_normal_windows: [bool; 3],
    show_topmost_windows: [bool; 2],
    show_attached_child_windows: [bool; 2],
    overlay_anchor: Align2,
}

impl Default for TopmostWindowsApp {
    fn default() -> Self {
        Self {
            show_foreground_overlay: true,
            show_normal_windows: [true; 3],
            show_topmost_windows: [true; 2],
            show_attached_child_windows: [true; 2],
            overlay_anchor: Align2::CENTER_CENTER,
        }
    }
}

impl eframe::App for TopmostWindowsApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Topmost window layering");
            ui.add_space(8.0);
            ui.label("This example keeps several windows on the normal layer and multiple windows on the topmost layer.");
            ui.label("It also shows multiple attached child windows that stay directly above a topmost parent.");
            ui.label("The highlighted title bar shows the single globally active window, including the front-most attached child.");
            ui.label("The Foreground overlay stays above both windows to mimic menus and popups.");
            ui.add_space(12.0);

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_normal_windows[0], "Show normal window A");
                ui.checkbox(&mut self.show_normal_windows[1], "Show normal window B");
                ui.checkbox(&mut self.show_normal_windows[2], "Show normal window C");
                ui.checkbox(&mut self.show_topmost_windows[0], "Show topmost parent");
                ui.checkbox(&mut self.show_topmost_windows[1], "Show topmost sibling");
                ui.checkbox(&mut self.show_attached_child_windows[0], "Show attached child A");
                ui.checkbox(&mut self.show_attached_child_windows[1], "Show attached child B");
                ui.checkbox(&mut self.show_foreground_overlay, "Show Foreground overlay");
            });

            ui.add_space(8.0);
            ui.group(|ui| {
                ui.label(RichText::new("How to verify").strong());
                ui.label("1. Click and drag the normal windows so they overlap each other. They should reorder among themselves.");
                ui.label("2. Bring different normal windows to the front. Both topmost windows should still stay above all of them.");
                ui.label("3. Click between the two topmost windows. Only the front-most topmost window should show the active title bar.");
                ui.label("4. Keep both attached child windows visible, then click inside the parent topmost window. The front-most child should stay above its parent and be the only attached child with the active title bar.");
                ui.label("5. Drag the topmost parent window. Both attached child windows remain above the parent and above normal windows.");
                ui.label("6. Toggle the Foreground overlay. It should cover both topmost windows and all attached child windows when visible.");
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
                .show(&ctx, |ui| {
                    ui.colored_label(color, "Middle layer");
                    ui.separator();
                    ui.label("This is a normal window on Order::Middle.");
                    ui.label("Clicking it only reorders it relative to the other normal windows.");
                    ui.label("Even when this becomes the front-most normal window, it should stay behind the topmost window.");
                });
        }

        if self.show_topmost_windows[0] {
            egui::Window::new("Topmost Parent")
                .default_pos(egui::pos2(360.0, 180.0))
                .default_size([340.0, 220.0])
                .topmost(true)
                .show(&ctx, |ui| {
                    ui.colored_label(Color32::from_rgb(220, 120, 70), "Topmost layer");
                    ui.separator();
                    ui.label("This window uses Window::topmost(true).");
                    ui.label(
                        "It stays above Order::Middle windows without entering Order::Foreground.",
                    );
                    ui.label("The attached child windows are shown with Window::show_sublayer_of.");
                    ui.label("Click this parent window while the children overlap it to verify the parent does not steal the active title bar.");
                    ui.label("Menus, popups, and other Foreground content can still cover both.");

                    if self.show_attached_child_windows[0] {
                        egui::Window::new("Attached Child A")
                            .id(Id::new("attached_child_window_a"))
                            .default_pos(egui::pos2(560.0, 250.0))
                            .default_size([280.0, 160.0])
                            .show_sublayer_of(ui.ctx(), ui.layer_id(), |ui| {
                                ui.colored_label(Color32::from_rgb(245, 200, 90), "Attached sublayer");
                                ui.separator();
                                ui.label("This child window inherits the parent's topmost order.");
                                ui.label("It is registered as a sublayer of the parent window.");
                                ui.label("When both children are visible, this one should only be active if it is the front-most child.");
                            });
                    }

                    if self.show_attached_child_windows[1] {
                        egui::Window::new("Attached Child B")
                            .id(Id::new("attached_child_window_b"))
                            .default_pos(egui::pos2(620.0, 300.0))
                            .default_size([280.0, 160.0])
                            .show_sublayer_of(ui.ctx(), ui.layer_id(), |ui| {
                                ui.colored_label(Color32::from_rgb(230, 170, 70), "Attached sublayer");
                                ui.separator();
                                ui.label("This is a second child sublayer of the same topmost parent.");
                                ui.label("Because it is shown later, it starts in front of child A.");
                                ui.label("Only the front-most child should show the active title bar.");
                            });
                    }
                });
        }

        if self.show_topmost_windows[1] {
            egui::Window::new("Topmost Sibling")
                .id(Id::new("topmost_sibling_window"))
                .default_pos(egui::pos2(720.0, 170.0))
                .default_size([300.0, 200.0])
                .topmost(true)
                .show(&ctx, |ui| {
                    ui.colored_label(Color32::from_rgb(210, 90, 90), "Topmost layer");
                    ui.separator();
                    ui.label("This is an independent topmost window.");
                    ui.label("It competes with the topmost parent and its child windows for the active title bar.");
                    ui.label("Only the front-most topmost stack member should look active at a time.");
                });
        }

        if self.show_foreground_overlay {
            let offset = egui::vec2(0.0, -40.0);
            Area::new(Id::new("foreground_overlay"))
                .order(Order::Foreground)
                .anchor(self.overlay_anchor, offset)
                .show(&ctx, |ui| {
                    Frame::popup(ui.style()).show(ui, |ui| {
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
