// advanced micro platformer
// @matthughson

// if you make a game with this
// starter kit, please consider
// linking back to the bbs post
// for this cart, so that others
// can learn from it too!
// enjoy!
// @matthughson

// https://www.lexaloffle.com/bbs/?tid=28793

#![allow(dead_code)]

use super::game;
use super::Action;
use super::EngineState;
use super::Input;

use game::Game;
use game::COLLIDE_SOLID;

use hli::fc::peek_main_bg;

fn map(_unk0: f64, _unk1: f64, _unk2: f64, _unk3: f64, _unk4: f64, _unk5: f64) {
    todo!()
}

fn cls(_color: f64) -> bool {
    todo!()
}

fn sfx(_snd: f64) {
    ()
}

fn num_to_action(num: usize) -> Action {
    use Action::*;

    match num {
        0 => Left,
        1 => Right,
        2 => Up,
        3 => Down,
        4 => Fire,
        5 => Jump,
        _ => todo!("unmapped action number"),
    }
}

fn btn(b: usize, input: &Input) -> bool {
    input.held[&num_to_action(b)]
}

fn btnp(b: usize, input: &Input) -> bool {
    input.pressed[&num_to_action(b)]
}

fn spr(_frame: f64, _x: f64, _y: f64, _unk_w: f64, _unk_h: f64, _flipx: bool, _unk: bool) {
    todo!()
}

// get map tile at screen position x, y maybe?
// certainly pixel position
fn mget(engine_state: &EngineState, x: f64, y: f64) -> u8 {
    peek_main_bg(
        &engine_state.fc,
        x.floor() as isize as usize,
        y.floor() as isize as usize,
    )
    .0
}

fn fget(tile: u8, _flag: f64) -> bool {
    let _old = (tile & COLLIDE_SOLID) != 0;
    tile / 16 != 0
}

fn camera(_x: f64, _y: f64) -> f64 {
    todo!()
}

fn flr(n: f64) -> f64 {
    n.floor()
}

fn rnd(n: f64) -> f64 {
    (n + 0.5).floor()
}

fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

fn mid(a: f64, b: f64, c: f64) -> f64 {
    if (a < b && b < c) || (c < b && b < a) {
        return b;
    } else if (b < a && a < c) || (c < a && a < b) {
        return a;
    } else {
        return c;
    }
}

fn clamp(low: f64, value: f64, high: f64) -> f64 {
    low.max(value.min(high))
}

//----

// log
//~ printh("\n\n-------\n-start-\n-------");

// config
// ------------------------------

// sfx
pub struct Snd {
    jump: f64,
}

// math
// ------------------------------

impl Player {
    // check if pushing into side tile and resolve.
    // requires self.dx,self.x,self.y, and
    // assumes tile flag 0 == solid
    // assumes sprite size of 8x8
    fn collide_side(&mut self, engine_state: &EngineState) -> bool {
        let offset = self.w / 3.0;
        for i in ((-(self.w as i32) / 3)..=(self.w as i32 / 3)).step_by(2) {
            let i = i as f64;

            // if self.dx>0.0 {
            if fget(
                mget(engine_state, (self.x + (offset)) / 8.0, (self.y + i) / 8.0),
                0.0,
            ) {
                self.dx = 0.0;
                self.x = (flr((self.x + (offset)) / 8.0) * 8.0) - (offset);
                return true;
            }
            // } else if self.dx<0.0 {
            if fget(
                mget(engine_state, (self.x - (offset)) / 8.0, (self.y + i) / 8.0),
                0.0,
            ) {
                self.dx = 0.0;
                self.x = (flr((self.x - (offset)) / 8.0) * 8.0) + 8.0 + (offset);
                return true;
            }
            //     }
        }
        // didn't hit a solid tile.
        return false;
    }

    // check if pushing into floor tile and resolve.
    // requires self.dx,self.x,self.y,self.grounded,self.airtime and
    // assumes tile flag 0 or 1 == solid
    fn collide_floor(&mut self, engine_state: &EngineState) -> bool {
        // only check for ground when falling.
        if self.dy < 0.0 {
            return false;
        }
        let mut landed = false;
        // check for collision at multiple points along the bottom
        // of the sprite: left, center, and right.
        for i in ((-(self.w as i32) / 3)..=(self.w as i32 / 3)).step_by(2) {
            let i = i as f64;

            let tile = mget(
                engine_state,
                (self.x + i) / 8.0,
                (self.y + (self.h / 2.0)) / 8.0,
            );
            if fget(tile, 0.0) || (fget(tile, 1.0) && self.dy >= 0.0) {
                self.dy = 0.0;
                self.y = (flr((self.y + (self.h / 2.0)) / 8.0) * 8.0) - (self.h / 2.0);
                self.grounded = true;
                self.airtime = 0;
                landed = true;
            }
        }
        return landed;
    }

