// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]

use eframe::{egui, egui_glow, glow};
use egui::mutex::Mutex;
use egui_gizmo::{Gizmo, GizmoMode};
use glow::HasContext as _;
use std::sync::Arc;

struct RenderOptions {
    model: nalgebra::Matrix4<f32>,
    view: nalgebra::Matrix4<f32>,
    projection: nalgebra::Matrix4<f32>,
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 380.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D Cube painting in eframe using glow",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    /// Behind an `Arc<Mutex<…>>` so we can pass it to [`egui::PaintCallback`] and paint later.
    rotating_triangle: Arc<Mutex<RotatingTriangle>>,
    model_matrix: nalgebra::Matrix4<f32>,
    view_matrix: nalgebra::Matrix4<f32>,
    projection_matrix: nalgebra::Matrix4<f32>,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");

        let model_matrix = nalgebra::Matrix4::identity();
        let view_matrix = nalgebra::Matrix4::look_at_rh(
            &nalgebra::Point3::new(3.0, 3.0, 3.0),
            &nalgebra::Point3::origin(),
            &nalgebra::Vector3::new(0.0, 1.0, 0.0),
        );
        let projection_matrix =
            nalgebra::Matrix4::new_perspective(45.0f32.to_radians(), 1.0, 0.1, 10.0);

        Self {
            rotating_triangle: Arc::new(Mutex::new(RotatingTriangle::new(gl))),
            model_matrix,
            view_matrix,
            projection_matrix,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("The Cube is being painted using ");
                ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
                ui.label(" (OpenGL).");
            });

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);

                let v: [f32; 16] = self.model_matrix.as_slice().try_into().unwrap();
                let mut model_matrix = mint::ColumnMatrix4::from(v);
                let v: [f32; 16] = self.view_matrix.as_slice().try_into().unwrap();
                let mut view_matrix = mint::ColumnMatrix4::from(v);
                let v: [f32; 16] = self.projection_matrix.as_slice().try_into().unwrap();
                let mut projection_matrix = mint::ColumnMatrix4::from(v);

                let gizmo = Gizmo::new("My gizmo")
                    .model_matrix(model_matrix)
                    .view_matrix(view_matrix)
                    .projection_matrix(projection_matrix)
                    .mode(GizmoMode::Rotate);

                let ui: &mut egui::Ui = ui;
                if let Some(response) = gizmo.interact(ui) {
                    let model_matrix = response.rotation;
                    println!("model_matrix: {:?}", model_matrix);
                    let model_matrix: &[f32; 16] = model_matrix.as_ref();
                    self.model_matrix =
                        self.model_matrix * nalgebra::Matrix4::from_column_slice(model_matrix);
                }
            });
            ui.label("Drag to rotate!");
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
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        // self.angle += response.drag_delta().x * 0.01;

        // Clone locals so we can move them into the paint callback:
        let mut rotating_triangle = self.rotating_triangle.clone();
        // rotating_triangle.model = self.model_matrix;
        // rotating_triangle.view = self.view_matrix;
        // rotating_triangle.projection = self.projection_matrix;

        let options = RenderOptions {
            model: self.model_matrix,
            view: self.view_matrix,
            projection: self.projection_matrix,
        };

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                rotating_triangle.lock().paint(painter.gl(), &options);
            })),
        };
        ui.painter().add(callback);
    }
}

struct RotatingTriangle {
    program: glow::Program,
    vertex_buffer: glow::Buffer,
    vertex_array: glow::VertexArray,
    color_buffer: glow::Buffer,
    color_array: glow::VertexArray,
}

