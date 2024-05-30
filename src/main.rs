//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{Context, event, GameError, GameResult, glam::*, graphics::{self, Color}};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::Rect;
use ggez::input::keyboard::KeyInput;
use rand::Rng;

const WIDTH: usize = 80;
const HEIGHT: usize = 60;
const FILL_RATE: f32 = 35.0;

struct MainState {
    pos_x: f32,
    rect: graphics::Mesh,
    alive: Vec<Vec<bool>>,
    start: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new_i32(0, 0, 10, 10),
            Color::WHITE,
        )?;
        let mut v = vec![vec![false; HEIGHT]; WIDTH];
        let mut rng = rand::thread_rng();
        // for i in 0..WIDTH {
        //     for j in 0..HEIGHT {
        //         v[i][j] = rng.gen::<f32>() < FILL_RATE / 100.0;
        //     }
        // }
        init(&mut v);
        Ok(MainState { pos_x: 0.0, rect, alive: v, start: true })
    }
}

fn init(v: &mut Vec<Vec<bool>>) {
    v[40][30] = true;
    v[40][31] = true;
    v[40][32] = true;
    v[41][32] = true;
    v[42][32] = true;
    v[42][31] = true;
    v[42][30] = true;
}

fn copy(v: &mut Vec<Vec<bool>>, src: &Vec<Vec<bool>>) {
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            v[i][j] = src[i][j];
        }
    }
}

fn neighbor(v: &Vec<Vec<bool>>, x: &usize, y: &usize) -> u8 {
    let mut total: u8 = 0;
    let first_x: usize = if *x == 0 { 0 } else { *x - 1 };
    let last_x: usize = if *x == WIDTH - 1 { WIDTH - 1 } else { *x + 1 };
    let first_y: usize = if *y == 0 { 0 } else { *y - 1 };
    let last_y: usize = if *y == HEIGHT - 1 { HEIGHT - 1 } else { *y + 1 };

    for i in first_x..last_x + 1 {
        for j in first_y..last_y + 1 {
            if i != *x || j != *y {
                if v[i][j] {
                    total += 1;
                }
            }
        }
    }
    total
}

fn gof(v: &mut Vec<Vec<bool>>) {
    let mut target = vec![vec![false; HEIGHT]; WIDTH];
    std::thread::sleep_ms(100);
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            match neighbor(v, &i, &j) {
                3 => target[i][j] = true,
                2 => target[i][j] = v[i][j],
                _ => target[i][j] = false,
            }
        }
    }
    copy(v, &target);
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 0.5;
        if self.start {
            self.start = false
        } else {
            gof(&mut self.alive);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        // canvas.draw(&self.circle, Vec2::new(self.pos_x, 380.0));
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                if self.alive[i][j] {
                    canvas.draw(&self.rect, Vec2::new(0.0 + (i as f32 * 10.0), 0.0 + (j as f32 * 10.0)));
                }
            }
        }
        canvas.finish(ctx)?;

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        std::process::exit(0);
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("gof", "ggez")
        .window_setup(WindowSetup::default().title("Game of life"))
        .window_mode(WindowMode::default()
            .dimensions(1024.0, 768.0));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}