pub trait Mesh {
    fn draw(&self, gl: &glow::Context);
}
