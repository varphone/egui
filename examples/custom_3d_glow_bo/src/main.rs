#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use eframe::egui;
use egui::mutex::Mutex;
use glow::HasContext;
use std::sync::Arc;

mod mesh;
mod points_geometry;
mod points_mesh;

use mesh::Mesh;
use points_geometry::PointsGeometry;
use points_mesh::PointsMesh;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(350.0, 380.0)),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    rotating_triangle: Arc<Mutex<RotatingTriangle>>,
    points_mesh: Arc<Mutex<PointsMesh>>,
    angle: f32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            rotating_triangle: Arc::new(Mutex::new(RotatingTriangle::new(gl))),
            points_mesh: Arc::new(Mutex::new(PointsMesh::new(gl))),
            angle: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("The triangle is being painted using ");
                ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
                ui.label(" (OpenGL).");
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
            ui.label("Drag to rotate!");
            let mut my_f32 = self.points_mesh.lock().point_size;
            ui.add(egui::Slider::new(&mut my_f32, 0.0..=100.0).text("Point Size"));
            self.points_mesh.lock().point_size = my_f32;
        });
    }

    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.rotating_triangle.lock().destroy(gl);
        }
    }
}

impl MyApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(400.0), egui::Sense::drag());

        self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let angle = self.angle;
        let rotating_triangle = self.rotating_triangle.clone();
        let points_mesh = self.points_mesh.clone();
        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                // rotating_triangle.lock().paint(painter.gl(), angle);
                // println!("Painted!");
                points_mesh.lock().draw(painter.gl());
            })),
        };
        ui.painter().add(callback);
    }
}

struct RotatingTriangle {
    program: glow::Program,
    vertex_buffer: glow::Buffer,
    vertex_array: glow::VertexArray,
}

impl RotatingTriangle {
    unsafe fn create_vertex_buffer(
        gl: &glow::Context,
    ) -> (glow::NativeBuffer, glow::NativeVertexArray) {
        let verts: &[f32] = &[
            0.5f32, 1.0f32, // top
            0.0f32, 0.0f32, // left
            1.0f32, 0.0f32, // right
        ];
        let colors: &[f32] = &[
            1.0, 0.0, 0.0, 1.0, // red
            0.0, 1.0, 0.0, 1.0, // green
            0.0, 0.0, 1.0, 1.0, // blue
        ];
        let verts_u8: &[u8] = core::slice::from_raw_parts(
            verts.as_ptr() as *const u8,
            verts.len() * core::mem::size_of::<f32>(),
        );
        let colors_u8: &[u8] = core::slice::from_raw_parts(
            colors.as_ptr() as *const u8,
            colors.len() * core::mem::size_of::<f32>(),
        );

        println!("verts_u8: {:?}", verts_u8.len());
        println!("colors_u8: {:?}", colors_u8.len());

        let mut buffer_u8 = Vec::with_capacity(verts_u8.len() + colors_u8.len());
        for (vert, color) in verts_u8.chunks(8).zip(colors_u8.chunks(16)) {
            buffer_u8.extend_from_slice(vert);
            buffer_u8.extend_from_slice(color);
        }
        // let triangle_vertices_u8 = verts_u8
        //     .chunks(8)
        //     .zip(colors_u8.chunks(16))
        //     .flat_map(|(vert, color)| vec![*vert, *color])
        //     .collect::<Vec<u8>>();
        println!("buffer: {:?}", buffer_u8.len());

        // We construct a buffer and upload the data
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &buffer_u8, glow::STATIC_DRAW);

        // We now construct a vertex array to describe the format of the input buffer
        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 24, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 24, 8);

        (vbo, vao)
    }

    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;

        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                r#"
                    layout (location = 0) in vec2 in_position;
                    layout (location = 1) in vec4 in_color;
                    // uniform float u_angle;
                    out vec2 v_position;
                    out vec4 v_color;
                    void main() {
                        v_position = in_position;
                        v_color = in_color;
                        gl_Position = vec4(in_position - 0.5, 0.0, 1.0);
                    }
                "#,
                r#"
                    precision mediump float;
                    in vec2 v_position;
                    in vec4 v_color;
                    out vec4 out_color;
                    void main() {
                        out_color = v_color;
                    }
                "#,
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();

            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            // let vertex_array = gl
            //     .create_vertex_array()
            //     .expect("Cannot create vertex array");

            let (vertex_buffer, vertex_array) = Self::create_vertex_buffer(gl);

            Self {
                program,
                vertex_buffer,
                vertex_array,
            }
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
            gl.delete_buffer(self.vertex_buffer);
        }
    }

    fn paint(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            // gl.clear_color(0.1, 0.2, 0.3, 1.0);
            // gl.clear(glow::COLOR_BUFFER_BIT);
            gl.use_program(Some(self.program));
            // gl.uniform_1_f32(
            //     gl.get_uniform_location(self.program, "u_angle").as_ref(),
            //     angle,
            // );
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}
