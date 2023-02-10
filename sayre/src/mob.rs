use std::collections::HashMap;

use super::snd::SoundClip;
use super::vector::Vector;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    #[default]
    Player,
    Enemy,
}

/*-- whether this mob is controlled by a player
-- 0 is AI, anything else is that player's number*/
#[derive(Default, Clone)]
pub enum Controller {
    #[default]
    Dummy,
    Ai(AiController),
    Player(usize),
}

#[derive(Default, Clone)]
pub struct AiController {}

impl AiController {
    pub fn tick(&mut self) {
        todo!()
    }
}

/*
    -- rotation_types:
    -- add - dir number is added to sprite number
    -- rotate - sprite is rotated numerically
*/
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum RotationType {
    #[default]
    Rotate,
    Add,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Right = 0,
    Down,
    Left,
    Up,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum MobState {
    #[default]
    Walk,
}

#[derive(Default, Clone)]
pub struct Resistances {
    // TODO: Fields TBD because i don't know what the two damages are
    //~ damage = {0, 0},
    //~ knockback = {0, 0},
}

#[derive(Default, Clone)]
pub struct InputState {
    left: usize,
    down: usize,
    right: usize,
    up: usize,
    // TODO: figure out what goes in here
    //~ hold_time = {},
}

#[derive(Default, Clone)]
pub struct CooldownMap(HashMap<AttackType, usize>);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    #[default]
    Touch,
    Melee,
    Projectile,
}

// TODO: break this up into different components
#[derive(Default, Clone)]
pub struct Mob {
    // probably required to actually a mob
    state: MobState,
    pos: Vector<2>,
    delta_pos: Vector<2>,
    dir: usize,

    // name
    name: String,

    // team
    team: Team,

    // controllable
    input: InputState,
    controller: Controller,

    // attacking related
    cooldowns: CooldownMap,

    // attackable
    hearts: u8,
    resistances: Resistances,
    hitbox: Option<Hitbox>,

    // drawable
    draworder: usize,
    rotation_type: RotationType,
    //~ animation: Animation,

    // not sure how best to implement this because of aliasing rules
    // copy for now?
    last_damage_source: Option<Box<Mob>>,
}

#[derive(Default, Clone)]
pub struct Hitbox {
    corner: Vector<2>,
    dimensions: Vector<2>,
}

impl InputState {
    fn get_gamepad_input(_player_number: usize) -> InputState {
        todo!()
    }
}

impl Hitbox {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Hitbox {
        Hitbox {
            corner: Vector([x, y]),
            dimensions: Vector([width, height]),
        }
    }

    pub fn centered_from_dimensions(width: f64, height: f64) -> Hitbox {
        let x = -width / 2.0;
        let y = -height / 2.0;
        Hitbox::new(x, y, width, height)
    }

    pub fn square_from_side_length(side: f64) -> Hitbox {
        Hitbox::centered_from_dimensions(side, side)
    }
}

impl Mob {
    fn set_draw_order(&mut self) {
        todo!()
    }

    fn update_state(&mut self) {
        todo!()
        /*
        if self.state == 'walk' or self.state == 'invuln' then
            local delta = Vector:new{
                self.input.right - self.input.left,
                self.input.down - self.input.up,
            }
            local grid = 8
            local xdir, ydir = 0, 0
            local x, y
            if math.abs(delta.x) > math.abs(delta.y) then
                y, x, xdir = alignmove(self.pos.y, self.pos.x, math.abs(delta.x), delta.x, grid)
            else -- by not handling x == y separately we have one axis (y) which is favored when a diagonal is pressed
                x, y, ydir = alignmove(self.pos.x, self.pos.y, math.abs(delta.y), delta.y, grid)
            end

            self.knockback_delta = self.knockback_delta or Vector:new{0, 0}
            self.delta_pos = self.delta_pos + (Vector:new{x, y} - self.pos) * self.speed + self.knockback_delta
            self.knockback_delta = self.knockback_delta / 2
            if self.knockback_delta:mag() < 0.01 then self.knockback_delta = Vector:new{0, 0} end


            local prevdir = self.dir

            --FIXME: makes for sliding, but you face the right way so
            xdir, ydir = delta:unpack()
            if xdir > 0 then xdir = 1 end
            if xdir < 0 then xdir = -1 end
            if ydir > 0 then ydir = 1 end
            if ydir < 0 then ydir = -1 end
            if ydir == 0 then
                if xdir == 1 then
                    self.dir = 0
                elseif xdir == -1 then
                    self.dir = 2
                end
            else
                if ydir == 1 then
                    self.dir = 1
                elseif ydir == -1 then
                    self.dir = 3
                end
            end

        elseif self.state == 'attack' then
        else
            error('mob in invalid state "' .. tostring(self.state) .. '"')
        end
        */
    }

