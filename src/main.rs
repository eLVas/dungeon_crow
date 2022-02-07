use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, it's me a terminal.");

        if let Some(VirtualKeyCode::Escape) = ctx.key {
            ctx.quitting = true
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crow")
        .with_fullscreen(true)
        .build()?;

    main_loop(context, State {})
}
