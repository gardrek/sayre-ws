use std::collections::HashMap;
use std::collections::HashSet;

use vfc::Vfc;
use hli::fc;
use hli::file;
use hli::random;
use hli::vector;

//~ mod fc;
//~ mod file;
//~ mod random;
//~ mod vector;

use file::load_tileset_from_path;

use vector::Vector;

use random::shuffle;

const TILE_FEATURE_ICONS: usize = 0x01;
const TILE_MAP_GRID: usize = 0xb5;
const TILE_MAP_DOT: usize = 0x80;

const MAP_X: usize = 9;
const MAP_Y: usize = 11;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Restart,
    ShowRoom,
    MainPlay,
    FireSelect,
    DraculaAlerted,
    FireContinue,
    Die,
    GameOver,
}

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;

struct Game {
    state: GameState,
    map: Map,
    player: Player,
    rng: random::Prng,
    displayed_features: HashSet<RoomFeature>,
}

impl Game {
    /*
    pub fn change_state(&mut self, fc: &mut Vfc, new_state: GameState) {
        use GameState::*;

        match new_state {
            GameOver => (),
            _ => (),
        }

        self.state = new_state;
    }
    */

    pub fn update_features(&mut self) {
        self.displayed_features = self.map.near_rooms_features(self.player.coords);
    }

    pub fn move_dracula(&mut self) {
        let old_coords = self.map.get_dracula_coords();

        let n = self.rng.next().unwrap();

        let potential_rooms = self
            .map
            .near_rooms_coords(old_coords)
            .into_iter()
            .filter(|coords| {
                let room = self.map.get_room(*coords);
                room.is_some() && room.unwrap().feature.is_none()
            })
            .collect::<Vec<_>>();

        let count = potential_rooms.len();

        let potential_room = potential_rooms
            .iter()
            .enumerate()
            .filter(|(index, _)| *index == (n as usize) % count)
            .map(|(_, room)| room)
            .next();

        let coords = match potential_room {
            Some(c) => *c,
            None => old_coords,
        };

        self.map.set_room_feature(old_coords, None);
        self.map
            .set_room_feature(coords, Some(RoomFeature::Dracula));
    }

    pub fn displayed_features(&self) -> &HashSet<RoomFeature> {
        &self.displayed_features
    }
}

struct Map {
    //~ data: [[Room; MAP_HEIGHT]; MAP_WIDTH],
    data: Vec<Room>,
}

impl Map {
    pub fn new(rng: &mut random::Prng) -> Map {
        use RoomFeature::*;

        let mut v = vec![];

        //~ let mut features = vec![Bat, Bat, Bat, Pit, Pit, Arrow, Dracula, PlayerStart];
        let features = vec![Bat, Bat, Bat, Pit, Pit, Dracula, PlayerStart];

        let len = features.len();

        for h in features {
            v.push(Some(h));
        }

        for _ in len..(MAP_HEIGHT * MAP_WIDTH) {
            v.push(None);
        }

        shuffle(rng, &mut v);

        let data = v
            .into_iter()
            .map(|feature| Room { feature })
            .collect::<Vec<_>>();

        Map { data }
    }

    pub fn in_bounds(&self, coords: Vector<2, isize>) -> bool {
        let [x, y] = coords.0;

        x >= 0 && x < MAP_WIDTH as isize && y >= 0 && y < MAP_HEIGHT as isize
    }

    pub fn get_room(&self, coords: Vector<2, isize>) -> Option<&Room> {
        if !self.in_bounds(coords) {
            return None;
        }

        let index = Map::index_from_coords(coords);

        Some(&self.data[index])
    }

    pub fn get_room_mut(&mut self, coords: Vector<2, isize>) -> Option<&mut Room> {
        if !self.in_bounds(coords) {
            return None;
        }

        let index = Map::index_from_coords(coords);

        Some(&mut self.data[index])
    }

    pub fn set_room_feature(&mut self, coords: Vector<2, isize>, feature: Option<RoomFeature>) {
        let room = self.get_room_mut(coords).unwrap();

        room.feature = feature;
    }

