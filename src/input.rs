use rltk::Rltk;

pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Escape,
}

pub fn parse_input(ctx: &Rltk) -> Option<Input> {
    match ctx.key {
        None => None,
        Some(key) => match key {
            rltk::VirtualKeyCode::W | rltk::VirtualKeyCode::Up => Some(Input::Up),
            rltk::VirtualKeyCode::A | rltk::VirtualKeyCode::Left => Some(Input::Left),
            rltk::VirtualKeyCode::S | rltk::VirtualKeyCode::Down => Some(Input::Down),
            rltk::VirtualKeyCode::D | rltk::VirtualKeyCode::Right => Some(Input::Right),
            _ => None
        }
    }
}