use crate::{Mesh, PointsGeometry};
use glow::HasContext as _;
use glow::Program;

pub struct PointsMesh {
    pub geometry: PointsGeometry,
    pub program: Program,
    pub point_size: f32,
}

impl PointsMesh {
    pub fn new(gl: &glow::Context) -> Self {
        Self {
            geometry: PointsGeometry::new(gl),
            program: create_points_mesh_program(gl),
            point_size: 2.0f32,
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
        }
    }
}

impl Mesh for PointsMesh {
    fn draw(&self, gl: &glow::Context) {
        unsafe {
            gl.enable(glow::VERTEX_PROGRAM_POINT_SIZE);
            gl.use_program(Some(self.program));
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_point_size")
                    .as_ref(),
                self.point_size,
            );
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.geometry.vbo));
            gl.bind_vertex_array(Some(self.geometry.vao));
            gl.draw_arrays(glow::POINTS, 0, self.geometry.num_points as i32);
        }
    }
}

fn create_points_mesh_program(gl: &glow::Context) -> Program {
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
                uniform float u_point_size;
                out vec2 v_position;
                out vec4 v_color;
                void main() {
                    v_position = in_position;
                    v_color = in_color;
                    gl_Position = vec4(in_position - 0.5, 0.0, 1.0);
                    gl_PointSize = u_point_size;
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
        program
    }
}