impl RotatingTriangle {
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
                    precision mediump float;
                    in vec3 position;
                    in vec3 color;
                    uniform mat4 u_model;
                    uniform mat4 u_view;
                    uniform mat4 u_projection;
                    out vec3 v_color;
                    void main() {
                        v_color = color;
                        gl_Position = u_projection * u_view * u_model * vec4(position, 1.0);
                    }
                "#,
                r#"
                    precision mediump float;
                    in vec3 v_color;
                    out vec4 out_color;
                    void main() {
                        out_color = vec4(v_color, 1.0);
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

            gl.bind_attrib_location(program, 0, "position");
            gl.bind_attrib_location(program, 1, "color");

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

            let vertex_location = gl
                .get_attrib_location(program, "position")
                .expect("Cannot get attrib `position` location");
            let color_location = gl
                .get_attrib_location(program, "color")
                .expect("Cannot get attrib `color` location");

            print!(
                "vertex_location: {}, color_location: {}",
                vertex_location, color_location
            );

            let (vertex_buffer, vertex_array) = create_vertex_buffer(gl, vertex_location);
            let (color_buffer, color_array) = create_color_buffer(gl, color_location);

            // let model = nalgebra::Matrix4::identity();
            // let view = nalgebra::Matrix4::look_at_rh(
            //     &nalgebra::Point3::new(3.0, 3.0, 3.0),
            //     &nalgebra::Point3::origin(),
            //     &nalgebra::Vector3::new(0.0, 1.0, 0.0),
            // );
            // let projection =
            //     nalgebra::Matrix4::new_perspective(45.0f32.to_radians(), 1.0, 0.1, 10.0);

            Self {
                program,
                vertex_buffer,
                vertex_array,
                color_buffer,
                color_array,
                // model,
                // view,
                // projection,
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

    fn paint(&self, gl: &glow::Context, options: &RenderOptions) {
        use glow::HasContext as _;
        unsafe {
            // gl.clear_color(0.1, 0.2, 0.3, 1.0);
            // gl.clear(glow::COLOR_BUFFER_BIT);
            // gl.enable(glow::DEPTH_TEST);
            // gl.enable(glow::CULL_FACE);
            // gl.cull_face(glow::BACK);
            // gl.front_face(glow::CCW);
            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_model").as_ref(),
                false,
                options.model.as_slice(),
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_view").as_ref(),
                false,
                options.view.as_slice(),
            );
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "u_projection")
                    .as_ref(),
                false,
                options.projection.as_slice(),
            );
            // gl.uniform_1_f32(
            //     gl.get_uniform_location(self.program, "u_angle").as_ref(),
            //     angle,
            // );
            // gl.enable_vertex_attrib_array(0);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertex_buffer));
            gl.bind_vertex_array(Some(self.vertex_array));
            // gl.enable_vertex_attrib_array(1);
            // gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.color_buffer));
            // gl.bind_vertex_array(Some(self.color_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 36);
        }
    }
}

unsafe fn create_color_buffer(
    gl: &glow::Context,
    location: u32,
) -> (glow::Buffer, glow::VertexArray) {
    // This is a flat array of f32s that are to be interpreted as vec3s.
    #[rustfmt::skip]
    let cube_colors: [f32; 72] = [
        // 前面
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        1.0, 0.0, 0.0,
        // 后面
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        0.0, 1.0, 0.0,
        // 左面
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        0.0, 0.0, 1.0,
        // 右面
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        1.0, 1.0, 0.0,
        // 上面
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        0.0, 1.0, 1.0,
        // 下面
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
        1.0, 0.0, 1.0,
    ];

    let cube_colors_u8: &[u8] = core::slice::from_raw_parts(
        cube_colors.as_ptr() as *const u8,
        cube_colors.len() * core::mem::size_of::<f32>(),
    );

    // We construct a buffer and upload the data
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cube_colors_u8, glow::STATIC_DRAW);

    // We now construct a vertex array to describe the format of the input buffer
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.vertex_attrib_pointer_f32(location, 3, glow::FLOAT, false, 12, 0);
    // gl.enable_vertex_attrib_array(location);

    (vbo, vao)
}

