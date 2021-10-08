/**
 * Chess GUI template.
 * Author: Viola Söderlund <violaso@kth.se>
 * Last updated: 2021-10-03
 */

use ggez::{conf, event, graphics, ContextBuilder, Context, GameError, GameResult};
use std::{path, env, collections::HashMap};
use murnion_chess::{Game, Colour, Piece};

/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color = graphics::Color::new(228.0/255.0, 196.0/255.0, 108.0/255.0, 1.0);
const WHITE: graphics::Color = graphics::Color::new(188.0/255.0, 140.0/255.0, 76.0/255.0, 1.0);
const SELECTED: graphics::Color = graphics::Color::new(0.0/255.0, 140.0/255.0, 10.0/255.0, 0.8);
const HIGHLIGHTED: graphics::Color = graphics::Color::new(0.0/255.0, 140.0/255.0, 10.0/255.0, 0.3);

/// GUI logic and event implementation structure. 
struct AppState {
    sprites: Vec<(Piece, graphics::Image)>,
    game: Game,
    // Save piece positions, which tiles has been clicked, current colour, etc...
    selected_square: Option<(usize,usize)>,
    highlighted_squares: Vec<(usize,usize)>
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {

        
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            game: Game::new(),
            selected_square: None,
            highlighted_squares: vec![]
        };

        Ok(state)
    }

    /// Loads chess piese images into vector.
    fn load_sprites(ctx: &mut Context) -> Vec<(Piece, graphics::Image)> {

        [
            (Piece::King(Colour::Black), "/black_king.png".to_string()),
            (Piece::Queen(Colour::Black), "/black_queen.png".to_string()),
            (Piece::Rook(Colour::Black), "/black_rook.png".to_string()),
            (Piece::Pawn(Colour::Black), "/black_pawn.png".to_string()),
            (Piece::Bishop(Colour::Black), "/black_bishop.png".to_string()),
            (Piece::Knight(Colour::Black), "/black_knight.png".to_string()),
            (Piece::King(Colour::White), "/white_king.png".to_string()),
            (Piece::Queen(Colour::White), "/white_queen.png".to_string()),
            (Piece::Rook(Colour::White), "/white_rook.png".to_string()),
            (Piece::Pawn(Colour::White), "/white_pawn.png".to_string()),
            (Piece::Bishop(Colour::White), "/white_bishop.png".to_string()),
            (Piece::Knight(Colour::White), "/white_knight.png".to_string())
        ]
        .iter()
        .map(|(_piece, _path)| (*_piece, graphics::Image::new(ctx, _path).unwrap()))
        .collect::<Vec<(Piece, graphics::Image)>>()
    }
}

