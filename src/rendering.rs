use rltk::Rltk;

pub trait Renderable {
    fn render(&mut self, ctx: &mut Rltk);
}