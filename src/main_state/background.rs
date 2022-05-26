use ggez::graphics;
use ggez::Context;
use glam::*;

use std::fs;

pub struct Backgrounds {
    pos_x: Vec<f32>,
    layers: Vec<graphics::Image>,
    scaled_factor: Vec<f32>,
    velocity: f32,
}

impl Backgrounds {
    pub fn new(ctx: &mut Context, height: f32) -> Backgrounds {
        //recover images
        let mut l: Vec<graphics::Image> = Vec::new();
        // Calculating scale
        let mut scales: Vec<f32> = Vec::new();

        // Sorted entry form a directory
        let mut paths: Vec<_> = fs::read_dir("./resources/layers")
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        paths.sort_by_key(|dir| dir.path());

        for path in paths {
            let file = path.path();
            let path_string = file.display().to_string();
            let p: Vec<&str> = path_string.split("/").collect();
            // println!("{}",p.last().unwrap());
            let mut e =
                graphics::Image::new(ctx, "/layers/".to_owned() + p.last().unwrap()).unwrap();
            //avoiding fuzzy render
            e.set_filter(graphics::FilterMode::Nearest);
            l.push(e);

            // Filling scale factor on the fly
            scales.push(height / l.last().unwrap().height() as f32);
        }

        // Init x positions
        let x = vec![0.0; l.len()];

        let b = Backgrounds {
            pos_x: x,
            layers: l,
            scaled_factor: scales,
            velocity: 3.,
        };
        b
    }

    /*
        pub fn get_pos_x(&self) -> &Vec<f32> {
            &self.pos_x
        }

        pub fn get_velocity(&self) -> f32 {
            self.velocity
        }

        pub fn get_scaled_factor(&self) -> &Vec<f32> {
            &self.scaled_factor
        }

        pub fn get_layers(&self) -> &Vec<graphics::Image> {
            &self.layers
        }
    */
    pub fn update(&mut self, _ctx: &Context, velocity: f32) {
        self.velocity = velocity / 8.;

        for (i, p) in self.pos_x.iter_mut().enumerate() {
            *p -= (1. + i as f32) * self.velocity;
            if *p <= -self.scaled_factor[i] * self.layers[i].width() as f32 {
                *p += self.scaled_factor[i] * self.layers[i].width() as f32;
            }
            if *p >= 0. {
                *p -= self.scaled_factor[i] * self.layers[i].width() as f32;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        for (i, e) in self.layers.iter().enumerate() {
            let param = graphics::DrawParam::new()
                .dest(Vec2::new(self.pos_x[i], 0.0))
                .scale(Vec2::new(self.scaled_factor[i], self.scaled_factor[i]));
            //.rotation(self.background.get_pos_x() / 100.0)
            //            .offset(Vec2::new(0.5, 0.5))
            graphics::draw(ctx, e, param).unwrap();
            let param_sub = param.clone().dest(Vec2::new(
                self.pos_x[i] + self.scaled_factor[i] * e.width() as f32,
                0.,
            ));
            graphics::draw(ctx, e, param_sub).unwrap();
        }
    }
}
