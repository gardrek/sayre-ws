use super::EngineState;

use hli::fc;
use hli::random;
use hli::vector;
use hli::vfc;

use vector::Vector;

use super::plat::Player;

mod map;

use map::Map;

pub const COLLIDE_SOLID: u8 = 0b0100_0000;

pub const PLAYER_SPRITE: u8 = 0x00;
pub const PLAYER_WEAPON_SPRITE_START: u8 = 0x01;
//~ pub const PLAYER_WEAPON_SPRITE_END: u8 = 0x07;
pub const ENEMY_SPRITE_START: u8 = 0x07;
//~ pub const ENEMY_SPRITE_END: u8 = 0x0f;

//~ pub const VECTOR_8x8: Vector<f64, 2> = Vector([8.0, 8.0]);

//~ use random::shuffle;

//

pub struct Game {
    enemies: Vec<Enemy>,
    rng: random::Prng,
    player: Player,
    chain: Option<Chain>,
    map: Map,
}

impl Game {
    pub fn new() -> Game {
        let rng = random::new_prng();

        //~ let x = rng.next().unwrap() as i16 as f64 / 1024.0;
        //~ let y = rng.next().unwrap() as i16 as f64 / 1024.0;

        //~ let player_position = Vector([x, y]);

        let map = Map::test_level();

        let v_fix = |v: &Vector<i32, 2>| Vector([v.x() as f64 + 0.5, v.y() as f64 + 0.5]) * 8.0;

        let player_position = v_fix(&map.player_position());

        //~ let player_position = Vector([
        //~ map.player_position().x() as f64,
        //~ map.player_position().y() as f64,
        //~ ]) * 8.0;

        let mut enemies = vec![];

        for (ch, v) in map.enemies().iter() {
            let enemy = Enemy::new(*ch, v_fix(v));

            enemies.push(enemy);
        }

        let player = Player::new(player_position.x(), player_position.y());

        Game {
            enemies,
            rng,
            player,
            chain: None,
            map,
        }
    }

    pub fn init(&mut self, engine_state: &mut EngineState) {
        self.map.draw_map(engine_state, 0, 0, 0, 0, 32, 32);
    }

    pub fn tick(&mut self, engine_state: &mut EngineState) {
        /*
        {
            let mut scroll_x = &mut engine_state.fc.bg_layers[0].x;
            *scroll_x = scroll_x.wrapping_add(1);

            let mut scroll_y = &mut engine_state.fc.bg_layers[0].y;
            *scroll_y = scroll_y.wrapping_add(1);
        }
        //~ */

        //~ let x = self.rng.next().unwrap() as i16 as f64 / 1024.0;
        //~ let y = self.rng.next().unwrap() as i16 as f64 / 1024.0;

        let up = engine_state.input.held[&crate::Action::Up];
        let down = engine_state.input.held[&crate::Action::Down];
        let left = engine_state.input.held[&crate::Action::Left];
        let right = engine_state.input.held[&crate::Action::Right];

        let y = if up { -1.0 } else { 0.0 } + if down { 1.0 } else { 0.0 };
        let x = if left { -1.0 } else { 0.0 }
            + if right { 1.0 } else { 0.0 }
            + if y == 0.0 {
                if self.player.flipx {
                    -1.0
                } else {
                    1.0
                }
            } else {
                0.0
            };

        let attack_direction = Vector([x as f64, y as f64]);

        self.chain = if let Some(mut chain) = self.chain.take() {
            chain.tick(&self.player);

            if chain.is_dead() {
                None
            } else {
                Some(chain)
            }
        } else {
            if engine_state.input.pressed[&crate::Action::Fire] {
                Some(Chain::new_flail(&self.player, 24, 4.0, attack_direction))
            } else {
                None
            }
        };

        /*if engine_state.input.pressed[&crate::Action::Fire] {
            self.reset(engine_state);
        }*/

        self.update_player(&engine_state);

        // enemy ai

        for enemy in self.enemies.iter_mut() {
            let turn = self.rng.next().unwrap() % 4 == 0;

            if turn {
                let d = Vector([
                    (self.rng.next().unwrap() as isize % 3 - 1) as f64,
                    (self.rng.next().unwrap() as isize % 5 - 3) as f64,
                ]);

                enemy.delta = if d.mag() == 0.0 { d } else { d.norm() } * 0.5;
            }

            //~ enemy.delta = d.norm() * 0.5;
            //~ enemy.delta = d;

            enemy.tick(&engine_state);
        }
    }