    pub fn near_rooms_coords(&self, coords: Vector<2, isize>) -> Vec<Vector<2, isize>> {
        [[-1, 0], [1, 0], [0, -1], [0, 1]]
            .iter()
            .map(|v| coords + Vector(*v))
            .collect::<Vec<_>>()
    }

    pub fn near_rooms_features(&self, coords: Vector<2, isize>) -> HashSet<RoomFeature> {
        let mut features = HashSet::new();

        for room_coords in self.near_rooms_coords(coords) {
            if self.in_bounds(room_coords) {
                let room = self.get_room(room_coords).unwrap();

                room.feature.map(|f| features.insert(f));
            }
        }

        features
    }

    pub fn get_player_start_coords(&self) -> Vector<2, isize> {
        for (index, room) in self.data.iter().enumerate() {
            match room.feature {
                Some(feature) => match feature {
                    RoomFeature::PlayerStart => return self.coords_from_index(index),
                    _ => continue,
                },
                None => continue,
            }
        }

        unreachable!()
    }

    pub fn get_dracula_coords(&self) -> Vector<2, isize> {
        for (index, room) in self.data.iter().enumerate() {
            match room.feature {
                Some(feature) => match feature {
                    RoomFeature::Dracula => return self.coords_from_index(index),
                    _ => continue,
                },
                None => continue,
            }
        }

        unreachable!()
    }

    fn coords_from_index(&self, index: usize) -> Vector<2, isize> {
        Vector([(index % MAP_WIDTH) as isize, (index / MAP_WIDTH) as isize])
    }

    fn index_from_coords(coords: Vector<2, isize>) -> usize {
        coords.y() as usize * MAP_WIDTH + coords.x() as usize
    }
}

//~ impl Iter

struct Player {
    coords: Vector<2, isize>,
    has_arrow: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            coords: Vector([2, 2]),
            has_arrow: false,
        }
    }

    pub fn start_map(&mut self, map: &Map) {
        self.coords = map.get_player_start_coords();
        self.has_arrow = true;
    }
}

#[derive(Default, Clone, Copy)]
struct Room {
    feature: Option<RoomFeature>,
    //~ dracula_smell: bool,
    //~ bat_sound: bool,
    //~ pit_breeze: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum RoomFeature {
    PlayerStart,
    Dracula,
    Arrow,
    Bat,
    Pit,
}

impl RoomFeature {
    fn get_feel_text(&self) -> Option<&'static str> {
        use RoomFeature::*;

        match self {
            PlayerStart => None,
            Dracula => Some("You smell blood."),
            Arrow => None,
            //~ Arrow => Some("The smell metal."),
            Bat => Some("You hear flapping."),
            Pit => Some("You feel a breeze."),
        }
    }

    fn get_arrival_text(&self) -> Option<&'static str> {
        use RoomFeature::*;

        match self {
            PlayerStart => Some("This seems familiar"),
            Dracula => Some("Dracula found you."),
            Arrow => Some("You find the Arrow."),
            Bat => Some("The bats take you."),
            Pit => Some("You fall into a pit."),
        }
    }
}

pub fn print_message(fc: &mut Vfc, coords: Vector<2, isize>, message: &str) {
    //~ println!("{}", message);

    fc::draw_text(0, fc, coords.x() as usize, coords.y() as usize, message);

    // TODO: also/only print message to graphics background layer
}

