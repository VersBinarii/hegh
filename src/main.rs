mod objects;

use ggez::{
    event::{self, EventHandler},
    graphics::{self, Image},
    nalgebra::Point2,
    Context, GameResult,
};
use std::time::{Duration, Instant};

use crate::objects::{Enemy, Hegh};
use rand::prelude::*;

const ENEMY_SPAWN_RATE: u64 = 1000;

pub struct HeghState {
    hegh: Hegh,
    enemies: Vec<Enemy>,
    enemy_image: Image,
    bullet_image: Image,
    enemy_spawn_rate: Duration,
    enemy_spawn_last: Instant,
}

impl EventHandler for HeghState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.hegh.update(ctx, self.bullet_image.clone());

        // Spawn enemies periodically
        let spawn_time = Instant::now();
        if spawn_time.duration_since(self.enemy_spawn_last)
            > self.enemy_spawn_rate
        {
            let mut rng = thread_rng();
            let rand_y = rng.gen_range(0, 730);
            let dest = Point2::new(rand_y as f32, 0.0);
            self.enemies
                .push(Enemy::new(self.enemy_image.clone(), dest));
            self.enemy_spawn_last = spawn_time;
        }

        self.enemies.retain(|e| e.on_screen);
        self.enemies.iter_mut().for_each(|e| e.update(ctx));
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.hegh.draw(ctx);

        self.enemies.iter().for_each(|e| e.draw(ctx));
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    use ggez::conf::*;
    let resource_dir = std::path::PathBuf::from("./assets");

    let cb = ggez::ContextBuilder::new("Hegh", "VersBinarii")
        .add_resource_path(resource_dir)
        .window_setup(
            WindowSetup::default()
                .title("Hegh")
                .samples(NumSamples::Zero)
                .vsync(true),
        )
        .window_mode(
            WindowMode::default()
                .maximized(true)
                .fullscreen_type(FullscreenType::Desktop),
        );

    let (ctx, event_loop) = &mut cb.build().expect("Failed to build ggez!");

    let hegh_image = Image::new(ctx, "/hegh.png")?;
    let bullet_image = Image::new(ctx, "/bullet.png")?;
    let enemy_image = Image::new(ctx, "/enemy.png")?;

    let hegh = Hegh::new(hegh_image);

    let mut hegh_state = HeghState {
        hegh,
        enemies: Vec::new(),
        enemy_image,
        bullet_image,
        enemy_spawn_last: Instant::now(),
        enemy_spawn_rate: Duration::from_millis(ENEMY_SPAWN_RATE),
    };

    event::run(ctx, event_loop, &mut hegh_state)
}
