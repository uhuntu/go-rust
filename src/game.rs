use draw;
use ggez::event;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::input;
use ggez::timer;
use ggez::{Context, GameResult};

use pixel_math::*;

pub const BOARD_SIZE: usize = 19;

lazy_static! {
    pub static ref HANDICAPS: Vec<(u16, u16)> = vec![
        (3, 3),
        (9, 3),
        (15, 3),
        (3, 9),
        (9, 9),
        (15, 9),
        (3, 15),
        (9, 15),
        (15, 15),
    ];
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Black,
    White,
}

pub struct Board {
    pub contents: Vec<Vec<Option<Piece>>>,
}

impl Board {
    fn new() -> Board {
        let v: Vec<Vec<Option<Piece>>> = vec![vec![None; BOARD_SIZE]; BOARD_SIZE];
        Board { contents: v }
    }
}

pub struct MainState {
    board: Board,
    current_turn: Piece,
}

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            board: Board::new(),
            current_turn: Piece::Black,
        };

        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            // hello
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // graphics::clear(ctx);
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        draw::add_board_background(ctx)?;

        // const BLACK: (u8, u8, u8) = (0, 0, 0);
        let game_mesh = draw::build_game_mesh(ctx, &self.board)?;
        // graphics::set_color(ctx, BLACK.into())?;
        graphics::draw(ctx, &game_mesh, DrawParam::default())?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == input::mouse::MouseButton::Left {
            let (i, j) = screen_to_board(x, y).unwrap();
            if self.current_turn == Piece::Black {
                self.current_turn = Piece::White;
                self.board.contents[i][j] = Some(Piece::White);
            } else {
                self.current_turn = Piece::Black;
                self.board.contents[i][j] = Some(Piece::Black);
            }
        }
    }
}
