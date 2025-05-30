#![feature(variant_count)]
#![feature(allocator_api)]
#![feature(one_sided_range)]

#![allow(non_snake_case)]
#![allow(unused_labels)]
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::{LazyLock, Mutex};

use bumpalo::Bump;
use macroquad::prelude::*;

mod init;
mod asset;
mod sprite;
mod render;
mod update;
mod tile;
mod collision;
mod world;
mod entity;
mod derived;
mod action;

use init::   { init };
use asset::  { init_assets };
use asset::  { Assets };
use update:: { update };
use render:: { render };
use world::  { World };
use entity:: { BoxCollider, Crusher, Minecart, Player };
use derived::{ DerivedState };
use action:: { ActionState };

pub mod prelude {
    pub use bumpalo::{ Bump };
    pub use macroquad::prelude::*;

    pub use super::consts::*;

    pub use super::{ debug, debug_point, debug_rect };
    pub use super::{ Game, Config, GameMesh };

    pub use crate::asset::{ Assets };

    pub use crate::sprite::{ Sprite, Animation, SpriteAsset };
    pub use crate::sprite::{ tick_animation };

    pub use crate::entity::*;

    pub use crate::tile::{ Tile, TileBounds };
    pub use crate::tile::consts::*;

    pub use crate::world::{ World };
    pub use crate::world::consts::*;

    pub use crate::derived::{ DerivedState };

    pub use crate::action::{ ActionState };
    pub use crate::action::actions::*;
}

mod consts {
    pub const GAME_WIDTH: usize = 320;
    pub const GAME_HEIGHT: usize = 240;

    pub const GAME_WIDTH_I32: i32 = GAME_WIDTH as i32;
    pub const GAME_HEIGHT_I32: i32 = GAME_HEIGHT as i32;

    pub const GAME_WIDTH_F32: f32 = GAME_WIDTH as f32;
    pub const GAME_HEIGHT_F32: f32 = GAME_HEIGHT as f32;
}

use consts::*;

#[derive(Clone)]
pub struct Config {
    pub LADDERS_TO_RESET: i32,
}

pub struct Game {
    pub dev_mode: bool,
    pub bump: Bump,
    pub assets: Assets,
    pub config: Config,
    pub world: World,
    pub visible_chunks: Vec<IVec2>,

    pub player: Player,
    pub crusher: Crusher,
    pub minecart: Minecart,

    pub tile_durability_map: HashMap<IVec2, f32>,

    pub player_max_carrying: usize,

    pub derived: DerivedState,
    pub action: ActionState,
}

fn window_conf() -> Conf {
    #[cfg(not(target_family = "wasm"))]
    return Conf {
        window_title: "Miner".to_owned(),
        window_width: 1280,
        window_height: 960,
        ..Default::default()
    };
    #[cfg(target_family = "wasm")]
    return Conf {
        window_title: "Miner".to_owned(),
        window_width: 640,
        window_height: 480,
        ..Default::default()
    };
}

#[macroquad::main(window_conf)]
async fn main() {
    let assets = init_assets().await;
    let mut game = init(assets).await;

    loop {
        update(&mut game);
        render(&game);
        next_frame().await;
    }
}

static DEBUG_COMMANDS: LazyLock<Mutex<Vec<DebugCommand>>> = LazyLock::new(|| {
    Mutex::new(Vec::with_capacity(128))
});

enum DebugCommand {
    DebugPoint { pos: Vec2, color: Color },
    DebugRect { rect: Rect, color: Color },
}

pub enum GenericDebug {
    DebugPoint(Vec2),
    DebugRect(Rect),
}

impl From<Vec2> for GenericDebug {
    fn from(value: Vec2) -> Self {
        Self::DebugPoint(value)
    }
}
impl From<Rect> for GenericDebug {
    fn from(value: Rect) -> Self {
        Self::DebugRect(value)
    }
}
impl From<BoxCollider> for GenericDebug {
    fn from(value: BoxCollider) -> Self {
        let x = value.p1.x;
        let y = value.p1.y;
        let w = value.p2.x - value.p1.x;
        let h = value.p2.y - value.p1.y;

        Self::DebugRect(Rect { x, y, w, h })
    }
}

pub fn debug(generic: impl Into<GenericDebug>, color: Color) {
    match generic.into() {
        GenericDebug::DebugPoint(pos) => debug_point(pos, color),
        GenericDebug::DebugRect(rect) => debug_rect(rect, color),
    }
}

pub fn debug_point(pos: Vec2, color: Color) {
    DEBUG_COMMANDS.lock().unwrap().push(DebugCommand::DebugPoint { pos, color });
}
pub fn debug_rect(rect: Rect, color: Color) {
    DEBUG_COMMANDS.lock().unwrap().push(DebugCommand::DebugRect { rect, color });
}

pub fn apply_debug_commands() {
    for command in DEBUG_COMMANDS.lock().unwrap().drain(..) {
        match command {
        | DebugCommand::DebugPoint { pos, color } => draw_rectangle(pos.x, pos.y, 1.0, 1.0, color),
        | DebugCommand::DebugRect { rect, color } => draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, color),
        }
    }
}

pub struct GameMesh(pub Mesh);

impl Clone for GameMesh {
    fn clone(&self) -> Self {
        Self(Mesh {
            vertices: self.0.vertices.clone(),
            indices: self.0.indices.clone(),
            texture: self.0.texture.clone(),
        })
    }
}

impl std::fmt::Debug for GameMesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mesh")
            .field("vertices", &self.0.vertices)
            .field("indices", &self.0.indices)
            .field("texture", &self.0.texture)
            .finish()
    }
}

impl GameMesh {
    pub fn new() -> Self {
        Self(Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        })
    }
}
