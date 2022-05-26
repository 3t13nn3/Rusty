use ggez::event;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::Context;
use glam::*;

const WIDTH: f32 = super::super::WIDTH;

pub struct Character {
    texture: graphics::Image,
    pos: Vec2,
    velocity: f32,
    rot: f32,
    rot_right: bool,
    rot_velocity: f32,
}

impl Character {
    pub fn new(ctx: &mut Context, height: f32) -> Character {
        let mut tex = graphics::Image::new(ctx, "/rustacean.png").unwrap();
        //avoiding fuzzy render
        tex.set_filter(graphics::FilterMode::Nearest);

        let positions = Vec2::new(220., height - tex.height() as f32 / 1.75 * 0.2 + 20.);

        let c = Character {
            texture: tex,
            pos: positions,
            velocity: 0.,
            rot: 0.,
            rot_right: true,
            rot_velocity: 0.035,
        };

        c
    }

    fn rotate(&mut self, _ctx: &Context) {
        if keyboard::is_key_pressed(_ctx, event::KeyCode::Right) {
            if self.rot_right {
                self.rot += self.rot_velocity;
            } else {
                self.rot -= self.rot_velocity;
            }
        } else if keyboard::is_key_pressed(_ctx, event::KeyCode::Left) {
            if !self.rot_right {
                self.rot += self.rot_velocity;
            } else {
                self.rot -= self.rot_velocity;
            }
        } else {
            if self.rot <= 0. {
                self.rot = -self.rot_velocity.abs();
            } else {
                self.rot = self.rot_velocity.abs();
            }
        }

        // Clamping rotation to 0.2
        let out = self.rot >= 0.2 || self.rot <= -0.2;
        if self.rot_right && out {
            //assigning rot to left
            self.rot_right = false;
        } else if !self.rot_right && out {
            //else right
            self.rot_right = true;
        }
    }

    fn position(&mut self, _ctx: &Context) {
        if keyboard::is_key_pressed(_ctx, event::KeyCode::Right) {
            if self.velocity <= 60. {
                self.velocity += 1.05;
            }
            //dynamic rotation in function of speed
            self.rot_velocity = 0.03 + self.velocity / 500.;
        } else if keyboard::is_key_pressed(_ctx, event::KeyCode::Left) {
            if self.velocity >= -60. {
                self.velocity -= 1.05;
            }
            //dynamic rotation in function of speed
            self.rot_velocity = -0.03 + self.velocity / 500.;
        } else {
            self.velocity /= 1.15;
            self.rot_velocity = -0.2 + self.velocity / 500.
        }
        self.pos[0] += self.velocity;
        println!("{}", self.velocity);

        //Clamping pos
        if self.pos[0] < self.texture.width() as f32 / 1.75 * 0.2 + 20. {
            self.pos[0] = self.texture.width() as f32 / 1.75 * 0.2 + 20.;
        } else if self.pos[0] >= WIDTH / 2. {
            self.pos[0] = WIDTH / 2.;
        }
    }

    pub fn update(&mut self, _ctx: &Context) {
        self.position(_ctx);
        self.rotate(_ctx);
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let param = graphics::DrawParam::new()
            .dest(self.pos)
            .scale(Vec2::new(0.2, 0.2))
            .rotation(self.rot)
            .offset(Vec2::new(0.5, 0.5));
        graphics::draw(ctx, &self.texture, param).unwrap();
    }

    pub fn get_velocity(&self) -> f32 {
        self.velocity
    }
}
