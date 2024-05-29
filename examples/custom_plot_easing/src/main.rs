//! This example shows how to implement custom gestures to pan and zoom in the plot
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::time::Duration;

use eframe::{
    egui::{self, Color32, DragValue, Event, Rounding, Sense, Vec2},
    emath::Easing,
};
use egui_plot::{Legend, Line, PieChart, PlotPoints};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Plot",
        options,
        Box::new(|_cc| Ok(Box::<PlotExample>::default())),
    )
}

struct PlotExample {
    lock_x: bool,
    lock_y: bool,
    ctrl_to_zoom: bool,
    shift_to_horizontal: bool,
    zoom_speed: f32,
    scroll_speed: f32,
    easing: Easing,
}

impl Default for PlotExample {
    fn default() -> Self {
        Self {
            lock_x: false,
            lock_y: false,
            ctrl_to_zoom: false,
            shift_to_horizontal: false,
            zoom_speed: 1.0,
            scroll_speed: 1.0,
            easing: Easing::SineIn,
        }
    }
}

impl eframe::App for PlotExample {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("options").show(ctx, |ui| {
            ui.checkbox(&mut self.lock_x, "Lock x axis").on_hover_text("Check to keep the X axis fixed, i.e., pan and zoom will only affect the Y axis");
            ui.checkbox(&mut self.lock_y, "Lock y axis").on_hover_text("Check to keep the Y axis fixed, i.e., pan and zoom will only affect the X axis");
            ui.checkbox(&mut self.ctrl_to_zoom, "Ctrl to zoom").on_hover_text("If unchecked, the behavior of the Ctrl key is inverted compared to the default controls\ni.e., scrolling the mouse without pressing any keys zooms the plot");
            ui.checkbox(&mut self.shift_to_horizontal, "Shift for horizontal scroll").on_hover_text("If unchecked, the behavior of the shift key is inverted compared to the default controls\ni.e., hold to scroll vertically, release to scroll horizontally");
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.zoom_speed)
                        .clamp_range(0.1..=2.0)
                        .speed(0.1),
                );
                ui.label("Zoom speed").on_hover_text("How fast to zoom in and out with the mouse wheel");
            });
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.scroll_speed)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1),
                );
                ui.label("Scroll speed").on_hover_text("How fast to pan with the mouse wheel");
            });

            let (response, painter) =  ui.allocate_painter(Vec2::splat(100.0), Sense::click());
            let t = ui.input(|i| i.time);
            let radius = 10.0 + self.easing.apply(t.sin().abs()) * 30.0;
            painter.rect_filled(response.rect, Rounding::ZERO, Color32::BLACK);
            painter.circle_filled(response.rect.center(), radius as f32, Color32::BLUE);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical_centered_justified(|ui| {
                    for easing in Easing::all() {
                        ui.selectable_value(&mut self.easing, easing, format!("{easing:?}"));
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (scroll, pointer_down, modifiers) = ui.input(|i| {
                let scroll = i.events.iter().find_map(|e| match e {
                    Event::MouseWheel {
                        unit: _,
                        delta,
                        modifiers: _,
                    } => Some(*delta),
                    _ => None,
                });
                (scroll, i.pointer.primary_down(), i.modifiers)
            });

            ui.label("This example shows how to use raw input events to implement different plot controls than the ones egui provides by default, e.g., default to zooming instead of panning when the Ctrl key is not pressed, or controlling much it zooms with each mouse wheel step.");


            egui_plot::Plot::new("plot")
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .data_aspect(1.0)
                .legend(Legend::default().position(egui_plot::Corner::RightBottom))
                .show(ui, |plot_ui| {
                    if let Some(mut scroll) = scroll {
                        if modifiers.ctrl == self.ctrl_to_zoom {
                            scroll = Vec2::splat(scroll.x + scroll.y);
                            let mut zoom_factor = Vec2::from([
                                (scroll.x * self.zoom_speed / 10.0).exp(),
                                (scroll.y * self.zoom_speed / 10.0).exp(),
                            ]);
                            if self.lock_x {
                                zoom_factor.x = 1.0;
                            }
                            if self.lock_y {
                                zoom_factor.y = 1.0;
                            }
                            plot_ui.zoom_bounds_around_hovered(zoom_factor);
                        } else {
                            if modifiers.shift == self.shift_to_horizontal {
                                scroll = Vec2::new(scroll.y, scroll.x);
                            }
                            if self.lock_x {
                                scroll.x = 0.0;
                            }
                            if self.lock_y {
                                scroll.y = 0.0;
                            }
                            let delta_pos = self.scroll_speed * scroll;
                            plot_ui.translate_bounds(delta_pos);
                        }
                    }
                    if plot_ui.response().hovered() && pointer_down {
                        let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
                        if self.lock_x {
                            pointer_translate.x = 0.0;
                        }
                        if self.lock_y {
                            pointer_translate.y = 0.0;
                        }
                        plot_ui.translate_bounds(pointer_translate);
                    }
/*
                    let pie = Pie::new([0.0, 0.0], 1.0, 30.0f32.to_radians(), 120.0f32.to_radians()).name("Pie A")
                        .shrink(1.0);
                    plot_ui.pie(pie);
                    let pie = Pie::new([0.0, 0.0], 1.0, 120.0f32.to_radians(), 190.0f32.to_radians())
                        // .stroke(egui::Stroke::new(1.0, Color32::from_rgb(0, 255, 0)))
                        .shrink(1.0)
                        .name("Pie B");
                    plot_ui.pie(pie);
                    let pie = Pie::new([0.0, 0.0], 1.0, 190.0f32.to_radians(), 290.0f32.to_radians())
                        // .stroke(egui::Stroke::new(1.0, Color32::from_rgb(0, 255, 0)))
                        .shrink(1.0)
                        .name("Pie C");
                    plot_ui.pie(pie);
                    let arc_line = ArcLine::new([0.0, 0.0], 0.5, 30.0f32.to_radians(), -170.0f32.to_radians()).name("Arc Line")
                        .stroke(egui::Stroke::new(30.0, Color32::from_rgb(0, 255, 0)));
                    plot_ui.arc_line(arc_line);
                    //     let sine_points = PlotPoints::from_explicit_callback(move |x| easing.apply(x), .., 5000);
                    //     plot_ui.line(Line::new(sine_points).name(format!("{:?}", easing)));
                    // }
*/

                    let data: Vec<f64> = vec![100.0, 200.0, 300.0, 400.0];
                    let labels = vec!["A Data".to_owned(), "B".to_owned(), "C".to_owned(), "D".to_owned()];
                    let pie_chart = PieChart::new([0.0, 0.0], 5.0, data).name("Pie Chart").labels(labels);
                    plot_ui.pie_chart(pie_chart);

                    let easeing = self.easing;
                    let points = PlotPoints::from_explicit_callback(move |x| easeing.apply(x), 0.0..=1.0, 1000);
                    plot_ui.line(Line::new(points).name(format!("{easeing:?}")));
                });
        });

        ctx.request_repaint_after(Duration::from_millis(15));
    }
}