    // check if pushing into roof tile and resolve.
    // requires self.dy,self.x,self.y, and
    // assumes tile flag 0 == solid
    fn collide_roof(&mut self, engine_state: &EngineState) {
        // check for collision at multiple points along the top
        // of the sprite: left, center, and right.
        for i in ((-(self.w as i32) / 3)..=(self.w as i32 / 3)).step_by(2) {
            let i = i as f64;

            if fget(
                mget(
                    engine_state,
                    (self.x + i) / 8.0,
                    (self.y - (self.h / 2.0)) / 8.0,
                ),
                0.0,
            ) {
                self.dy = 0.0;
                self.y = flr((self.y - (self.h / 2.0)) / 8.0) * 8.0 + 8.0 + (self.h / 2.0);
                self.jump_hold_time = 0;
            }
        }
    }
}

pub struct Vector2 {
    x: f64,
    y: f64,
}

// make 2d vector
fn m_vec(x: f64, y: f64) -> Vector2 {
    Vector2 { x, y }
}

// utils
// ------------------------------

// print string with outline.
/*
fn printo(s: &'static str,startx,
                                                             starty,col,
                                                             col_bg) {
    print(str,startx+1.0,starty,col_bg);
    print(str,startx-1.0,starty,col_bg);
    print(str,startx,starty+1.0,col_bg);
    print(str,startx,starty-1.0,col_bg);
    print(str,startx+1.0,starty-1.0,col_bg);
    print(str,startx-1.0,starty-1.0,col_bg);
    print(str,startx-1.0,starty+1.0,col_bg);
    print(str,startx+1.0,starty+1.0,col_bg);
    print(str,startx,starty,col);
}


// print string centered with
// outline.
fn printc(
    str,x,y,
    col,col_bg,
    special_chars) {

    let len=(#str*4.0)+(special_chars*3.0);
    let startx=x-(len/2.0);
    let starty=y-2.0;
    printo(str,startx,starty,col,col_bg);
}
*/

// objects
// ------------------------------

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub w: f64,
    pub h: f64,
    max_dx: f64, // max x speed
    max_dy: f64, // max y speed

    jump_speed: f64, // jump veloclity
    acc: f64,        // acceleration
    dcc: f64,        // decceleration
    air_dcc: f64,    // air decceleration
    grav: f64,

    jump_button: JumpButton,

    jump_hold_time: usize, // how long jump is held
    //~ min_jump_press: usize, // min time jump can be held
    max_jump_press: usize, // max time jump can be held

    //~ jump_btn_released: bool, // can we jump again?
    pub grounded: bool, // on ground

    airtime: usize, // time since grounded
    anims: HashMap<&'static str, Anim>,

    curanim: &'static str, // currently playing animation
    curframe: usize,       // curent frame of animation.
    animtick: usize,       // ticks until next frame should show.
    pub flipx: bool,       // show sprite be flipped.

    snd: Snd,
}

impl Player {
    pub fn new(x: f64, y: f64) -> Player {
        m_player(x, y)
    }

    // request new animation to play.
    fn set_anim(&mut self, anim: &'static str) {
        if anim == self.curanim {
            return;
        }; // early out.
        let a = &self.anims[anim];
        self.animtick = a.ticks; // ticks count down.
        self.curanim = anim;
        self.curframe = 1;
    }