    fn update_cooldowns(&mut self) {
        /*
        if self.projectile_cooldown and self.projectile_cooldown > 0 then
            self.projectile_cooldown = self.projectile_cooldown - 1
        end
        */
        todo!()
    }

    fn env_collision(&mut self) {
        todo!()
    }

    fn update_animation(&mut self) {
        todo!()
        /*
        if self.anim and self.anim.states then
            local current = self.anim.states[self.state]
            if current then
                if self.anim.start then
                    self.anim.start = false
                    self.anim.timer = current.timer
                else
                    self.anim.timer = self.anim.timer - 1
                    if type(current.tick) == 'function' then
                        current.tick(self)
                    end
                    if self.anim.timer <= 0 and type(current.finish) == 'function' then
                        current.finish(self)
                    end
                end
            else
                --print('warning: ' .. self.name .. ' has no animation for state ' .. tostring(self.state))
            end
        end
        */
    }

    fn update(&mut self) {
        self.delta_pos = Vector([0.0, 0.0]);

        match &mut self.controller {
            Controller::Dummy => (),
            Controller::Ai(ai) => ai.tick(),
            Controller::Player(player_number) => {
                self.input = InputState::get_gamepad_input(*player_number)
            }
        }

        self.set_draw_order();

        self.update_cooldowns();

        self.update_state();

        self.env_collision();
    }
}

fn alignmove(
    mut base_axis: f64,
    mut second_axis: f64,
    base_speed: f64,
    second_speed: f64,
    grid_size: f64,
) -> (f64, f64, f64) {
    assert!(base_speed != 0.0);
    assert!(second_speed != 0.0);

    let off = base_axis % grid_size;
    let half = (grid_size / 2.0).floor();
    let dir = {
        if off == 0.0 {
            second_axis = second_axis + second_speed;
            if second_speed > 0.0 {
                1.0
            } else if second_speed < 0.0 {
                -1.0
            } else {
                unreachable!()
            }
        } else if off < half {
            base_axis = base_axis - base_speed;
            if base_axis % grid_size > half {
                base_axis = (base_axis / grid_size).floor() * grid_size + grid_size
            }
            -1.0
        } else if off >= half {
            base_axis = base_axis + base_speed;
            if base_axis % grid_size < half {
                base_axis = (base_axis / grid_size).floor() * grid_size
            }
            1.0
        } else {
            unreachable!()
        }
    };

    (base_axis, second_axis, dir)
}

/* mob.lua
function Mob:inherit(template)
  for k, v in pairs(template) do
    if type(v) == 'table' then
      if k == 'collision' then
        local c = {
          tags = v.tags and recursive_copy(v.tags) or {},
          mob = self,
          onhit = v.onhit,
          on_hit_env = v.on_hit_env,
          screenborder_timer = v.screenborder_timer,
        }
        if v.rotating_hitbox then
          c.rotating_hitbox = new_hitbox(v.rotating_hitbox)
          c.hitbox = new_hitbox(v.rotating_hitbox)
        end
        if v.hitbox then
          c.hitbox = new_hitbox(v.hitbox)
        end
        self[k] = c
      elseif type(v.dup) == 'function' then
        self[k] = v:dup()
      else
        self[k] = recursive_copy(v)
      end
    else
      self[k] = v
    end
  end

  if self.hearts and not self.health then
    self.health = self.hearts * 16
  end

  setmetatable(self, ThisClass)
  return self
end
*/

