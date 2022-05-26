use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

mod background;
mod character;

pub struct MainState {
    background: background::Backgrounds,
    character: character::Character,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let m = MainState {
            background: background::Backgrounds::new(ctx, super::HEIGHT),
            character: character::Character::new(ctx, super::HEIGHT),
        };
        Ok(m)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.background.update(_ctx, self.character.get_velocity());
        self.character.update(_ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.background.draw(ctx);
        self.character.draw(ctx);
        graphics::present(ctx)?;

        Ok(())
    }
}
