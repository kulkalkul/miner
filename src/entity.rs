use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub trans: Transform,
    pub tile_size: Vec2,
    pub sprite: Sprite,
    pub anim: Animation,
    pub bag_mesh: GameMesh,
    pub carrying: Array<ItemKind, 64>,
    pub last_positions: Box<[Vec2; 24]>,
    pub mining_fatigue: f32,
    pub climb_momentum: f32,
    pub jetpack_fuel: f32,
    pub jetpack_out_of_fuel_tick: f32,
}

#[derive(Debug)]
pub struct CoinBundle {
    pub trans: Transform,
    pub sprite: Sprite,
    pub amount: i32,
    pub velocity: Vec2,
    pub sine_index: usize,
}

#[derive(Debug)]
pub struct UIEntity {
    pub sprite: Sprite,
    pub anim: Animation,
}

#[derive(Debug)]
pub struct SimpleEntity {
    pub trans: Transform,
    pub sprite: Sprite,
    pub anim: Animation,
}

#[derive(Debug)]
pub struct Crusher {
    pub trans: Transform,
    pub sprite: Sprite,
    pub anim: Animation,
}

#[derive(Debug)]
pub struct Minecart {
    pub trans: Transform,
    pub rotation: f32,
    pub speed: f32,
    pub sprite: Sprite,
    pub anim: Animation,
    pub carrying: Array<Item, 128>,
    pub cooldown: f32,
    pub movement: MinecartMovement,
}

#[derive(Debug)]
pub struct ElevatorPlatform {
    pub trans: Transform,
    pub sprite: Sprite,
    pub anim: Animation,
    pub velocity: Vec2,
    pub down_or_up: bool,
    pub player_inside_for: f32,
    pub walk_collider: Vec2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MinecartMovement {
    Idle,
    Forwards,
    Backwards,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Item {
    pub trans: Transform,
    pub kind: ItemKind,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct DroppedItem {
    pub trans: Transform,
    pub kind: ItemKind,
    pub accumulated_tick: f32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum ItemKind {
    #[default]
    Air,
    CopperOre,
    IronOre,
    GoldOre,
    RawEmerald,
    RawRuby,
    RawSapphire,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Transform {
    pub pos: Vec2,
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Debug, Copy, Clone)]
pub struct BoxCollider {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl ItemKind {
    pub fn value(&self) -> i32 {
        match self {
            ItemKind::Air => 0,
            ItemKind::CopperOre => 5,
            ItemKind::IronOre => 35,
            ItemKind::GoldOre => 110,
            ItemKind::RawEmerald => 350,
            ItemKind::RawRuby => 1500,
            ItemKind::RawSapphire => 2500,
        }
    }
}


impl Transform {
    pub fn collider(&self) -> BoxCollider {
        BoxCollider::new(self.offset+self.pos, self.size)
    }
    pub fn collider_size(&self, size: Vec2) -> BoxCollider {
        let collider = BoxCollider::new(self.offset+self.pos, size);
        BoxCollider {
            p1: Vec2::min(collider.p1, collider.p2),
            p2: Vec2::max(collider.p1, collider.p2),
        }
    }
    pub fn collider_offset_size(&self, offset: Vec2, size: Vec2) -> BoxCollider {
        let collider = BoxCollider::new(self.offset+self.pos+offset, size);
        BoxCollider {
            p1: Vec2::min(collider.p1, collider.p2),
            p2: Vec2::max(collider.p1, collider.p2),
        }
    }
    pub fn offsetted_pos(&self) -> Vec2 {
        self.offset+self.pos
    }
    pub fn x_offsetted_pos(&self) -> Vec2 {
        vec2(self.offset.x+self.pos.x, self.pos.y)
    }
    pub fn y_offsetted_pos(&self) -> Vec2 {
        vec2(self.pos.x, self.offset.y+self.pos.y)
    }
}

impl BoxCollider {
    pub fn new(position: Vec2, size: Vec2) -> BoxCollider {
        BoxCollider {
            p1: position,
            p2: position+size,
        }
    }
    pub fn intersects(&self, other: BoxCollider) -> bool {
        self.p1.x <= other.p2.x && self.p2.x >= other.p1.x &&
        self.p1.y <= other.p2.y && self.p2.y >= other.p1.y
    }
    pub fn collides(&self, mut other: BoxCollider, vel: Vec2) -> Option<(Vec2, Vec2, f32)> {
        if vel.x == 0.0 && vel.y == 0.0 { return None; }

        let size = self.p2 - self.p1;

        other.p1 -= size/2.0;
        other.p2 += size/2.0;

        if let Some((contact_point, contact_normal, contact_time)) = other.ray_collides(self.p1 + size/2.0, vel) {
            if contact_time <= 1.0 {
                return Some((contact_point, contact_normal, contact_time));
            }
        }

        None
    }
    pub fn ray_collides(&self, ray_origin: Vec2, ray_dir: Vec2) -> Option<(Vec2, Vec2, f32)> {
        let mut t_near = (self.p1 - ray_origin)/ray_dir;
        let mut t_far = (self.p2 - ray_origin)/ray_dir;

        if t_near.x > t_far.x { std::mem::swap(&mut t_near.x, &mut t_far.x); }
        if t_near.y > t_far.y { std::mem::swap(&mut t_near.y, &mut t_far.y); }
        
        if t_near.x > t_far.y || t_near.y > t_far.x { return None; }

        let t_hit_near = f32::max(t_near.x, t_near.y);
        let t_hit_far = f32::min(t_far.x, t_far.y);

        if t_hit_far < 0.0 { return None; }

        let contact_point = ray_origin + t_hit_near * ray_dir;
        let mut contact_normal = Vec2::ZERO;

        if t_near.x > t_near.y {
            if ray_dir.x < 0.0 {
                contact_normal = vec2(1.0, 0.0);
            } else {
                contact_normal = vec2(-1.0, 0.0);
            }
        } else if t_near.x < t_near.y {
            if ray_dir.y < 0.0 {
                contact_normal = vec2(0.0, 1.0);
            } else {
                contact_normal = vec2(0.0, -1.0);
            }
        }

        Some((contact_point, contact_normal, t_hit_near))
    }
    pub fn contains(&self, other: BoxCollider) -> bool {
        self.p1.x <= other.p2.x && self.p2.x <= other.p2.x &&
        self.p1.x >= other.p1.x && self.p2.x >= other.p1.x &&
        self.p1.y <= other.p2.y && self.p2.y <= other.p2.y &&
        self.p1.y >= other.p1.y && self.p2.y >= other.p1.y
    }
}
