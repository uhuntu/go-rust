extern crate ggez;

use game::{Board, HANDICAPS};
use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder};
use ggez::{graphics, mint};
use ggez::{Context, GameResult};
use pixel_math;
use pixel_math::{MARGIN, POSITION_SIZE, SCREEN_SIZE};

pub fn build_game_mesh(ctx: &mut Context, _board: &Board) -> GameResult<Mesh> {
    let mb = &mut MeshBuilder::new();

    add_grid_to_mesh(mb);

    add_handicaps_to_mesh(mb);

    // add_pieces_to_mesh(mb, board);

    mb.build(ctx)
}

pub fn add_board_background(ctx: &mut Context) -> GameResult<()> {
    const BEIGE: (u8, u8, u8) = (245, 245, 220);

    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            MARGIN.0,
            MARGIN.1,
            SCREEN_SIZE.0 - MARGIN.0 * 2.0,
            SCREEN_SIZE.1 - MARGIN.1 * 2.0,
        ),
        BEIGE.into(),
    )?;

    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    Ok(())
}

pub fn add_grid_to_mesh(mb: &mut MeshBuilder) {
    const LINE_WIDTH: f32 = 2.0;
    const BLACK: (u8, u8, u8) = (0, 0, 0);

    for r in pixel_math::ROWS.iter() {
        mb.line(
            &[
                mint::Point2 {
                    x: MARGIN.0 + POSITION_SIZE.0 / 2.0,
                    y: r + POSITION_SIZE.1 / 2.0,
                },
                mint::Point2 {
                    x: SCREEN_SIZE.0 - MARGIN.0 - POSITION_SIZE.0 / 2.0,
                    y: r + POSITION_SIZE.1 / 2.0,
                },
            ],
            LINE_WIDTH,
            BLACK.into(),
        )
        .unwrap();
    }

    for c in pixel_math::COLUMNS.iter() {
        mb.line(
            &[
                mint::Point2 {
                    x: c + POSITION_SIZE.0 / 2.0,
                    y: MARGIN.1 + POSITION_SIZE.1 / 2.0,
                },
                mint::Point2 {
                    x: c + POSITION_SIZE.0 / 2.0,
                    y: SCREEN_SIZE.1 - MARGIN.1 - POSITION_SIZE.1 / 2.0,
                },
            ],
            LINE_WIDTH,
            BLACK.into(),
        )
        .unwrap();
    }
}

pub fn add_handicaps_to_mesh(mb: &mut MeshBuilder) {
    for (i, j) in HANDICAPS.iter() {
        mb.circle(
            DrawMode::fill(),
            mint::Point2 {
                x: MARGIN.0 + POSITION_SIZE.0 * *i as f32 + POSITION_SIZE.0 / 2.0,
                y: MARGIN.1 + POSITION_SIZE.1 * *j as f32 + POSITION_SIZE.1 / 2.0,
            },
            6.0,
            6.0,
            Color::BLACK,
        )
        .unwrap();
    }
}