    // call once per tick.
    pub fn update(&mut self, _game_state: &Game, engine_state: &EngineState) {
        let input = &engine_state.input;

        // todo: kill enemies.

        // track button presses
        let raw_bl = btn(0, input); // left
        let raw_br = btn(1, input); // right

        // handle double press (pressing left and right at once)
        let bl = raw_bl && !raw_br;
        let br = raw_br && !raw_bl;

        // move left/right
        if bl {
            self.dx -= self.acc;
        }

        if br {
            self.dx += self.acc;
        }

        if !br && !bl {
            if self.grounded {
                self.dx *= self.dcc;
            } else {
                self.dx *= self.air_dcc;
            }
        }

        // limit walk speed
        self.dx = clamp(-self.max_dx, self.dx, self.max_dx);

        //~ self.dx = self.dx.max(-self.max_dx);

        //~ self.dx = self.dx.min(self.max_dx);

        // move in x
        self.x += self.dx;

        // hit walls
        self.collide_side(engine_state);

        // jump buttons
        self.jump_button.update(input);

        // jump is complex.
        // we allow jump if:
        //     on ground
        //     recently on ground
        //     pressed btn right before landing
        // also, jump velocity is
        // not instant. it applies over
        // multiple frames.
        if self.jump_button.is_down {
            // is player on ground recently.
            // allow for jump right after
            // walking off ledge.
            let on_ground = self.grounded || self.airtime < 5;
            // was btn presses recently?
            // allow for pressing right before
            // hitting ground.
            let new_jump_btn = self.jump_button.ticks_down < 10.0;
            // is player continuing a jump
            // or starting a new one?
            if self.jump_hold_time > 0 || (on_ground && new_jump_btn) {
                if self.jump_hold_time == 0 {
                    sfx(self.snd.jump);
                    sfx(0.0);
                } // new jump snd
                self.jump_hold_time += 1;
                // keep applying jump velocity
                // until max jump time.
                if self.jump_hold_time < self.max_jump_press {
                    self.dy = self.jump_speed; // keep going up while held
                }
            }
        } else {
            self.jump_hold_time = 0;
        }

        // move in y
        self.dy += self.grav;
        self.dy = mid(-self.max_dy, self.dy, self.max_dy);
        self.y += self.dy;

        // floor
        if !self.collide_floor(engine_state) {
            self.set_anim("jump");
            self.grounded = false;
            self.airtime += 1;
        }

        // roof
        self.collide_roof(engine_state);

        // handle playing correct animation when
        // on the ground.
        if self.grounded {
            if br {
                if self.dx < 0.0 {
                    // pressing right but still moving left.
                    self.set_anim("slide");
                } else {
                    self.set_anim("walk");
                }
            } else if bl {
                if self.dx > 0.0 {
                    // pressing left but still moving right.
                    self.set_anim("slide");
                } else {
                    self.set_anim("walk");
                }
            } else {
                self.set_anim("stand");
            }
        }

        // flip
        if br {
            self.flipx = false;
        } else if bl {
            self.flipx = true;
        }

        // anim tick
        self.animtick -= 1;
        if self.animtick <= 0 {
            self.curframe += 1;
            let a = &self.anims[self.curanim];
            self.animtick = a.ticks; // reset timer
            if self.curframe > a.frames.len() {
                self.curframe = 1; // loop
            }
        }
    }

    // draw the player
    fn draw(&self) {
        let a = &self.anims[self.curanim];
        let frame = a.frames[self.curframe];
        spr(
            frame,
            self.x - (self.w / 2.0),
            self.y - (self.h / 2.0),
            self.w / 8.0,
            self.h / 8.0,
            self.flipx,
            false,
        );
    }
}

// helper for more complex
// button press tracking.
// todo: generalize button index.
#[derive(Default)]
pub struct JumpButton {
    // state
    is_pressed: bool, // pressed this frame
    is_down: bool,    // currently down
    ticks_down: f64,  // how long down
}

impl JumpButton {
    fn update(&mut self, input: &Input) {
        // start with assumption
        // that not a new press.
        self.is_pressed = false;
        if btn(5, input) {
            if !self.is_down {
                self.is_pressed = true;
            }
            self.is_down = true;
            self.ticks_down += 1.0;
        } else {
            self.is_down = false;
            self.is_pressed = false;
            self.ticks_down = 0.0;
        }
    }
}

use std::collections::HashMap;

pub struct Anim {
    ticks: usize,
    frames: Vec<f64>,
}

// make the player
fn m_player(x: f64, y: f64) -> Player {
    let snd = Snd { jump: 0.0 };

    // todo: refactor with m_vec.
    let p = Player {
        x: x,
        y: y,

        dx: 0.0,
        dy: 0.0,

        w: 8.0,
        h: 8.0,

        max_dx: 1.0, // max x speed
        max_dy: 2.0, // max y speed

        jump_speed: -1.75, // jump veloclity
        acc: 0.05,         // acceleration
        dcc: 0.8,          // decceleration
        air_dcc: 1.0,      // air decceleration
        grav: 0.15,

        jump_button: JumpButton::default(),

        jump_hold_time: 0, // how long jump is held
        //~ min_jump_press: 5,  // min time jump can be held
        max_jump_press: 15, // max time jump can be held

        //~ jump_btn_released: true, // can we jump again?
        grounded: false, // on ground

        airtime: 0, // time since grounded

        // animation definitions.
        // use with set_anim()
        anims: HashMap::from([
            (
                "stand",
                Anim {
                    ticks: 1,          // how long is each frame shown.
                    frames: vec![2.0], // what frames are shown.
                },
            ),
            (
                "walk",
                Anim {
                    ticks: 5,
                    frames: vec![3.0, 4.0, 5.0, 6.0],
                },
            ),
            (
                "jump",
                Anim {
                    ticks: 1,
                    frames: vec![1.0],
                },
            ),
            (
                "slide",
                Anim {
                    ticks: 1,
                    frames: vec![7.0],
                },
            ),
        ]),

        curanim: "walk", // currently playing animation
        curframe: 1,     // curent frame of animation.
        animtick: 0,     // ticks until next frame should show.
        flipx: false,    // show sprite be flipped.

        snd,
    };

    return p;
}

