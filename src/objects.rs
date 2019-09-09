use crate::HeghState;
use ggez::{
    event::KeyCode,
    graphics::{self, Image},
    input::keyboard,
    nalgebra::Point2,
    Context,
};
use std::time::{Duration, Instant};

const SHOT_SPEED: f32 = 2.5;
const FIRE_RATE: u64 = 100;

pub struct Hegh {
    image: Image,
    position: Point2<f32>,
    fire_rate: Duration,
    last_shot: Instant,
    bullets: Vec<Bullet>,
}

impl Hegh {
    pub fn new(image: Image) -> Self {
        Self {
            image,
            position: Point2::new(300.0, 500.0),
            bullets: Vec::new(),
            fire_rate: Duration::from_millis(FIRE_RATE),
            last_shot: Instant::now(),
        }
    }
    pub fn update(&mut self, ctx: &mut Context, bullet_image: Image) {
        if keyboard::is_key_pressed(ctx, KeyCode::Space) {
            // Fire bullets
            let now = Instant::now();
            if now.duration_since(self.last_shot) > self.fire_rate {
                self.bullets
                    .push(Bullet::new(bullet_image, self.position()));
                self.last_shot = now;
            }
        }

        if keyboard::is_key_pressed(ctx, KeyCode::Up) {
            if self.position.y - 1.0 < 0.0 {
                self.position.y = 0.0
            } else {
                self.position.y -= 1.0
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Down) {
            if self.position.y + 1.0 > 550.0 {
                self.position.y = 550.0
            } else {
                self.position.y += 1.0
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            if self.position.x - 1.0 < 0.0 {
                self.position.x = 0.0
            } else {
                self.position.x -= 1.0
            }
        }
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            if self.position.x + 1.0 > 730.0 {
                self.position.x = 730.0
            } else {
                self.position.x += 1.0
            }
        }

        self.bullets.retain(|b| !b.on_screen);
        self.bullets.iter_mut().for_each(|b| b.update(ctx));
    }

    pub fn draw(&self, ctx: &mut Context) {
        let _ = graphics::draw(ctx, &self.image, (self.position,));
        self.bullets.iter().for_each(|b| b.draw(ctx));
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}

pub struct Enemy {
    image: Image,
    position: Point2<f32>,
    fire_rate: Duration,
    last_shot: Instant,
    pub on_screen: bool,
}

impl Enemy {
    pub fn new(image: Image, position: Point2<f32>) -> Self {
        Self {
            image,
            position,
            fire_rate: Duration::from_millis(FIRE_RATE),
            last_shot: Instant::now(),
            on_screen: true,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        self.position.y += 1.0 * 0.5;
        if self.position.y > 550.0 {
            self.on_screen = true;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let _ = graphics::draw(ctx, &self.image, (self.position,));
    }
}

pub struct Bullet {
    image: Image,
    position: Point2<f32>,
    velocity: f32,
    pub on_screen: bool,
}

impl Bullet {
    pub fn new(image: Image, position: Point2<f32>) -> Self {
        Self {
            image,
            position,
            velocity: SHOT_SPEED,
            on_screen: false,
        }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        self.position.y -= 1.0 * self.velocity;
        if self.position.y <= 0.0 {
            self.on_screen = true;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        let _ = graphics::draw(ctx, &self.image, (self.position(),));
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}