    pub fn render(&mut self, engine_state: &mut EngineState) {
        let p = &mut engine_state.fc.oam[vfc::OamIndex(PLAYER_SPRITE)];

        //~ p.x = ((self.player_position.x() as usize) as u8).wrapping_mul(1);
        //~ p.y = ((self.player_position.y() as usize) as u8).wrapping_mul(1);

        let scroll_x = engine_state.fc.bg_layers[0].x;
        let scroll_y = engine_state.fc.bg_layers[0].y;

        let coord_fix = |x: f64, y: f64| {
            (
                ((x.floor() as isize % 256) as u8)
                    .wrapping_sub(4)
                    .wrapping_add(scroll_x),
                ((y.floor() as isize % 256) as u8)
                    .wrapping_sub(4)
                    .wrapping_add(scroll_y),
            )
        };

        p.tile_index = vfc::TileIndex(b'@');

        (p.x, p.y) = coord_fix(self.player.x, self.player.y);

        let r = if self.player.flipx { 0b000 } else { 0b001 };

        p.attributes.set_rotation(r);

        let chain_head = &mut engine_state.fc.oam[vfc::OamIndex(PLAYER_WEAPON_SPRITE_START)];

        if let Some(c) = &mut self.chain {
            (chain_head.x, chain_head.y) = coord_fix(c.head.x(), c.head.y());
        } else {
            chain_head.hide();
        }

        chain_head.tile_index = vfc::TileIndex(b'*');

        let max = 4;

        //~ let parity = engine_state.frames % 30 / 10;
        let parity = engine_state.frames % 3;

        for i in 1..max {
            //~ let chain_tail = &mut engine_state.fc.oam[vfc::OamIndex(1 + i as u8)];
            let chain_tail =
                &mut engine_state.fc.oam[vfc::OamIndex(PLAYER_WEAPON_SPRITE_START + 1)];

            if i % 3 != parity {
                //~ chain_tail.hide();
                continue;
            }

            if let Some(c) = &mut self.chain {
                let v = Vector([self.player.x as f64, self.player.y as f64])
                    + c.offset_vec * (i as f64 / max as f64);

                (chain_tail.x, chain_tail.y) = coord_fix(v.x(), v.y());
            } else {
                chain_tail.hide();
            }

            chain_tail.tile_index = vfc::TileIndex(b'+');
        }

        //

        for (i, enemy) in self.enemies.iter().enumerate() {
            // TODO: make it so too many enemies on screeen get cycle-flickered
            let e =
                &mut engine_state.fc.oam[vfc::OamIndex(ENEMY_SPRITE_START.wrapping_add(i as u8))];

            (e.x, e.y) = coord_fix(enemy.position.x(), enemy.position.y());

            e.tile_index = vfc::TileIndex(enemy.ch);
        }
    }

    /*
    pub fn reset(&mut self, engine_state: &mut EngineState) {
        fc::clear_bg_tiles(0, &mut engine_state.fc);
        self.init(engine_state);
    }
    */

    fn update_player(&mut self, engine_state: &EngineState) {
        let mut player = Player::new(0.0, 0.0);

        std::mem::swap(&mut player, &mut self.player);

        player.update(&self, engine_state);

        std::mem::swap(&mut player, &mut self.player);
    }
}