impl event::EventHandler<GameError> for AppState {

    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());


        // draw grid
        for _row in 0..8 {
            for _col in 0..8 {

                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new_i32(
                        _col * GRID_CELL_SIZE.0 as i32,
                        _row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ), match _col % 2 {
                        0 => 
                            if _row % 2 == 0 { WHITE } 
                            else { BLACK },
                        _ => 
                            if _row % 2 == 0 { BLACK } 
                            else { WHITE },
                    }).expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default()).expect("Failed to draw tiles.");

                // draw piece
                if self.game.board[_row as usize][_col as usize] != Piece::Empty {
                    graphics::draw(ctx, &self.sprites.get(self.sprites.iter().position(|p| p.0 == self.game.board[_row as usize][_col as usize]).unwrap()).unwrap().1, graphics::DrawParam::default()
                        .scale([2.0, 2.0])  // Tile size is 90 pixels, while image sizes are 45 pixels.
                        .dest(
                            [_col as f32 * GRID_CELL_SIZE.0 as f32, _row as f32 * GRID_CELL_SIZE.1 as f32],
                        )
                    ).expect("Failed to draw piece.");
                }
            }
        }

        if let Some(s) = self.selected_square {
            // draw selected square
            let rectangle = graphics::Mesh::new_rectangle(ctx, 
                graphics::DrawMode::fill(), 
                graphics::Rect::new_i32(
                    s.1 as i32 * GRID_CELL_SIZE.0 as i32,
                    s.0 as i32 * GRID_CELL_SIZE.1 as i32,
                    GRID_CELL_SIZE.0 as i32,
                    GRID_CELL_SIZE.1 as i32,
                ), 
                SELECTED
                ).expect("Failed to create tile.");
            graphics::draw(ctx, &rectangle, graphics::DrawParam::default()).expect("Failed to draw tiles.");

            // draw highlighted squares
            for squ in self.highlighted_squares.iter() {
                let rectangle = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new_i32(
                        squ.1 as i32 * GRID_CELL_SIZE.0 as i32,
                        squ.0 as i32 * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ), 
                    HIGHLIGHTED
                    ).expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default()).expect("Failed to draw tiles.");
            }
        }
        
        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }

    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        if button == event::MouseButton::Left {
            /* check click position and update board accordingly */
            let rank = (y / GRID_CELL_SIZE.1 as f32).floor() as usize;
            let file = (x / GRID_CELL_SIZE.0 as f32).floor() as usize;
            match self.selected_square {
                Some(pos) => {
                    if pos == (rank, file) {
                        self.selected_square = None;
                        self.highlighted_squares = Vec::new();
                    } else if self.highlighted_squares.iter().any(|p| p == &(rank,file)) {
                        self.game.take_turn(move_string(pos, (rank,file)));
                        self.selected_square = None;
                        self.highlighted_squares = Vec::new();
                    } else {
                        self.selected_square = Some((rank, file));
                        self.highlighted_squares = Vec::new();
                        if let Some(c) = get_colour(self.game.board[rank][file]) {
                            if c == self.game.current_turn {
                                self.highlighted_squares = self.game.board[rank][file].get_valid_moves( (rank, file), &self.game.board, self.game.en_passant_square, self.game.castlings, self.game.current_turn);
                            };
                        };
                    }
                },
                None => {
                    self.selected_square = Some((rank, file));
                    self.highlighted_squares = Vec::new();
                    if let Some(c) = get_colour(self.game.board[rank][file]) {
                        if c == self.game.current_turn {
                            self.highlighted_squares = self.game.board[rank][file].get_valid_moves( (rank, file), &self.game.board, self.game.en_passant_square, self.game.castlings, self.game.current_turn);
                        };
                    };
                }
            }
        }
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        if keycode == event::KeyCode::Escape {
            event::quit(ctx);
        } else if keycode == event::KeyCode::R {
            self.game = Game::new();
            self.selected_square = None;
            self.highlighted_squares = Vec::new();
        }
    }
}

pub fn main() -> GameResult {

    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("schack", "viola")
        .add_resource_path(resource_dir)        // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()  
                .title("Schack")                // Set window title "Schack"
                .icon("/icon.png")              // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false)               // Fixate window size
        );
    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");

    let state = AppState::new(&mut contex).expect("Failed to create state.");
    event::run(contex, event_loop, state)       // Run window event loop
}

fn get_colour(piece: Piece) -> Option<Colour> {
    match piece {
        Piece::King(c)
        | Piece::Queen(c)
        | Piece::Rook(c)
        | Piece::Knight(c)
        | Piece::Bishop(c)
        | Piece::Pawn(c) => Some(c),
        Piece::Empty => None,
    }
}

fn move_string(_from: (usize, usize), _to: (usize, usize)) -> String  {
    let mut string = String::new();

    string.push(match _from.1 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("File wrong")
    });
    match _from.0 {
        0..=7 => string.push(char::from_digit(8 - _from.0 as u32, 10).unwrap()),
        _ => panic!("Rank wrong"),
    };
    string.push(' ');
    string.push(match _to.1 {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => panic!("File wrong")
    });
    match _to.0 {
        0..=7 => string.push(char::from_digit(8 - _to.0 as u32, 10).unwrap()),
        _ => panic!("Rank wrong"),
    };

    string
}