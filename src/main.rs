extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event::{self, MouseButton};
use ggez::graphics::{self};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use rand::{seq::SliceRandom, thread_rng};
use std::{env, path};

const WINDOW_SIZE: f32 = 400_f32;

struct MainState {
    board: Vec<u8>,
    zero: (u8, u8),
    solved: bool,
}

impl MainState {
    fn new() -> Self {
        let mut board: Vec<u8> = (0..16).collect();
        let slice: &mut [u8] = &mut board;
        slice.shuffle(&mut thread_rng());
        while !validate_board_state(slice) {
            slice.shuffle(&mut thread_rng());
        }
        MainState {
            board: slice.to_vec(),
            zero: (0, 0),
            solved: false,
        }
    }
}

fn validate_board_state(board: &[u8]) -> bool {
    let mut inv_count = 0;
    let mut zerow = 0;
    for i in 0..15 {
        for j in 0..16 {
            if board[j] == 0 {
                zerow = j
            }
            if i < j && board[j] != 0 && board[i] > board[j] {
                inv_count += 1;
            }
        }
    }
    zerow /= 4;
    (zerow % 2 == 0) ^ (inv_count % 2 == 0)
}

fn idx(x: usize, y: usize) -> usize {
    x.wrapping_mul(4).wrapping_add(y)
}

fn swap(board: &mut Vec<u8>, loc1: (u8, u8), zero: (u8, u8)) {
    let l1 = (i32::from(loc1.0), i32::from(loc1.1));
    let z = (i32::from(zero.0), i32::from(zero.1));
    match ((l1.0 - z.0).abs() as u8, (l1.1 - z.1).abs() as u8) {
        (0, 1) => do_swap(board, loc1, zero),
        (1, 0) => do_swap(board, loc1, zero),
        _ => (),
    }
}

fn do_swap(board: &mut Vec<u8>, loc1: (u8, u8), zero: (u8, u8)) {
    let il1 = idx(loc1.0 as usize, loc1.1 as usize);
    let il2 = idx(zero.0 as usize, zero.1 as usize);
    let l1 = board[il1];
    let l2 = board[il2];
    let tmp = l1;
    board[il1] = l2;
    board[il2] = tmp;
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        let blue = graphics::Color::new(0.0, 0.0, 1.0, 1.0);
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf").unwrap();

        for x in 0..4 {
            for y in 0..4 {
                // dbg!(&x, &y);
                let val = self.board[idx(x, y)];
                match val {
                    0 => {
                        let s =
                            graphics::Rect::new(x as f32 * 100.0, y as f32 * 100.0, 100.0, 100.0);
                        graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            s,
                            graphics::Color::BLACK,
                        )?;
                        self.zero = (x as u8, y as u8)
                    }
                    _ => {
                        let b_bounds =
                            graphics::Rect::new(x as f32 * 100.0, y as f32 * 100.0, 99.0, 99.0);

                        let b = graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            b_bounds,
                            graphics::Color::WHITE,
                        )?;
                        graphics::draw(ctx, &b, graphics::DrawParam::default())?;
                        let text = graphics::Text::new((u8::to_string(&val), font, 18.0));
                        let f_w = text.dimensions(ctx).x as f32 / 2.0;
                        let f_h = text.dimensions(ctx).y as f32 / 2.0;
                        graphics::draw(
                            ctx,
                            &text,
                            graphics::DrawParam::new().color(blue).dest(Point2::from([
                                b_bounds.x + 50.0 - f_w,
                                b_bounds.y + 50.0 - f_h,
                            ])),
                        )?;
                    }
                }
            }
        }

        let winning: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        if winning == &self.board[..] {
            self.solved = true;
            graphics::clear(ctx, graphics::Color::BLACK);
            let text = graphics::Text::new(("You Win!", font, 36.0));
            let f_w = text.dimensions(ctx).x as f32;
            let f_h = text.dimensions(ctx).y as f32;
            let center = Point2::from([200.0 + (f_w / 2.0) - f_w, 200.0 + (f_h / 2.0) - f_h]);
            graphics::draw(
                ctx,
                &text,
                graphics::DrawParam::new()
                    .color(graphics::Color::WHITE)
                    .dest(center),
            )?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if !self.solved {
            let loc = (
                (x as f32 / 100f32).floor() as u8,
                (y as f32 / 100f32).floor() as u8,
            );
            swap(&mut self.board, loc, self.zero)
        }
    }
}

pub fn main() -> GameResult {
    let mut cb = ContextBuilder::new("Fifteen", "nathan");
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(path);
    }
    let (ctx, events_loop) = cb
        .window_setup(conf::WindowSetup::default().title("Fifteen!"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_SIZE, WINDOW_SIZE))
        .build()?;

    let state = MainState::new();
    event::run(ctx, events_loop, state)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_validates_a_solvable_puzzle() {
        let valid = &[6, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 12, 13, 3];
        assert!(::validate_board_state(valid));
    }
    #[test]
    fn it_validates_a_solved_puzzle() {
        let valid = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        assert!(::validate_board_state(valid));
    }
    #[test]
    fn it_rejects_canonical_unsolvable() {
        let invalid = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 14, 0];
        assert!(!::validate_board_state(invalid));
    }
    #[test]
    fn it_invalidates_other_unsolvable_states() {
        let invalid = &[15, 9, 13, 8, 7, 14, 12, 3, 11, 1, 5, 10, 4, 2, 0, 6];
        assert!(!::validate_board_state(invalid));
    }
    #[test]
    fn it_validates_other_solvable_states() {
        let valid = &[12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3];
        assert!(::validate_board_state(valid));
    }
    #[test]
    fn it_allows_valid_move_left() {
        let mut board = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3];
        let swapped_board = &[12, 1, 10, 2, 7, 11, 4, 14, 5, 9, 0, 15, 8, 13, 6, 3];
        ::swap(&mut board, (2, 2), (1, 2));
        assert_eq!(board, swapped_board)
    }

    #[test]
    fn it_allows_valid_move_right() {
        let mut board = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3];
        let swapped_board = &[12, 1, 10, 2, 7, 11, 4, 14, 0, 5, 9, 15, 8, 13, 6, 3];
        ::swap(&mut board, (0, 2), (1, 2));
        assert_eq!(board, swapped_board)
    }
    #[test]
    fn it_allows_valid_move_down() {
        let mut board = vec![12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3];
        let swapped_board = &[12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
        ::swap(&mut board, (1, 1), (1, 2));
        assert_eq!(board, swapped_board)
    }
    #[test]
    fn it_allows_valid_move_up() {
        let mut board = vec![12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
        let swapped_board = &[12, 1, 10, 2, 7, 11, 4, 14, 5, 0, 9, 15, 8, 13, 6, 3];
        ::swap(&mut board, (1, 2), (1, 1));
        assert_eq!(board, swapped_board)
    }
    #[test]
    fn it_disallows_invalid_move() {
        let mut board = vec![12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
        let board_copy = vec![12, 1, 10, 2, 7, 0, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
        let swapped_board = &[0, 1, 10, 2, 7, 12, 4, 14, 5, 11, 9, 15, 8, 13, 6, 3];
        ::swap(&mut board, (0, 0), (1, 1));
        assert_ne!(board, swapped_board);
        assert_eq!(board, board_copy)
    }
}