/*

function Mob:setDrawOrder()
    self.draworder = self.pos.y * 512 + self.pos.x
end

function Mob:take_damage(col)
    if self.state ~= 'invuln' then
        local damage = 0
        if type(col) == 'number' then
            damage = col
            error("[DEPRECATED]")
        else
            if col.tags.damage then
                damage = col.tags.damage
            end

            if col.mob then
                self.last_hit_by = col.mob
            end

            local knockback
            if col.tags.knockback then
                if col.tags.knockbacktype == 'parent_dir' then
                    knockback = self.pos - col.mob.pos
                    if knockback == Vector:new{0, 0} then
                        knockback = dir2vec[math.random(0, 3)]
                    else
                        if math.abs(knockback.x) > math.abs(knockback.y) then
                            knockback = dir2vec[0] * (knockback.x > 0 and 1 or -1)
                        else
                            knockback = dir2vec[1] * (knockback.y > 0 and 1 or -1)
                        end
                    end
                    --print(knockback)
                elseif col.tags.knockbacktype == 'opposite_dir' then
                    knockback = -dir2vec[self.dir] * col.tags.knockback
                elseif col.tags.knockbacktype == 'self_dir' then
                    knockback = dir2vec[col.mob.dir] * col.tags.knockback
                else
                    error('invalid knockback type ' .. tostring(col.tags.knockbacktype))
                end
                --self.delta_pos = self.delta_pos + knockback:norm() * col.tags.knockback * (1 - self.resistance.knockback[1]) - self.resistance.knockback[2]
                self.knockback_delta =
                    knockback:norm() * col.tags.knockback * (1 - self.resistance.knockback[1]) - self.resistance.knockback[2]
            end
                --self.pos = self.pos + Vector:new{10, 10}
        end

        if self.take_damage_sound then
            self.take_damage_sound:replay()
        end
        local resistance = self.damage_resistance and self.damage_resistance or 1 -- ???

        local final_damage = damage
        --[[ TODO: make better weakness and resistance system
        if self.weakness and type(col) == 'table' then
            final_damage = self.weakness(col.mob, final_damage)
        end
        ]]
        final_damage = math.ceil(final_damage * resistance)

        self.health = self.health - final_damage
        self:start_state'invuln'
    end
end

function Mob:heal(ammount)
    ammount = ammount or self.hearts * HEART_VALUE
    self.health = math.min(self.health + ammount, self.hearts * HEART_VALUE)
end

function Mob:is_dead()
    return self.health <= 0
end

function Mob:start_state(state)
    self.state = state
    if self.anim then
        if self.anim.states[state] and self.anim.states[state] then
            self.anim.state = self.anim.states[state]
            self.anim.timer = 0
            self.anim.start = true
        else
            self.anim.state = false
            self.anim.timer = 0
            self.anim.start = false
        end
    end
end

Mob.changeState = Mob.start_state

function Mob:isInvuln()
    return self.state == 'invuln'
end

function Mob:recieve_input()
    self.input_last_frame = {}
    for k, v in pairs(self.input) do
        self.input_last_frame[k] = v
    end

    for _, v in ipairs{'up', 'down', 'left', 'right'} do
        self.input[v] = love.keyboard.isDown(v) and 1 or 0
    end
    self.input.use_left = love.keyboard.isDown('z') and 1 or 0
    self.input.use_right = love.keyboard.isDown('x') and 1 or 0
    self.input.use_left_reserve = love.keyboard.isDown('a') and 1 or 0
    self.input.use_right_reserve = love.keyboard.isDown('s') and 1 or 0
    self.input.left_shoulder = love.keyboard.isDown('lshift') and 1 or 0
    self.input.right_shoulder = love.keyboard.isDown('lctrl') and 1 or 0

    self.input_hold_time = self.input_hold_time or {}
    for k, v in pairs(self.input) do
        --if v == 1 then
            --self.input_hold_time[k] = (self.input_hold_time[k] or 0) + 1
        --else
            --self.input_hold_time[k] = false
        --end
        self.input_hold_time[k] = v == 1 and ((self.input_hold_time[k] or 0) + 1) or 0
    end
--  prinspect(self.input_hold_time)
end

function Mob:do_death()
    local mobs = {}
    if self.drops then
        for _, v in pairs(self.drops) do
            local item = v:roll()
            if item then
                table.insert(mobs, item:make_pickup(self.pos))
            end
        end
    end
    if self.on_death then
        local mobs2 = self:on_death()
        if mobs2 then
            for _, v in ipairs(mobs2) do
                table.insert(mobs, v)
            end
        end
    end
    return mobs
end

function Mob:overlaps(other)
    if self.collision and other.collision then
        local hb0 = self.collision.hitbox
        local topleft0 = self.pos + hb0.corner
        local bottomright0 = topleft0 + hb0.dim

        local hb1 = other.collision.hitbox
        local topleft1 = other.pos + hb1.corner
        local bottomright1 = topleft1 + hb1.dim

        local minkowski = {
            topleft = topleft0 - bottomright1,
            dim = hb0.dim + hb1.dim,
        }

        minkowski.bottomright = minkowski.topleft + minkowski.dim

        if
            minkowski.topleft.x < 0 and
            minkowski.topleft.y < 0 and
            minkowski.bottomright.x > 0 and
            minkowski.bottomright.y > 0 then
                return true -- return penetration vector instead?
        else
            return false
        end
    else
        error'mobs do not both have collision data'
    end
end

return Mob
*/