////====////====\\\\====\\\\ Main ////====////====\\\\====\\\\

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
        &("Hunt the Dracula".to_owned() + " " + "(debug - hold Escape to exit)"),
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
    keybinds.insert(Key::Up, Action::Up);
    keybinds.insert(Key::Down, Action::Down);
    keybinds.insert(Key::Space, Action::Fire);

    keybinds.insert(Key::A, Action::Left);
    keybinds.insert(Key::D, Action::Right);
    keybinds.insert(Key::W, Action::Up);
    keybinds.insert(Key::S, Action::Down);
    keybinds.insert(Key::Enter, Action::Fire);

    let mut key_pressed = HashMap::new();
    let mut key_down = HashMap::new();

    //----\\ INITIAL GAME STATE //----\\

    let debug_output = false;

    let mut game = {
        let mut rng = random::new_prng();

        let map = Map::new(&mut rng);

        let mut player = Player::new();

        player.start_map(&map);

        let displayed_features = map.near_rooms_features(player.coords);

        Game {
            state: ShowRoom,
            map,
            player,
            rng,
            displayed_features,
        }
    };

    //----\\ MAIN LOOP //----\\

    let mut emergency_exit = 100;

    let debug_allowed = false;
    let mut debug_mode = false;

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
                fc.oam_hidden = !fc.oam_hidden;
            }

            if window.is_key_pressed(Key::F9, KeyRepeat::No) {
                fc.bg_layers[0].hidden = !fc.bg_layers[0].hidden;
            }

            if window.is_key_pressed(Key::F10, KeyRepeat::No) {
                fc.bg_layers[1].hidden = !fc.bg_layers[1].hidden;
            }
        }

        // Game Input //

        key_pressed.insert(Action::Left, false);
        key_pressed.insert(Action::Right, false);
        key_pressed.insert(Action::Up, false);
        key_pressed.insert(Action::Down, false);
        key_pressed.insert(Action::Fire, false);

        key_down.insert(Action::Left, false);
        key_down.insert(Action::Right, false);
        key_down.insert(Action::Up, false);
        key_down.insert(Action::Down, false);
        key_down.insert(Action::Fire, false);

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

        let fire_pressed = *key_pressed.get(&Action::Fire).unwrap();

        //// State-specific Logic ////

        match &game.state {
            MainPlay => {
                let moved = if horizontal_move_vector != 0 {
                    let new_coords = game.player.coords + Vector([horizontal_move_vector, 0]);

                    Some(if game.map.in_bounds(new_coords) {
                        game.player.coords = new_coords;
                        true
                    } else {
                        false
                    })
                } else if vertical_move_vector != 0 {
                    let new_coords = game.player.coords + Vector([0, vertical_move_vector]);

                    Some(if game.map.in_bounds(new_coords) {
                        game.player.coords = new_coords;
                        true
                    } else {
                        false
                    })
                } else {
                    None
                };

                let room = game.map.get_room(game.player.coords).unwrap();

                let allow_move = if let Some(feature) = room.feature {
                    use RoomFeature::*;

                    match feature {
                        Dracula | Bat | Pit => {
                            for i in 3..=9 {
                                fc::clear_line(0, &mut fc, i);
                            }

                            print_message(
                                &mut fc,
                                Vector([1, 9]),
                                feature.get_arrival_text().unwrap(),
                            );

                            game.state = Die;

                            false
                        }
                        _ => true,
                    }
                } else {
                    true
                };

                if allow_move {
                    match moved {
                        Some(true) => {
                            fc::clear_bg_tiles(0, &mut fc);
                            //~ game.change_state(GameOver);
                            game.state = ShowRoom;
                        }
                        _ => (),
                    }
                }

                // TODO: show walls map

                // TODO: actual game over check

                if fire_pressed {
                    fc::clear_bg_tiles(0, &mut fc);
                    game.state = FireSelect;
                }

                let game_over = false;

                if game_over {
                    //~ game.change_state(GameOver);
                    game.state = GameOver;
                }
            }
            ShowRoom => {
                use RoomFeature::*;

                game.update_features();

                print_message(&mut fc, Vector([1, 1]), "You are in the castle.");

                let mut y = 3;
                for f in [Dracula, Pit, Bat].iter() {
                    if game.displayed_features().contains(f) {
                        let msg = f.get_feel_text().unwrap();

                        print_message(&mut fc, Vector([1, y]), msg);
                    }

                    y += 2
                }

                game.state = MainPlay;
            }
            FireSelect => {
                print_message(&mut fc, Vector([1, 1]), "Hit direction to fire.");

                if horizontal_move_vector != 0 || vertical_move_vector != 0 {
                    let new_coords = if horizontal_move_vector != 0 {
                        game.player.coords + Vector([horizontal_move_vector, 0])
                    } else {
                        game.player.coords + Vector([0, vertical_move_vector])
                    };

                    print_message(&mut fc, Vector([1, 3]), "The Arrow flies,");

                    let (success, alerted, msg) = if let Some(room) = game.map.get_room(new_coords)
                    {
                        let empty_room = (false, true, "hits the floor loudly.");

                        if let Some(feature) = room.feature {
                            use RoomFeature::*;

                            match feature {
                                Dracula => (true, true, "Dracula is defeated."),
                                Bat => (true, false, "you slay the Bat."),
                                Pit => (false, false, "you hear nothing."),
                                _ => empty_room,
                            }
                        } else {
                            empty_room
                        }
                    } else {
                        (false, true, "hits the wall loudly.")
                    };

                    print_message(&mut fc, Vector([1, 4]), msg);

                    let game_over = if success {
                        let room = game.map.get_room(new_coords).unwrap();

                        if let Some(feature) = room.feature {
                            use RoomFeature::*;

                            match feature {
                                Dracula => true,
                                Bat => {
                                    game.map.set_room_feature(new_coords, None);
                                    false
                                }
                                _ => unreachable!(),
                            }
                        } else {
                            unreachable!()
                        }
                    } else {
                        false
                    };

                    game.state = if game_over {
                        GameOver
                    } else {
                        if alerted {
                            DraculaAlerted
                        } else {
                            FireContinue
                        }
                    };
                } else if fire_pressed {
                    game.state = ShowRoom;
                }
            }
            DraculaAlerted => {
                print_message(&mut fc, Vector([1, 7]), "Dracula could hear.");

                game.move_dracula();

                game.state = FireContinue;
            }
            FireContinue => {
                print_message(&mut fc, Vector([1, 9]), "Press fire to continue.");

                if fire_pressed {
                    fc::clear_bg_tiles(0, &mut fc);
                    game.state = ShowRoom;
                }
            }
            Die => {
                print_message(&mut fc, Vector([12, 18]), "You died.");

                game.state = GameOver;
            }
            GameOver => {
                print_message(&mut fc, Vector([1, 18]), "Game Over.");

                if fire_pressed {
                    fc::clear_bg_tiles(0, &mut fc);
                    game.state = Restart;
                }
            }
            Restart => {
                game = {
                    let mut rng = random::new_prng();

                    let map = Map::new(&mut rng);

                    let mut player = Player::new();

                    player.start_map(&map);

                    let displayed_features = map.near_rooms_features(player.coords);

                    Game {
                        state: ShowRoom,
                        map,
                        player,
                        rng,
                        displayed_features,
                    }
                };
            }
        }

        //----\\ RENDERING //----\\

        clear_sprites(&mut fc);

        let dot_sprite = &mut fc.oam.0[0];

        dot_sprite.tile_index = TileIndex(TILE_MAP_DOT as u8);

        dot_sprite.x = ((MAP_X + game.player.coords.0[0] as usize) % 32) as u8 * 8;
        dot_sprite.y = ((MAP_Y + game.player.coords.0[1] as usize) % 32) as u8 * 8;

        for yi in 0..MAP_HEIGHT {
            for xi in 0..MAP_WIDTH {
                fc::poke_bg(
                    0,
                    &mut fc,
                    (xi + MAP_X) % 32,
                    (yi + MAP_Y) % 32,
                    TileIndex((TILE_MAP_GRID + yi * 16 + xi) as u8),
                );
            }
        }

        if game.state == GameOver || debug_mode {
            use RoomFeature::*;

            let feature_id = {
                let features = [PlayerStart, Dracula, Arrow, Bat, Pit];

                let mut feature_id = HashMap::new();

                for (i, f) in features.iter().enumerate() {
                    feature_id.insert(*f, i + TILE_FEATURE_ICONS);
                }

                feature_id
            };

            for yi in 0..MAP_HEIGHT {
                for xi in 0..MAP_WIDTH {
                    let coords = Vector([xi as isize, yi as isize]);

                    let room = game.map.get_room(coords).unwrap();

                    if let Some(feature) = room.feature {
                        let i = feature_id.get(&feature).unwrap();

                        fc::poke_bg(
                            0,
                            &mut fc,
                            (xi + MAP_X) % 32,
                            (yi + MAP_Y) % 32,
                            TileIndex(*i as u8),
                        );
                    }
                }
            }
        }

        //----\\ MORE RENDERING //----\\

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
                "{instant_microsecs_per_frame:?}\t{average_frametime:?}\t{average_fps}\t{frames}"
            );
        }

        frames += 1;
    }
}
