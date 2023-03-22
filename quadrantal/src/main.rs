use std::collections::HashMap;

mod col;
mod file;
mod piece;
mod random;
mod tet;

//~ use piece::Piece;
use piece::FloatingPiece;

use file::load_tileset_from_path;

fn render_to_argb_u32(framebuffer: &[vfc::Rgb; vfc::NUM_SCREEN_PIXELS], target_buffer: &mut [u32]) {
    for (index, argb) in framebuffer.iter().map(|rgb| rgb.as_argb_u32()).enumerate() {
        target_buffer[index] = argb;
    }
}

fn clear_sprites(fc: &mut vfc::Vfc) {
    let range = 0..=63;
    for i in range {
        fc.oam.0[i].y = vfc::SCREEN_HEIGHT as u8;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Action {
    Left,
    Right,
    RotateClockwise,
    RotateAnticlockwise,
    SoftDrop,
    SonicDrop,
    HardDrop,
    Save,
    Start,
    Pause,

    //not-yet-implemented
    //~ Save,

    // debug mode only
    Up,
    Down,
    Lock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    SpawnPiece,
    MainPlay,
    PauseScreen,
    LineClear,
    GameOver,
    Practice,
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
    use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
    use vfc::*;
    use GameState::*;

    let mut fc = Vfc::default();

    //// VFC setup ////////////

    let preview_palette_array = [
        //~ Rgb::new(0x33, 0x77, 0xdd), // background color
        //~ Rgb::new(0xbb, 0xcc, 0xdd), // background color
        //~ Rgb::new(0x11, 0xbb, 0xdd), // background color
        //~ Rgb::new(0xee, 0xee, 0xdd), // background color
        //~ Rgb::new(0x22, 0x33, 0x44),
        //~ Rgb::new(0x44, 0x77, 0x11),
        //~ Rgb::new(0x66, 0xaa, 0x55),
        //~ Rgb::new(0x99, 0xdd, 0x66),
        //~ Rgb::new(0xaa, 0x33, 0x22),
        //~ Rgb::new(0xdd, 0xbb, 0x66),
        //~ Rgb::new(0xff, 0xff, 0xff),

        // bg color
        // gameboy esque palette
        Rgb::new(0xbb, 0xbb, 0x88), // Tan
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0x55, 0x66, 0x66), // Dull Teal
        Rgb::new(0x77, 0x88, 0x44), // Moss Green
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xbb, 0xbb, 0x88), // Tan
        //~ */

        // O Tetromino yellow
        //~ /*
        Rgb::new(0x00, 0x00, 0x00), // unused
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0xdd, 0x99, 0x44), // Orange
        Rgb::new(0xee, 0xee, 0x77), // Yellow
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xee, 0xee, 0xdd), // White
        //~ */

        // I Tetromino blue
        //~ /*
        Rgb::new(0x00, 0x00, 0x00), // unused
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0x77, 0x99, 0xee), // Cerulean
        Rgb::new(0xbb, 0xdd, 0xcc), // Cyan
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xee, 0xee, 0xdd), // White
        //~ */

        // T Tetromino purple
        //~ /*
        Rgb::new(0x00, 0x00, 0x00), // unused
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0x88, 0x66, 0x88), // Dark Purple
        Rgb::new(0xbb, 0x99, 0xaa), // Light Purple
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xee, 0xee, 0xdd), // White
        //~ */

        // J Tetromino blue
        //~ /*
        Rgb::new(0x00, 0x00, 0x00), // unused
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0x44, 0x44, 0x88), // Navy
        Rgb::new(0x77, 0x99, 0xee), // Cerulean
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xee, 0xee, 0xdd), // White
        //~ */

        // L Tetromino orange
        //~ /*
        Rgb::new(0x00, 0x00, 0x00), // unused
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0x88, 0x66, 0x44), // Brown
        Rgb::new(0xdd, 0x99, 0x44), // Orange
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xee, 0xee, 0xdd), // White
        //~ */

        // S Tetromino red
        //~ /*
        Rgb::new(0x00, 0x00, 0x00), // unused
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0xcc, 0x44, 0x33), // Red
        Rgb::new(0xee, 0xbb, 0xaa), // Pink
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xee, 0xee, 0xdd), // White
        //~ */

        // Z Tetromino green
        //~ /*
        Rgb::new(0x00, 0x00, 0x00), // unused
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0x77, 0x88, 0x44), // Moss Green
        Rgb::new(0x99, 0xdd, 0x55), // Lime Green
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xee, 0xee, 0xdd), // White
                                    //~ */

                                    // main palette in order of approximate brightness/lightness/idk luma or something
                                    /*
                                    Rgb::new(0x00, 0x11, 0x11), // Black
                                    Rgb::new(0x44, 0x44, 0x88), // Navy
                                    Rgb::new(0xcc, 0x44, 0x33), // Red
                                    Rgb::new(0x55, 0x66, 0x66), // Dull Teal
                                    Rgb::new(0x88, 0x66, 0x44), // Brown
                                    Rgb::new(0x88, 0x66, 0x88), // Dark Purple
                                    Rgb::new(0x77, 0x88, 0x44), // Moss Green
                                    Rgb::new(0x77, 0x99, 0xee), // Cerulean
                                    Rgb::new(0xdd, 0x99, 0x44), // Orange
                                    Rgb::new(0xbb, 0x99, 0xaa), // Light Purple
                                    Rgb::new(0xbb, 0xbb, 0x88), // Tan
                                    Rgb::new(0x99, 0xdd, 0x55), // Lime Green
                                    Rgb::new(0xee, 0xbb, 0xaa), // Pink
                                    Rgb::new(0xbb, 0xdd, 0xcc), // Cyan
                                    Rgb::new(0xee, 0xee, 0x77), // Yellow
                                    Rgb::new(0xee, 0xee, 0xdd), // White
                                    */
    ];

    fc.palette = Palette::new(
        (0..NUM_PALETTE_ENTRIES)
            .map(|index| {
                if index < (&preview_palette_array[..]).len() {
                    preview_palette_array[index]
                } else {
                    Rgb::default()
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|_| unreachable!()),
    );

    fc.tileset = load_tileset_from_path("quad_tiles.png").unwrap();

    //// minifb Setup ////////////

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        "Quadrantal (debug - hold Escape to exit)",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Limit to max ~640 fps cause why not
    //~ window.limit_update_rate(Some(std::time::Duration::from_micros(1612)));

    //// timing ////

    let mut start_time;

    let mut frametime_hist = std::collections::VecDeque::from(vec![]);

    for _ in 0..5 {
        frametime_hist.push_back(0);
    }

    let mut frames = 0;

    let mut average_fps;

    //----\\ INPUT //----\\

    let mut keybinds = HashMap::new();

    keybinds.insert(Key::Left, Action::Left);
    keybinds.insert(Key::Right, Action::Right);
    keybinds.insert(Key::Up, Action::SonicDrop);
    keybinds.insert(Key::Down, Action::SoftDrop);
    keybinds.insert(Key::X, Action::RotateClockwise);
    keybinds.insert(Key::Z, Action::RotateAnticlockwise);
    keybinds.insert(Key::D, Action::HardDrop);
    keybinds.insert(Key::Enter, Action::Start);
    keybinds.insert(Key::Escape, Action::Pause);
    keybinds.insert(Key::C, Action::Save);
    keybinds.insert(Key::LeftShift, Action::Save);

    // TODO: disable/remove debug move commands
    keybinds.insert(Key::J, Action::Left);
    keybinds.insert(Key::L, Action::Right);
    keybinds.insert(Key::I, Action::Up);
    keybinds.insert(Key::K, Action::Down);
    keybinds.insert(Key::Space, Action::Lock);

    let mut key_pressed = HashMap::new();
    let mut key_down = HashMap::new();

    //----\\ INITIAL GAME STATE //----\\

    let debug_output = false;

    let mut score = 0;
    let mut lines_cleared = 0;
    let mut pieces_dropped = 0;
    let mut lines_cleared_per_piece = [0; 8];

    let mut bag = random::Bag::default();
    // 2 is the nunmber of sets of all pieces to mix together

    // incremented on rotation, if it's four or more then rotating doesn't reset the gravity timer
    let mut rotate_counter = 0;

    // incremented on switching from moving left to right
    // if it's four or more then moving laterally doesn't reset the gravity timer
    let mut x_direction_switch_counter = 0;

    let mut last_x_direction = 0;

    let gravity_delay = 50; // time before a piece moves down, in frames
    let mut gravity_delay_counter = gravity_delay;

    let line_clear_delay = 24;
    //~ let line_clear_delay = 28;
    let mut line_clear_timer = line_clear_delay;

    let mut full_rows = vec![];

    tet::init_playfield(&mut fc);

    let mut controlled_piece =
        FloatingPiece::new(piece::Piece::new_basic(0, Subpalette::new(0)), (1, 1));

    let mut saved_piece: Option<piece::Piece> = None;

    let mut already_swapped = false;

    let mut game_state = SpawnPiece;

    let mut play_state = MainPlay;

    //----\\ MAIN LOOP //----\\

    let mut emergency_exit = 100;

    'main: while window.is_open() && !window.is_key_down(Key::Backspace) {
        //----\\ SOME TIMING //----\\
        start_time = std::time::Instant::now();

        //----\\ INPUT //----\\

        // debug input //

        if window.is_key_down(Key::Escape) {
            emergency_exit -= 1;

            if emergency_exit <= 0 {
                break 'main;
            }
        } else {
            emergency_exit = 100;
        }

        if window.is_key_pressed(Key::F6, KeyRepeat::No) {
            if play_state == Practice {
                play_state = MainPlay;
            } else {
                play_state = Practice;
                if game_state == MainPlay {
                    game_state = Practice;
                }
            }
        }

        if window.is_key_pressed(Key::F8, KeyRepeat::No) {
            //~ std::mem::swap(&mut fc.oam, &mut hidden_oam);
            fc.oam_hidden = !fc.oam_hidden;
        }

        if window.is_key_pressed(Key::F9, KeyRepeat::No) {
            fc.bg_layers[0].hidden = !fc.bg_layers[0].hidden;
        }

        if window.is_key_pressed(Key::F10, KeyRepeat::No) {
            fc.bg_layers[1].hidden = !fc.bg_layers[1].hidden;
        }

        // Game Input //

        key_pressed.insert(Action::Left, false);
        key_pressed.insert(Action::Right, false);
        key_pressed.insert(Action::RotateClockwise, false);
        key_pressed.insert(Action::RotateAnticlockwise, false);
        key_pressed.insert(Action::SoftDrop, false);
        key_pressed.insert(Action::SonicDrop, false);
        key_pressed.insert(Action::HardDrop, false);
        key_pressed.insert(Action::Save, false);
        key_pressed.insert(Action::Pause, false);
        key_pressed.insert(Action::Start, false);

        // TODO: disable/remove debug move commands
        key_pressed.insert(Action::Up, false);
        key_pressed.insert(Action::Down, false);
        key_pressed.insert(Action::Lock, false);

        key_down.insert(Action::Left, false);
        key_down.insert(Action::Right, false);
        key_down.insert(Action::RotateClockwise, false);
        key_down.insert(Action::RotateAnticlockwise, false);
        key_down.insert(Action::SoftDrop, false);
        key_down.insert(Action::SonicDrop, false);
        key_down.insert(Action::HardDrop, false);
        key_down.insert(Action::Save, false);
        key_down.insert(Action::Pause, false);
        key_down.insert(Action::Start, false);

        // TODO: disable/remove debug move commands
        key_down.insert(Action::Up, false);
        key_down.insert(Action::Down, false);
        key_down.insert(Action::Lock, false);

        for (key, action) in keybinds.iter() {
            // this may seem complicated but it's how we let two keys do the same action
            let pressed = window.is_key_pressed(*key, KeyRepeat::No);

            if let Some(already_pressed) = key_pressed.get(action) {
                key_pressed.insert(*action, pressed || *already_pressed);
                //~ ()
            } else {
                panic!("action not inserted {:?}", action)
            };

            let down = window.is_key_down(*key);

            if let Some(already_down) = key_down.get(action) {
                key_down.insert(*action, down || *already_down);
                //~ ()
            } else {
                panic!("action not inserted {:?}", action)
            };
        }

        //----\\ LOGIC //----\\

        let rotation_vector = {
            let left = *key_pressed.get(&Action::RotateAnticlockwise).unwrap();
            let right = *key_pressed.get(&Action::RotateClockwise).unwrap();

            if left && !right {
                -1
            } else if right && !left {
                1
            } else {
                0
            }
        };

        let horizontal_move_vector = {
            let left = *key_pressed.get(&Action::Left).unwrap();
            let right = *key_pressed.get(&Action::Right).unwrap();

            let horizontal = if left && !right {
                -1
            } else if right && !left {
                1
            } else {
                0
            };

            horizontal
        };

        let vertical_move_vector = {
            let up = *key_pressed.get(&Action::Up).unwrap();
            let down = *key_pressed.get(&Action::Down).unwrap();

            let vertical = if up && !down {
                -1
            } else if down && !up {
                1
            } else {
                0
            };

            vertical
        };

        let horizontal_move_repeat = {
            let up = *key_down.get(&Action::Up).unwrap();
            let down = *key_down.get(&Action::Down).unwrap();

            let vertical = if up && !down {
                -1
            } else if down && !up {
                1
            } else {
                0
            };

            vertical
        };

        //~ let horizontal_move_vector = -1;

        let start_button = *key_pressed.get(&Action::Start).unwrap();
        let pause_action = *key_pressed.get(&Action::Pause).unwrap() || start_button;

        let hard_drop = *key_pressed.get(&Action::HardDrop).unwrap();
        let sonic_drop = *key_pressed.get(&Action::SonicDrop).unwrap();
        let soft_drop = *key_pressed.get(&Action::SoftDrop).unwrap();

        let save_piece = *key_pressed.get(&Action::Save).unwrap();

        let debug_force_lock = *key_pressed.get(&Action::Lock).unwrap();

        //// State-specific Logic ////

        match &game_state {
            Practice => todo!(),
            MainPlay => {
                // pause menu //

                if pause_action {
                    game_state = PauseScreen;
                }

                // do rotation //

                let rotate_new_position = if rotation_vector != 0 {
                    controlled_piece.try_rotate_with_kicks(&fc, rotation_vector)
                } else {
                    None
                };

                let rotate_success = match rotate_new_position {
                    Some(pos) => {
                        controlled_piece.rotate_move(rotation_vector, pos);
                        true
                    }
                    None => false,
                };

                // do horizontal movement //

                let horizontal_move_success =
                    controlled_piece.try_move(&fc, (horizontal_move_vector, 0));

                if rotation_vector != 0 && rotate_success {
                    if rotate_counter < 4 {
                        gravity_delay_counter = gravity_delay;
                    }

                    rotate_counter += 1;
                }

                if horizontal_move_vector != 0 && horizontal_move_success {
                    if x_direction_switch_counter < 2 {
                        gravity_delay_counter = gravity_delay;
                    }

                    if horizontal_move_vector != last_x_direction {
                        x_direction_switch_counter += 1;
                    }

                    last_x_direction = horizontal_move_vector;
                }

                // do vertical movement if in debug mode //

                let vertical_move_success =
                    controlled_piece.try_move(&fc, (0, vertical_move_vector));

                if vertical_move_vector != 0 && vertical_move_success {
                    panic!()
                };

                // save piece //

                if save_piece && !already_swapped {
                    let next_saved_piece = match saved_piece {
                        Some(old_saved_piece) => {
                            let p = old_saved_piece.clone();

                            let next_saved_piece = controlled_piece.get_unrotated_piece().clone();

                            controlled_piece.reset(p);

                            Some(next_saved_piece)
                        }
                        None => {
                            let next_saved_piece = controlled_piece.get_unrotated_piece().clone();

                            controlled_piece.reset(bag.next());

                            if controlled_piece.get_piece().test_collision(
                                &fc,
                                controlled_piece.position().0,
                                controlled_piece.position().1,
                            ) {
                                let pc = controlled_piece.get_piece();
                                let position = controlled_piece.position();
                                pc.force_lock_xor(
                                    &mut fc,
                                    position.0,
                                    position.1,
                                    TileIndex(TILE_GAME_OVER_BLOCK.0 + 1),
                                    TILE_GAME_OVER_BLOCK,
                                );

                                game_state = GameOver;
                            }

                            Some(next_saved_piece)
                        }
                    };

                    saved_piece = next_saved_piece;

                    already_swapped = true;
                }

                // do various drops //

                if sonic_drop || hard_drop {
                    if controlled_piece.sonic_drop(&fc) {
                        gravity_delay_counter = gravity_delay;
                    }

                    rotate_counter = 0;
                    x_direction_switch_counter = 0;
                }

                if soft_drop || hard_drop || gravity_delay_counter <= 0 {
                    let locked = if debug_force_lock {
                        controlled_piece.lock(&mut fc);

                        true
                    } else {
                        controlled_piece.soft_drop(&mut fc)
                    };

                    if locked {
                        use tet::*;

                        already_swapped = false;

                        // check for lines
                        'l: for row in 0..FIELD_HEIGHT {
                            for column in 0..FIELD_WIDTH {
                                let tile = peek_game_layer(&fc, FIELD_X + column, FIELD_Y + row);

                                if !col::tile_is_block(tile) {
                                    continue 'l;
                                }
                            }

                            full_rows.push(row);
                        }

                        pieces_dropped += 1;

                        lines_cleared += full_rows.len();

                        let pid = controlled_piece.get_piece().index();

                        lines_cleared_per_piece[pid] += full_rows.len();

                        score += if full_rows.is_empty() {
                            1
                        } else {
                            let mut this_score = 5;

                            for _ in 0..full_rows.len() {
                                this_score *= 2
                            }

                            this_score
                        };

                        game_state = if !full_rows.is_empty() {
                            LineClear
                        } else {
                            SpawnPiece
                        };
                    }

                    gravity_delay_counter = gravity_delay;

                    rotate_counter = 0;
                    x_direction_switch_counter = 0;
                }

                gravity_delay_counter -= 1;
            }
            PauseScreen => {
                if pause_action {
                    game_state = play_state;
                }
            }
            SpawnPiece => {
                gravity_delay_counter = gravity_delay;

                rotate_counter = 0;
                x_direction_switch_counter = 0;

                let next_piece = bag.next();

                game_state = if controlled_piece.reset_and_test_overlap(&mut fc, next_piece) {
                    GameOver
                } else {
                    play_state
                };
            }
            LineClear => {
                use col::*;
                use tet::*;

                // get them out of the way so we can see our nice line-clearing animation
                // (they're locked in anyway so this is more correct in general)
                //clear_sprites(&mut fc);

                // animate the rows
                let animation_frames = 8;

                let frame =
                    (line_clear_delay - line_clear_timer) / (line_clear_delay / animation_frames);

                for row in full_rows.iter() {
                    for column in 0..FIELD_WIDTH {
                        poke_game_layer(
                            &mut fc,
                            FIELD_X + column,
                            FIELD_Y + row,
                            TileIndex(TILE_ROW_CLEAR.0 + frame),
                        );
                        poke_game_layer_palette(
                            &mut fc,
                            FIELD_X + column,
                            FIELD_Y + row,
                            Subpalette::new(0),
                        );
                    }
                }

                line_clear_timer -= 1;

                if line_clear_timer == 0 {
                    for full_row in full_rows.iter() {
                        for row in (0..*full_row).rev() {
                            for column in 0..FIELD_WIDTH {
                                let current_row = row + 1;
                                let next_row = row;

                                let next_tile =
                                    peek_game_layer(&mut fc, FIELD_X + column, FIELD_Y + next_row);

                                let next_tile_palette = peek_game_layer_palette(
                                    &mut fc,
                                    FIELD_X + column,
                                    FIELD_Y + next_row,
                                );

                                let current_tile = peek_game_layer(
                                    &mut fc,
                                    FIELD_X + column,
                                    FIELD_Y + current_row,
                                );

                                if tile_is_wall(next_tile) {
                                    panic!()
                                }
                                if tile_is_wall(current_tile) {
                                    panic!()
                                }

                                let next_tile = if current_tile == TILE_CEILING {
                                    if col::tile_is_empty(next_tile) {
                                        current_tile
                                    } else {
                                        next_tile
                                    }
                                } else {
                                    if col::tile_is_empty(next_tile) {
                                        TILE_EMPTY
                                    } else {
                                        next_tile
                                    }
                                };

                                poke_game_layer(
                                    &mut fc,
                                    FIELD_X + column,
                                    FIELD_Y + current_row,
                                    next_tile,
                                );

                                poke_game_layer_palette(
                                    &mut fc,
                                    FIELD_X + column,
                                    FIELD_Y + current_row,
                                    next_tile_palette,
                                );
                            }
                        }
                    }

                    for xi in 0..FIELD_WIDTH {
                        let x = FIELD_X + xi;
                        let y = FIELD_Y + CEILING_HEIGHT;
                        let bg_tile = peek_game_layer(&mut fc, x, y);
                        if tile_is_empty(bg_tile) {
                            poke_game_layer(&mut fc, x, y, TILE_CEILING);
                        }
                    }

                    line_clear_timer = line_clear_delay;

                    full_rows.clear();

                    game_state = SpawnPiece;
                }
            }
            GameOver => {
                if start_button {
                    score = 0;
                    lines_cleared = 0;
                    pieces_dropped = 0;

                    for i in lines_cleared_per_piece.iter_mut() {
                        *i = 0;
                    }

                    saved_piece = None;

                    tet::clear_playfield(&mut fc);
                    tet::clear_text_layer(&mut fc);

                    bag = random::Bag::default();
                    let next = bag.next();

                    controlled_piece.reset(next);

                    already_swapped = false;

                    gravity_delay_counter = gravity_delay;

                    game_state = play_state;
                }
            } //~ Restart => {
              //~ }
        }

        //----\\ RENDERING //----\\

        // Bag display
        {
            use tet::*;

            let peek_length = 5;

            // draw next pieces
            let p = bag.peek(peek_length).unwrap();

            for xi in (0..peek_length).rev() {
                if xi >= p.len() {
                    break;
                };

                poke_game_layer(
                    &mut fc,
                    FIELD_X + xi + FIELD_WIDTH - peek_length,
                    FIELD_Y + TOP_VISIBLE_ROW,
                    TileIndex(TILE_PIECE_ICON.0 + p[xi].index() as u8),
                );
            }

            //draw fps
            /*
            let x = FIELD_X + FIELD_WIDTH + 2;
            let y = 0;

            for xi in 0..4 {
                poke_game_layer(&mut fc, x + xi, y, TileIndex(TILE_PIECE_ICON.0 + p[xi].index() as u8));
            }

            let fps_string = format!("{average_fps:4}");

            draw_text(0, &mut fc, x, y, &fps_string)
            */
        }

        // swap display
        if let Some(ref p) = saved_piece {
            poke_menu_layer(
                &mut fc,
                FIELD_X,
                FIELD_Y + TOP_VISIBLE_ROW,
                TileIndex(TILE_PIECE_ICON.0 + p.index() as u8),
            );
        }

        clear_sprites(&mut fc);

        use tet::*;

        //~ tet::draw_text(1, &mut fc, FIELD_X - 1, SCORE_Y, if game_state == PauseScreen { "PAUSE" } else { "SCORE" });
        tet::draw_text(
            1,
            &mut fc,
            FIELD_X - 1,
            SCORE_Y,
            if game_state == PauseScreen {
                "PAUSE"
            } else {
                "SCORE"
            },
        );
        tet::draw_text(0, &mut fc, FIELD_X - 1, SCORE_Y, " ");
        tet::draw_text(0, &mut fc, FIELD_X + FIELD_WIDTH, SCORE_Y, " ");

        let t = &format!("{}", score);
        let len = t.len();
        tet::draw_text(1, &mut fc, FIELD_X + FIELD_WIDTH + 1 - len, SCORE_Y, t);

        let stats = [("LINES", lines_cleared), ("PIECES", pieces_dropped)];

        let y = 5;

        for (i, (label, number)) in stats.iter().enumerate() {
            tet::draw_text(
                1,
                &mut fc,
                FIELD_X.wrapping_sub(7) % 32,
                SCORE_Y + y + 4 * i,
                label,
            );
            let t = &format!("{}", number);
            let len = t.len();
            tet::draw_text(
                1,
                &mut fc,
                FIELD_X.wrapping_sub(len + 1) % 32,
                SCORE_Y + y + 4 * i + 2,
                t,
            );
        }

        tet::draw_text(
            1,
            &mut fc,
            FIELD_X.wrapping_add(FIELD_WIDTH + 1) % 32,
            SCORE_Y + 1,
            "LINES",
        );
        tet::draw_text(
            1,
            &mut fc,
            FIELD_X.wrapping_add(FIELD_WIDTH + 1) % 32,
            SCORE_Y + 2,
            " PER ",
        );
        tet::draw_text(
            1,
            &mut fc,
            FIELD_X.wrapping_add(FIELD_WIDTH + 1) % 32,
            SCORE_Y + 3,
            "PIECE",
        );

        for i in 1..=7 {
            let (x, y) = (FIELD_X.wrapping_add(FIELD_WIDTH + 1) % 32, SCORE_Y + i + 4);

            poke_bg(1, &mut fc, x, y, TileIndex(TILE_PIECE_ICON.0 + i as u8));

            let t = &format!("{}", lines_cleared_per_piece[i]);
            let len = t.len();
            tet::draw_text(1, &mut fc, x.wrapping_sub(len).wrapping_add(6) % 32, y, t);
        }

        match &game_state {
            GameOver => {
                tet::draw_text(0, &mut fc, FIELD_X, FIELD_Y + TOP_VISIBLE_ROW, "GAME  OVER")
            }
            LineClear | SpawnPiece => (),
            MainPlay | PauseScreen | Practice => {
                let shadow_pos = controlled_piece.shadow_drop(&fc);

                let offset = 0;

                // draw floating piece
                let offset = controlled_piece.get_piece().draw_as_sprites(
                    &mut fc,
                    controlled_piece.position().0 * 8,
                    controlled_piece.position().1 * 8,
                    offset,
                    0,
                    None,
                );

                // draw piece shadow
                let offset = controlled_piece.get_piece().draw_as_sprites(
                    &mut fc,
                    shadow_pos.0 * 8,
                    shadow_pos.1 * 8,
                    offset,
                    tet::TILE_SHADOW_OFFSET.0,
                    Some(Subpalette::new(0)),
                );

                // draw big next piece
                //~ let offset = bag
                //~     .peek_next()
                //~     .draw_as_sprites(&mut fc, 15 * 8, 1 * 8, offset, 0);

                // draw swap piece
                //~ let offset = bag.peek_next().draw_as_sprites(&mut fc, 15 * 8, 1 * 8, offset, 0);

                let _offset = offset;
            }
        }

        //~ eprintln!("{}", gravity_delay_counter);

        fc.render_frame();

        render_to_argb_u32(&mut fc.framebuffer, &mut buffer);

        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();

        let end_time = std::time::Instant::now();

        //----\\ MORE TIMING //----\\

        let instant_microsecs_per_frame = (end_time - start_time).as_micros();

        frametime_hist.pop_front().expect("fifo wrong length");

        frametime_hist.push_back(instant_microsecs_per_frame);

        let mut average_frametime = 0;
        for i in 0..5 {
            average_frametime += frametime_hist[i];
        }
        average_frametime /= 5;

        average_fps = 1_000_000 / average_frametime;

        if debug_output {
            eprintln!(
                "{instant_microsecs_per_frame:?}\t{average_frametime:?}\t{average_fps}\t{frames}\n{score}"
            );
        }

        frames += 1;
    }
}