pub struct Camera {
    pos: Vector2,
    pull_threshold: f64,
    pos_min: Vector2,
    pos_max: Vector2,
    shake_remaining: f64,
    shake_force: f64,
}

impl Camera {
    fn update(&mut self, target: &Player) {
        self.shake_remaining = max(0.0, self.shake_remaining - 1.0);

        // follow target outside of
        // pull range.
        if self.pull_max_x() < target.x {
            self.pos.x += min(target.x - self.pull_max_x(), 4.0);
        }
        if self.pull_min_x() > target.x {
            self.pos.x += min(target.x - self.pull_min_x(), 4.0);
        }
        if self.pull_max_y() < target.y {
            self.pos.y += min(target.y - self.pull_max_y(), 4.0);
        }
        if self.pull_min_y() > target.y {
            self.pos.y += min(target.y - self.pull_min_y(), 4.0);
        }

        // lock to edge
        if self.pos.x < self.pos_min.x {
            self.pos.x = self.pos_min.x
        };
        if self.pos.x > self.pos_max.x {
            self.pos.x = self.pos_max.x
        };
        if self.pos.y < self.pos_min.y {
            self.pos.y = self.pos_min.y
        };
        if self.pos.y > self.pos_max.y {
            self.pos.y = self.pos_max.y
        };
    }

    fn cam_pos(&self) -> (f64, f64) {
        // calculate camera shake.
        let mut shk = m_vec(0.0, 0.0);
        if self.shake_remaining > 0.0 {
            shk.x = rnd(self.shake_force) - (self.shake_force / 2.0);
            shk.y = rnd(self.shake_force) - (self.shake_force / 2.0);
        }
        return (self.pos.x - 64.0 + shk.x, self.pos.y - 64.0 + shk.y);
    }

    fn pull_max_x(&self) -> f64 {
        return self.pos.x + self.pull_threshold;
    }

    fn pull_min_x(&self) -> f64 {
        return self.pos.x - self.pull_threshold;
    }

    fn pull_max_y(&self) -> f64 {
        return self.pos.y + self.pull_threshold;
    }

    fn pull_min_y(&self) -> f64 {
        return self.pos.y - self.pull_threshold;
    }

    fn shake(&mut self, ticks: f64, force: f64) {
        self.shake_remaining = ticks;
        self.shake_force = force;
    }
}

// make the camera.
fn m_cam(target: &Player) -> Camera {
    let c = Camera {
        pos: m_vec(target.x, target.y),

        // how far from center of screen target must
        // be before camera starts following.
        // allows for movement in center without camera
        // constantly moving.
        pull_threshold: 16.0,

        // min and max positions of camera.
        // the edges of the level.
        pos_min: m_vec(64.0, 64.0),
        pos_max: m_vec(320.0, 64.0),

        shake_remaining: 0.0,
        shake_force: 0.0,
    };

    return c;
}

// game flow
// ------------------------------

pub struct GameState {
    ticks: usize,
    p1: Player,
    cam: Camera,
}

impl GameState {
    fn new() -> GameState {
        let ticks = 0;
        let mut p1 = m_player(64.0, 100.0);
        p1.set_anim("walk");
        let cam = m_cam(&p1);

        GameState { ticks, p1, cam }
    }

    // reset the game to its initial
    // state. use this instead of
    // _init()
    fn reset(&mut self) {
        self.ticks = 0;
        self.p1 = m_player(64.0, 100.0);
        self.p1.set_anim("walk");
        self.cam = m_cam(&self.p1);
    }

    // p8 functions
    // ------------------------------

    fn _init(&mut self) {
        self.reset();
    }

    fn _update60(&mut self) {
        self.ticks += 1;
        //~ self.p1.update();
        self.cam.update(&self.p1);
        // demo camera shake
        //~ if btnp(4, input) {
        //~ self.cam.shake(15.0, 2.0)
        //~ }
        //~ if btnp(4, input) {
        //~ self.cam.shake(15.0, 2.0)
        //~ }
    }

    fn _draw(&mut self) {
        cls(0.0);

        {
            let (x, y) = self.cam.cam_pos();
            camera(x, y);
        }

        map(0.0, 0.0, 0.0, 0.0, 128.0, 128.0);

        self.p1.draw();

        // hud
        camera(0.0, 0.0);

        //~ printc("adv. micro platformer", 64.0, 4.0, 7.0 ,0.0 ,0.0 );
    }
}

fn main() {
    let mut game = GameState::new();

    game._init();

    for _ in 0..60 {
        game._update60();
        game._draw();
    }

    game.reset();
}