/* main.lua
function Mob:draw(pos_offset)
    love.graphics.setColor(Color.FullBright)
    local tile_offset = 0
    local palette_offset = 0
    local rotation = 0
    local palette
    if self.state == 'invuln' and self.invuln_palette then
        --tile_offset = self.invuln_offset or 0
        palette = self.invuln_palette
    else
        palette = self.palette or 0
    end
    if self.rotation_type == 'add' then
        tile_offset = tile_offset + self.dir
    elseif self.rotation_type == 'rotate' then
        rotation = self.dir
    end
    if self.anim then
        if self.anim.tile_offset then
            tile_offset = tile_offset + self.anim.tile_offset
        end
        if self.anim.palette_offset then
            palette_offset = palette_offset + self.anim.palette_offset
        end
    end
    local half = TILESIZE / 2
    --sprites:drawSprite(self.sprite + offset, self.pos.x, self.pos.y, math.deg(90 * 1), 1, 1, half, half)
    if not self.sprite then error('tried to draw mob ' .. self.name .. ' but it has no sprite') end
    local pos = self.pos + pos_offset

    sprites:drawSpriteRecolor(palette + palette_offset, self.sprite + tile_offset, math.floor(pos.x), math.floor(pos.y), rotation)
end

function Mob:draw_hitbox()
    if self.collision then
        love.graphics.setColor(Color.Hitbox)
        local v = self.pos + self.collision.hitbox.corner
        local d = self.collision.hitbox.dim
        love.graphics.rectangle('fill', v.x, v.y, d.x, d.y)
    end
end

function Mob:draw_hearts_as_bar_and_numbers(x, y)
    local tile
    local health = self.counter and self.counter.health or self.health
    for i = 0, self.hearts - 1 do
        if health >= (i + 1) * HEART_VALUE then
            tile = TILE.BAR_FULL
        elseif health < i * HEART_VALUE then
            tile = TILE.BAR
        else
            tile = TILE.BAR + math.floor((health % HEART_VALUE))
        end
        sprites:drawSprite(tile, x + 8 * (i % 8), y - 8 * math.floor(i / 8), 0)
    end

    local place = 1
    for i = 0, 2 do
        place = place * 10
        sprites:drawSprite(TILE.NUMERALS + math.floor(health / (place / 10)) % 10, x - 4 * i + 32, y - 8, 0)
    end
end

function Mob:draw_hearts(x, y)
    local tile
    local health = self.counter and self.counter.health or self.health
    for i = 0, self.hearts - 1 do
        if health >= (i + 1) * HEART_VALUE then
            tile = TILE.HEART_FULL
        elseif health < i * HEART_VALUE then
            tile = TILE.HEART
        else
            tile = TILE.HEART + math.floor((health % HEART_VALUE) / 4)
        end
        sprites:drawSpriteRecolor(PALETTE.RED, tile, x + 8 * (i % 8), y - 8 * math.floor(i / 8), 0)
    end
end

function Mob:env_collision()
    local try_pos = self.pos + self.delta_pos
    local half = TILESIZE / 2

    -- screen extents/borders
    --local topleft = TILEVEC + HALFTILEVEC
    --local bottomright = Vector:new{10, 6} * TILEVEC + HALFTILEVEC
    local topleft = HALFTILEVEC
    local bottomright = Vector:new{11, 7} * TILEVEC + HALFTILEVEC

    local x, y = try_pos:unpack()
    --if x >= half and x <= 11 * TILESIZE + half and y >= half and y <= 7 * TILESIZE + half then
        --self.pos = try_pos
    --end
    if self.collision then
        if not self.collision.tags.leave_screen or type(self.collision.on_hit_env) == 'function' then
            local hit = false
            local hitdir
            if x < topleft.x then
                x = topleft.x
                hit = 'screenborder'
                hitdir = 2
            elseif x > bottomright.x then
                x = bottomright.x
                hit = 'screenborder'
                hitdir = 0
            end
            if y < topleft.y then
                y = topleft.y
                hit = 'screenborder'
                hitdir = 3
            elseif y > bottomright.y then
                y = bottomright.y
                hit = 'screenborder'
                hitdir = 1
            end
            if hit then
                if type(self.collision.on_hit_env) == 'function' then
                    self.collision:on_hit_env(hit, hitdir)
                end
            else
                if self.collision.screenborder_timer then
                    self.collision.screenborder_timer = 0
                end
            end
        else
            -- weapons, etc.
        end
    end
    self.pos = Vector:new{x, y}
    self.delta_pos = self.delta_pos / 2
    if self.delta_pos:mag() < 0.01 then self.delta_pos = Vector:new{0, 0} end
    return hit
end

function Mob:__lt(other)
    return self.draworder < other.draworder
end
*/
