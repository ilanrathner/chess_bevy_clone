use bevy::{prelude::*};
use crate::bitboard::{Square, SquareLabels};

pub const SQUARE_SIZE: f32 = 80.0;

pub const LIGHT_SQUARE_COLOR: Color = Color::srgb_u8(232, 182, 114);
pub const DARK_SQUARE_COLOR: Color = Color::srgb_u8(132, 84, 42);

#[derive(Resource)]
pub struct PiecesAssets {
    b_bishop: Handle<Image>,
    w_bishop: Handle<Image>,
    b_knight: Handle<Image>,
    w_knight: Handle<Image>,
    b_rook: Handle<Image>,
    w_rook: Handle<Image>,
    b_king: Handle<Image>,
    w_king: Handle<Image>,
    b_queen: Handle<Image>,
    w_queen: Handle<Image>,
    b_pawn: Handle<Image>,
    w_pawn: Handle<Image>,
}

pub fn load_pieces_resource(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PiecesAssets {
        b_bishop: asset_server.load("b_bishop.png"),
        w_bishop: asset_server.load("w_bishop.png"),
        b_knight: asset_server.load("b_knight.png"),
        w_knight: asset_server.load("w_knight.png"),
        b_rook: asset_server.load("b_rook.png"),
        w_rook: asset_server.load("w_rook.png"),
        b_king: asset_server.load("b_king.png"),
        w_king: asset_server.load("w_king.png"),
        b_queen: asset_server.load("b_queen.png"),
        w_queen: asset_server.load("w_queen.png"),
        b_pawn: asset_server.load("b_pawn.png"),
        w_pawn: asset_server.load("w_pawn.png"),
    });
}

fn load_board(mut commands: Commands, piece_sprites: Res<PiecesAssets>) {
    commands.spawn(Camera2d);

    //x y is middle of the screen. so offset start 4 squares left - half a square. Then cannjust add a square on each time.
    let start_offset = -(SQUARE_SIZE  *4.0 - (SQUARE_SIZE / 2.0));

    for row in 0..8_usize {
        for col in 0..8_usize{
            let x = start_offset + col as f32 * SQUARE_SIZE;
            let y = start_offset + row as f32 * SQUARE_SIZE;

            let base_color = if (row + col) % 2 == 1 {
                LIGHT_SQUARE_COLOR
            } else {
                DARK_SQUARE_COLOR
            };

            let square = SquareLabels::from_index(row*8 + col);

            commands.spawn((
                Sprite::from_color(base_color, Vec2::splat(SQUARE_SIZE)),
                Transform::from_xyz(x, y, 0.0),
                Square(square as usize)
            ));

            let image = match square {
                // White back rank (rank 1)
                SquareLabels::A1 | SquareLabels::H1 => piece_sprites.w_rook.clone(),
                SquareLabels::B1 | SquareLabels::G1 => piece_sprites.w_knight.clone(),
                SquareLabels::C1 | SquareLabels::F1 => piece_sprites.w_bishop.clone(),
                SquareLabels::D1 => piece_sprites.w_queen.clone(),
                SquareLabels::E1 => piece_sprites.w_king.clone(),

                // White pawns (rank 2)
                SquareLabels::A2
                | SquareLabels::B2
                | SquareLabels::C2
                | SquareLabels::D2
                | SquareLabels::E2
                | SquareLabels::F2
                | SquareLabels::G2
                | SquareLabels::H2 => piece_sprites.w_pawn.clone(),

                // Black pawns (rank 7)
                SquareLabels::A7
                | SquareLabels::B7
                | SquareLabels::C7
                | SquareLabels::D7
                | SquareLabels::E7
                | SquareLabels::F7
                | SquareLabels::G7
                | SquareLabels::H7 => piece_sprites.b_pawn.clone(),

                // Black back rank (rank 8)
                SquareLabels::A8 | SquareLabels::H8 => piece_sprites.b_rook.clone(),
                SquareLabels::B8 | SquareLabels::G8 => piece_sprites.b_knight.clone(),
                SquareLabels::C8 | SquareLabels::F8 => piece_sprites.b_bishop.clone(),
                SquareLabels::D8 => piece_sprites.b_queen.clone(),
                SquareLabels::E8 => piece_sprites.b_king.clone(),

                // Empty center squares & SquareLabels::None
                _ => continue,
            };

            commands.spawn((
                Sprite::from_image(image),
                Transform::from_xyz(x, y, 1.0),
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
            ));
        }
    }
}

pub struct SetupDefaultBoard;

impl Plugin for SetupDefaultBoard{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_pieces_resource, load_board).chain());
    }
}
