extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::{Context, ContextBuilder, GameResult};
use rand::{thread_rng, Rng};
use std::{env, path};

const WINDOW_SIZE: u32 = 400;

struct MainState {
    board: Vec<u8>,
    zero: (u8, u8),
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut board: Vec<u8> = (0..16).collect();
        let slice: &mut [u8] = &mut board;
        thread_rng().shuffle(slice);
        while !validate_board_state(slice) {
            thread_rng().shuffle(slice);
        }
        let s = MainState {
            board: slice.to_vec(),
            zero: (0, 0),
        };
        Ok(s)
    }
}

fn validate_board_state(board: &[u8]) -> bool {
    let mut inv_count = 0;
    let mut zero_poz = 0;
    for i in 0..15 {
        for j in 0..16 {
            if board[j] == 0 {
                zero_poz = j
            }
            if i < j && board[i] > board[j] {
                inv_count += 1;
            }
        }
    }
    zero_poz /= 4;
    if zero_poz % 2 == 0 {
        inv_count % 2 != 0
    } else {
        inv_count % 2 == 0
    }
}

fn idx(x: usize, y: usize) -> usize {
    x + 4 * y
}

fn swap(board: &mut Vec<u8>, loc1: (u8, u8), zero: (u8, u8)) {
    let l1 = (loc1.0 as i32, loc1.1 as i32);
    let z = (zero.0 as i32, zero.1 as i32);
    match ((l1.0 - z.0).abs() as u8, (l1.1 - z.1).abs() as u8) {
        (0, 1) => do_swap(board, loc1, zero),
        (1, 0) => do_swap(board, loc1, zero),
        _ => (),
    }
}

fn do_swap(board: &mut Vec<u8>, loc1: (u8, u8), zero: (u8, u8)) {
    println!("{:?}, {:?}", loc1, zero);
    let il1 = idx(loc1.0 as usize, loc1.1 as usize);
    let il2 = idx(zero.0 as usize, zero.1 as usize);
    let l1 = board[il1];
    let l2 = board[il2];
    let tmp = l1;
    board[il1] = l2;
    board[il2] = tmp;
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let blue = graphics::Color::new(0.0, 0.0, 1.0, 1.0);
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 20).unwrap();
        graphics::clear(ctx);
        for x in 0..4 {
            for y in 0..4 {
                let val = self.board[idx(x, y)];
                match val {
                    0 => {
                        let s =
                            graphics::Rect::new(x as f32 * 100.0, y as f32 * 100.0, 100.0, 100.0);
                        graphics::set_color(ctx, graphics::BLACK)?;
                        graphics::rectangle(ctx, graphics::DrawMode::Fill, s)?;
                        self.zero = (x as u8, y as u8)
                    }
                    _ => {
                        let text = graphics::Text::new(ctx, &val.to_string(), &font)?;
                        let f_w = font.get_width(&val.to_string()) as f32;
                        let f_h = font.get_height() as f32;
                        let center = graphics::Point2::new(
                            (x as f32 * 100.0 + 50.0 + f_w / 2.0)
                                - font.get_width(&val.to_string()) as f32,
                            (y as f32 * 100.0 + 50.0 + f_h / 2.0) - font.get_height() as f32,
                        );
                        let s =
                            graphics::Rect::new(x as f32 * 100.0, y as f32 * 100.0, 100.0, 100.0);
                        graphics::set_color(ctx, blue)?;
                        graphics::rectangle(ctx, graphics::DrawMode::Fill, s)?;
                        graphics::set_color(ctx, graphics::WHITE)?;
                        let b = graphics::Rect::new(x as f32 * 100.0, y as f32 * 100.0, 99.0, 99.0);

                        graphics::rectangle(ctx, graphics::DrawMode::Line(1.0), b)?;
                        graphics::draw(ctx, &text, center, 0.0)?;
                    }
                }
            }
        }
        graphics::present(ctx);
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: i32,
        y: i32,
    ) {
        let loc = (
            (x as f32 / 100f32).floor() as u8,
            (y as f32 / 100f32).floor() as u8,
        );
        swap(&mut self.board, loc, self.zero)
    }
}

pub fn main() {
    let cb = ContextBuilder::new("15", "ggez")
        .window_setup(conf::WindowSetup::default().title("15"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_SIZE, WINDOW_SIZE));
    let ctx = &mut cb.build().unwrap();
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
