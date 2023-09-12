use glow::HasContext as _;
use glow::{Buffer, VertexArray};

pub struct PointsGeometry {
    pub vbo: Buffer,
    pub vao: VertexArray,
    pub num_points: usize,
}

impl PointsGeometry {
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

    pub fn new(gl: &glow::Context) -> Self {
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

            let (vbo, vao) = Self::create_vertex_buffer(gl);

            Self {
                vbo,
                vao,
                num_points: 3,
            }
        }
    }

    pub fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_vertex_array(self.vao);
            gl.delete_buffer(self.vbo);
        }
    }
}