/*
fn poke_platform(fc: &mut Vfc, x: usize, y: usize, w: usize, tile_index: TileIndex, rotation: u8) {
    for xi in (x.wrapping_sub(w / 2))..(x + w / 2) {
        fc::poke_main_bg(fc, xi, y, tile_index);
        fc::poke_main_rotation(fc, xi, y, rotation);
    }
}
*/

struct Chain {
    head: Vector<f64, 2>,
    offset_vec: Vector<f64, 2>,
    throw_direction: Vector<f64, 2>,
    throw_speed: f64,
    life: i32,
    max_life: i32,
    hooked: bool,
}

impl Chain {
    pub fn new_flail(
        player: &Player,
        life: i32,
        throw_speed: f64,
        throw_direction: Vector<f64, 2>,
    ) -> Chain {
        //~ let x_dir = if player.flipx { -1.0 } else { 1.0 };
        //~ let throw_direction = Vector([x_dir, 0.0]);

        let throw_direction = throw_direction.norm();

        Chain {
            head: Vector([player.x, player.y]),
            offset_vec: Vector::zero(),
            throw_direction,
            throw_speed,
            life,
            max_life: life,
            hooked: false,
        }
    }

    pub fn tick(&mut self, tracked_player: &Player) {
        let player_vec = Vector([tracked_player.x, tracked_player.y]);

        self.offset_vec = if self.life > self.max_life / 2 {
            self.throw_direction * (self.max_life - self.life) as f64 * self.throw_speed
        } else {
            self.throw_direction * (self.life) as f64 * self.throw_speed
        };

        self.head = player_vec + self.offset_vec;

        self.life -= 1;
    }

    pub fn is_dead(&self) -> bool {
        !self.hooked && self.life <= 0
    }
}

struct Enemy {
    ch: u8,
    //~ central_position: Vector<f64, 2>,
    //~ offset_position: Vector<f64, 2>,
    position: Vector<f64, 2>,
    pub delta: Vector<f64, 2>,
}

impl Enemy {
    pub fn new(ch: u8, position: Vector<f64, 2>) -> Enemy {
        Enemy {
            ch,
            position,
            delta: Vector::zero(),
            //~ central_position: position,
            //~ offset_position: Vector::zero(),
        }
    }

    pub fn tick(&mut self, engine_state: &EngineState) {
        let next_position = self.position + self.delta;

        let collision = fc::peek_main_bg(
            &engine_state.fc,
            next_position.x().floor() as isize as usize,
            next_position.y().floor() as isize as usize,
        )
        .0 != 0;

        if collision {
            self.delta = -self.delta;
            self.position = self.position + self.delta;
        } else {
            self.position = next_position;
        }
    }
}

/*
fn flerp(a: f64, b: f64, f: f64) -> f64 {
    a * (1.0 - f) + b * f
}

struct BBox {
    center: Vector<f64, 2>,
    dimensions: Vector<f64, 2>,
}


impl BBox {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> BBox {
        BBox::from_vectors(Vector([x, y]), Vector([width, height]))
    }

    pub fn from_vectors(center: Vector<f64, 2>, dimensions: Vector<f64, 2>) -> BBox {
        BBox { center, dimensions }
    }

    pub fn x(&self) -> f64 {
        self.center.x()
    }

    pub fn y(&self) -> f64 {
        self.center.y()
    }

    pub fn w(&self) -> f64 {
        self.dimensions.x()
    }

    pub fn h(&self) -> f64 {
        self.dimensions.y()
    }

    pub fn overlap(&self, other: &BBox) -> bool {
        let xd = self.x() - other.x();
        let xs = self.w() * 0.5 + other.w() * 0.5;
        if xd.abs() >= xs {
            return false;
        }

        let yd = self.y() - other.y();
        let ys = self.h() * 0.5 + self.h() * 0.5;
        if yd.abs() >= ys {
            return false;
        }

        return true;
    }
}
*/