unsafe fn create_vertex_buffer(
    gl: &glow::Context,
    location: u32,
) -> (glow::Buffer, glow::VertexArray) {
    // This is a flat array of f32s that are to be interpreted as vec3s.
    #[rustfmt::skip]
    let cube_vertices: [f32; 216] = [
        // 前面
        -1.0, -1.0,  1.0, 1.0, 0.0, 0.0, // 红色
         1.0, -1.0,  1.0, 0.0, 1.0, 0.0, // 绿色
         1.0,  1.0,  1.0, 0.0, 0.0, 1.0, // 蓝色
        -1.0, -1.0,  1.0, 1.0, 0.0, 0.0, // 红色
         1.0,  1.0,  1.0, 0.0, 0.0, 1.0, // 蓝色
        -1.0,  1.0,  1.0, 1.0, 1.0, 0.0, // 黄色
        // 后面
        -1.0, -1.0, -1.0, 1.0, 0.0, 1.0, // 品红
        -1.0,  1.0, -1.0, 0.0, 1.0, 1.0, // 青色
         1.0,  1.0, -1.0, 1.0, 1.0, 1.0, // 白色
        -1.0, -1.0, -1.0, 1.0, 0.0, 1.0, // 品红
         1.0,  1.0, -1.0, 1.0, 1.0, 1.0, // 白色
         1.0, -1.0, -1.0, 0.0, 0.0, 0.0, // 黑色
        // 左面
        -1.0,  1.0, -1.0, 1.0, 0.0, 0.0, // 红色
        -1.0,  1.0,  1.0, 0.0, 1.0, 0.0, // 绿色
        -1.0, -1.0,  1.0, 0.0, 0.0, 1.0, // 蓝色
        -1.0,  1.0, -1.0, 1.0, 0.0, 0.0, // 红色
        -1.0, -1.0,  1.0, 0.0, 0.0, 1.0, // 蓝色
        -1.0, -1.0, -1.0, 1.0, 1.0, 0.0, // 黄色
        // 右面
         1.0,  1.0, -1.0, 1.0, 0.0, 1.0, // 品红
         1.0,  1.0,  1.0, 0.0, 1.0, 1.0, // 青色
         1.0, -1.0,  1.0, 1.0, 1.0, 1.0, // 白色
         1.0,  1.0, -1.0, 1.0, 0.0, 1.0, // 品红
         1.0, -1.0,  1.0, 1.0, 1.0, 1.0, // 白色
         1.0, -1.0, -1.0, 0.0, 0.0, 0.0, // 黑色
        // 上面
        -1.0,  1.0, -1.0, 1.0, 0.0, 0.0, // 红色
         1.0,  1.0, -1.0, 0.0, 1.0, 0.0, // 绿色
         1.0,  1.0,  1.0, 0.0, 0.0, 1.0, // 蓝色
        -1.0,  1.0, -1.0, 1.0, 0.0, 0.0, // 红色
         1.0,  1.0,  1.0, 0.0, 0.0, 1.0, // 蓝色
        -1.0,  1.0,  1.0, 1.0, 1.0, 0.0, // 黄色
        // 下面
        -1.0, -1.0, -1.0, 1.0, 0.0, 1.0, // 品红
         1.0, -1.0, -1.0, 0.0, 1.0, 1.0, // 青色
         1.0, -1.0,  1.0, 1.0, 1.0, 1.0, // 白色
        -1.0, -1.0, -1.0, 1.0, 0.0, 1.0, // 品红
         1.0, -1.0,  1.0, 1.0, 1.0, 1.0, // 白色
        -1.0, -1.0,  1.0, 0.0, 0.0, 0.0, // 黑色
    ];

    let cube_vertices_u8: &[u8] = core::slice::from_raw_parts(
        cube_vertices.as_ptr() as *const u8,
        cube_vertices.len() * core::mem::size_of::<f32>(),
    );

    // We construct a buffer and upload the data
    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cube_vertices_u8, glow::STATIC_DRAW);

    // We now construct a vertex array to describe the format of the input buffer
    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));
    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 24, 0);
    gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 24, 12);
    gl.enable_vertex_attrib_array(0);
    gl.enable_vertex_attrib_array(1);

    (vbo, vao)
}
