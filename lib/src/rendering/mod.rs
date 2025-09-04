use render_agnostic::Renderer;

pub trait Render {
    fn render(&self, renderer: &mut dyn Renderer);
}
