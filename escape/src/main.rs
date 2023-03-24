use std::collections::HashMap;
use std::collections::HashSet;

use hli::vfc;
use hli::vfc::Vfc;
use hli::fc;
use hli::file;
use hli::random;
use hli::vector;

mod game;
use game::Game;

//~ mod fc;
//~ mod file;
//~ mod random;
//~ mod vector;

use file::load_tileset_from_path;

use vector::Vector;

use random::shuffle;

const GAME_NAME: &'static str = "Escape from Castle Dracula";

fn render_to_argb_u32(framebuffer: &[vfc::Rgb; vfc::NUM_SCREEN_PIXELS], target_buffer: &mut [u32]) {
    for (index, argb) in framebuffer.iter().map(|rgb| rgb.as_argb_u32()).enumerate() {
        target_buffer[index] = argb;
    }
}

fn clear_sprites(fc: &mut Vfc) {
    let range = 0..=63;
    for i in range {
        fc.oam.0[i].y = vfc::SCREEN_HEIGHT as u8;
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Action {
    Left,
    Right,
    Up,
    Down,
    Fire,
}

struct EngineState {
    pub fc: Vfc,
    pub input: Input,
}

#[derive(Default)]
struct Input {
    key_binds: HashMap<minifb::Key, Action>,
    pressed: HashMap<Action, bool>,
    held: HashMap<Action, bool>,
}

////====////====\\\\====\\\\ Main ////====////====\\\\====\\\\

fn main() {
    use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
    use vfc::*;

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

        // gameboy bg color
        //~ Rgb::new(0xbb, 0xbb, 0x88), // Tan
        //~ Rgb::new(0x00, 0x11, 0x11), // Black
        //

        // gameboy esque palette

        //~ /*
        //~ Rgb::new(0xaa, 0x55, 0xaa), // Black (background)
        Rgb::new(0x00, 0x11, 0x11), // Black (background)
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0xcc, 0x44, 0x33), // Red
        Rgb::new(0x77, 0x99, 0xee), // Cerulean
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xbb, 0xbb, 0x88), // Tan
        //~ */

        //~ /*
        Rgb::new(0x99, 0x00, 0x99), // [placeholder] (transparent)
        Rgb::new(0x00, 0x11, 0x11), // Black
        Rgb::new(0xdd, 0x99, 0x44), // Orange
        Rgb::new(0x55, 0x66, 0x66), // Dull Teal
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0x99, 0x00, 0x99), // [placeholder]
        Rgb::new(0xbb, 0xbb, 0x88), // Tan
        //~ */

        // main palette in order of approximate brightness/lightness/idk luma or something
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

    fc.tileset = load_tileset_from_path("hunt/hunt_tiles.png").unwrap();

    //// minifb Setup ////////////

    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut window = Window::new(
        &(GAME_NAME.to_owned() + " " + "(debug - hold Escape to exit)"),
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

    let mut key_binds = HashMap::new();

    key_binds.insert(Key::Left, Action::Left);
    key_binds.insert(Key::Right, Action::Right);
    key_binds.insert(Key::Up, Action::Up);
    key_binds.insert(Key::Down, Action::Down);
    key_binds.insert(Key::Space, Action::Fire);

    key_binds.insert(Key::A, Action::Left);
    key_binds.insert(Key::D, Action::Right);
    key_binds.insert(Key::W, Action::Up);
    key_binds.insert(Key::S, Action::Down);
    key_binds.insert(Key::Enter, Action::Fire);

    let mut pressed = HashMap::new();
    let mut held = HashMap::new();

    let mut engine_state = EngineState {
        fc,
        input: Input { key_binds, pressed, held },
    };

    //----\\ INITIAL GAME STATE //----\\

    let mut game = Game::new();

    //----\\ MAIN LOOP //----\\

    let mut emergency_exit = 100;

    let debug_allowed = true;
    let mut debug_mode = false;
    let debug_output = false;

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

        if debug_allowed {
            if window.is_key_pressed(Key::F7, KeyRepeat::No) {
                debug_mode = !debug_mode;
            }
        }

        if debug_mode {
            if window.is_key_pressed(Key::F8, KeyRepeat::No) {
                //~ std::mem::swap(&mut fc.oam, &mut hidden_oam);
                engine_state.fc.oam_hidden = !engine_state.fc.oam_hidden;
            }

            if window.is_key_pressed(Key::F9, KeyRepeat::No) {
                engine_state.fc.bg_layers[0].hidden = !engine_state.fc.bg_layers[0].hidden;
            }

            if window.is_key_pressed(Key::F10, KeyRepeat::No) {
                engine_state.fc.bg_layers[1].hidden = !engine_state.fc.bg_layers[1].hidden;
            }
        }

        // Game Input //

        // reset
        {
            use Action::*;
            let list = [Left, Right, Up, Down, Fire];

            for action in list {
                engine_state.input.pressed.insert(action, false);
                engine_state.input.held.insert(action, false);
            }
        }

        for (key, action) in engine_state.input.key_binds.iter() {
            let pressed = window.is_key_pressed(*key, KeyRepeat::No);

            // this may seem complicated but it's how we let two keys do the same action
            if let Some(already_pressed) = engine_state.input.pressed.get(action) {
                engine_state.input.pressed.insert(*action, pressed || *already_pressed);
                //~ ()
            } else {
                panic!("action not inserted {:?}", action)
            };

            let down = window.is_key_down(*key);

            if let Some(already_down) = engine_state.input.held.get(action) {
                engine_state.input.held.insert(*action, down || *already_down);
                //~ ()
            } else {
                panic!("action not inserted {:?}", action)
            };
        }

        //----\\ LOGIC //----\\

        game.tick(&mut engine_state);

        //----\\ RENDERING //----\\

        game.render(&mut engine_state);

        //----\\ MORE RENDERING //----\\

        engine_state.fc.render_frame();

        render_to_argb_u32(&mut engine_state.fc.framebuffer, &mut buffer);

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
                "{instant_microsecs_per_frame:?}\t{average_frametime:?}\t{average_fps}\t{frames}"
            );
        }

        frames += 1;
    }
}
