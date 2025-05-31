use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub trans: Transform,
    pub sprite: Sprite,
    pub anim: Animation,
    pub bag_mesh: GameMesh,
    pub carrying: Array<ItemKind, 24>,
    pub last_positions: Box<[Vec2; 24]>,
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
    pub sprite: Sprite,
    pub anim: Animation,
    pub carrying: Array<Item, 48>,
    pub cooldown: f32,
    pub movement: MinecartMovement,
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
            ItemKind::IronOre => 15,
            ItemKind::GoldOre => 50,
            ItemKind::RawEmerald => 500,
            ItemKind::RawRuby => 750,
            ItemKind::RawSapphire => 1_000,
        }
    }
}


impl Transform {
    pub fn collider(&self) -> BoxCollider {
        BoxCollider {
            p1: self.offset + self.pos,
            p2: self.offset + self.pos + self.size,
        }
    }
}

impl BoxCollider {
    pub fn intersects(&self, collidee: BoxCollider) -> bool {
        self.p1.x <= collidee.p2.x && self.p2.x >= collidee.p1.x &&
        self.p1.y <= collidee.p2.y && self.p2.y >= collidee.p1.y
    }
}
