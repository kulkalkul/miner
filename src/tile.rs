use crate::prelude::*;

use crate::asset::{ AssetState };

pub mod consts {
    pub const CHUNK_SIDE: usize = 16;
    pub const CHUNK_SIZE: usize = CHUNK_SIDE * CHUNK_SIDE;

    pub const CHUNK_SIDE_I32: i32 = CHUNK_SIDE as i32;
    pub const CHUNK_SIZE_I32: i32 = CHUNK_SIZE as i32;

    pub const CHUNK_SIDE_F32: f32 = CHUNK_SIDE as f32;
    pub const CHUNK_SIZE_F32: f32 = CHUNK_SIZE as f32;
}

use consts::*;

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Tile {
    Empty,
    
    ERR,
    UP,
    DOWN,
    GREEN,
    RED,
    
    BackgroundStone,
    BackgroundStoneLadder,
    BackgroundStoneLadderSupport,
    BackgroundStoneElevatorLeft,
    BackgroundStoneElevatorMiddle,
    BackgroundStoneElevatorRight,
    
    Stone,
    StoneDig,
    StoneDontDig,
    BackgroundStoneDontDig,
    BackgroundStoneLadderDontDig,
    HardStone,
    Barrier,
    WorldBoundary,

    StoneCopperOre,
    StoneIronOre,
    StoneGoldOre,
    StoneEmerald,
    StoneRuby,
    StoneSapphire,

    StoneBoundaryBottomRight,
    StoneBoundaryBottom,
    StoneBoundaryBottomLeft,
    StoneBoundaryLeft,
    StoneBoundaryTopLeft,
    StoneBoundaryTop,
    StoneBoundaryTopRight,
    StoneBoundaryRight,

    StoneBoundaryTopLeftInverse,
    StoneBoundaryTopRightInverse,
    StoneBoundaryBottomRightInverse,
    StoneBoundaryBottomLeftInverse,
}

impl Tile {
    pub const COUNT: usize = std::mem::variant_count::<Self>();
}

#[derive(Copy, Clone, Debug, Default)]
pub struct TileBounds {
    pub begin: Vec2,
    pub end: Vec2,
}

pub struct TileSetAsset {
    pub asset_id: u64,
    pub texture: Texture2D,
    pub bounds: [TileBounds; Tile::COUNT],
}

#[derive(Copy, Clone)]
pub struct TileChunk {
    pub tiles: [Tile; CHUNK_SIZE],
}

pub fn load_tile_set(asset_state: &mut AssetState, texture: &Texture2D, tile_size: IVec2) -> TileSetAsset {    
    let asset_id = asset_state.asset_id;
    asset_state.asset_id += 1;

    let texture = texture.clone();
    let width = tile_size.x as f32;
    let height = tile_size.y as f32;
    
    let texture_size = texture.size();
    texture.set_filter(FilterMode::Nearest);

    let mut bounds = [TileBounds::default(); Tile::COUNT];

    let rows_per_col = (texture_size.x / width) as i32;
    let cols = Tile::COUNT as i32 / rows_per_col as i32;

    // Using 1.0 as starting point for x, y because of the padding.
    // Same is also the case for side + 2.0 calculations.

    let mut i = 0;
    let mut x = 1.0f32;
    let mut y = 1.0f32;
    
    for _ in 0..i32::max(cols-1, 0) {
        for _ in 0..rows_per_col {
            bounds[i] = TileBounds { begin: vec2(x, y) / texture_size, end: vec2(x+width, y+height) / texture_size };
            i += 1;
            x += width + 2.0;
        }
        y += height + 2.0;
        x = 1.0;
    }

    let remaining_rows = Tile::COUNT - i;
    for _ in 0..remaining_rows {
        bounds[i] = TileBounds { begin: vec2(x, y) / texture_size, end: vec2(x+width, y+height) / texture_size };
        i += 1;
        x += width + 2.0;
    }

    bounds[0] = TileBounds::default();
    
    TileSetAsset {
        asset_id,
        texture,
        bounds,
    }
}

impl Tile {
    pub fn can_walk_through(&self) -> bool {
        match self {
            Tile::Empty => true,
            
            Tile::ERR => false,
            Tile::UP => false,
            Tile::DOWN => false,
            Tile::GREEN => false,
            Tile::RED => false,
            
            Tile::BackgroundStone => true,
            Tile::BackgroundStoneLadder => true,
            Tile::BackgroundStoneLadderSupport => true,
            Tile::BackgroundStoneElevatorLeft => false,
            Tile::BackgroundStoneElevatorMiddle => false,
            Tile::BackgroundStoneElevatorRight => false,
            
            Tile::Stone => false,
            Tile::StoneDig => false,
            Tile::StoneDontDig => false,
            Tile::BackgroundStoneDontDig => false,
            Tile::BackgroundStoneLadderDontDig => false,
            Tile::HardStone => false,
            Tile::Barrier => false,
            Tile::WorldBoundary => false,
            
            Tile::StoneCopperOre => false,
            Tile::StoneIronOre => false,
            Tile::StoneGoldOre => false,
            Tile::StoneEmerald => false,
            Tile::StoneRuby => false,
            Tile::StoneSapphire => false,
            
            Tile::StoneBoundaryBottomRight => false,
            Tile::StoneBoundaryBottom => false,
            Tile::StoneBoundaryBottomLeft => false,
            Tile::StoneBoundaryLeft => false,
            Tile::StoneBoundaryTopLeft => false,
            Tile::StoneBoundaryTop => false,
            Tile::StoneBoundaryTopRight => false,
            Tile::StoneBoundaryRight => false,
            Tile::StoneBoundaryTopLeftInverse => false,
            Tile::StoneBoundaryTopRightInverse => false,
            Tile::StoneBoundaryBottomRightInverse => false,
            Tile::StoneBoundaryBottomLeftInverse => false,
        }
    }

    pub fn can_mine(&self) -> bool {
        match self {
            Tile::Empty => false,
            
            Tile::ERR => false,
            Tile::UP => false,
            Tile::DOWN => false,
            Tile::GREEN => false,
            Tile::RED => false,
            
            Tile::BackgroundStone => false,
            Tile::BackgroundStoneLadder => false,
            Tile::BackgroundStoneLadderSupport => false,
            Tile::BackgroundStoneElevatorLeft => false,
            Tile::BackgroundStoneElevatorMiddle => false,
            Tile::BackgroundStoneElevatorRight => false,
            
            Tile::Stone => true,
            Tile::StoneDig => true,
            Tile::StoneDontDig => false,
            Tile::BackgroundStoneDontDig => false,
            Tile::BackgroundStoneLadderDontDig => false,
            Tile::HardStone => true,
            Tile::Barrier => false,
            Tile::WorldBoundary => false,
            
            Tile::StoneCopperOre => true,
            Tile::StoneIronOre => true,
            Tile::StoneGoldOre => true,
            Tile::StoneEmerald => true,
            Tile::StoneRuby => true,
            Tile::StoneSapphire => true,
            
            Tile::StoneBoundaryBottomRight => false,
            Tile::StoneBoundaryBottom => false,
            Tile::StoneBoundaryBottomLeft => false,
            Tile::StoneBoundaryLeft => false,
            Tile::StoneBoundaryTopLeft => false,
            Tile::StoneBoundaryTop => false,
            Tile::StoneBoundaryTopRight => false,
            Tile::StoneBoundaryRight => false,
            Tile::StoneBoundaryTopLeftInverse => false,
            Tile::StoneBoundaryTopRightInverse => false,
            Tile::StoneBoundaryBottomRightInverse => false,
            Tile::StoneBoundaryBottomLeftInverse => false,
        }
    }
    pub fn can_climb(&self) -> bool {
        match self {
            Tile::Empty => false,
            
            Tile::ERR => false,
            Tile::UP => false,
            Tile::DOWN => false,
            Tile::GREEN => false,
            Tile::RED => false,
            
            Tile::BackgroundStone => false,
            Tile::BackgroundStoneLadder => true,
            Tile::BackgroundStoneLadderSupport => true,
            Tile::BackgroundStoneElevatorLeft => false,
            Tile::BackgroundStoneElevatorMiddle => false,
            Tile::BackgroundStoneElevatorRight => false,
            
            
            Tile::Stone => false,
            Tile::StoneDig => false,
            Tile::StoneDontDig => false,
            Tile::BackgroundStoneDontDig => false,
            Tile::BackgroundStoneLadderDontDig => false,
            Tile::HardStone => false,
            Tile::Barrier => false,
            Tile::WorldBoundary => false,
            
            Tile::StoneCopperOre => false,
            Tile::StoneIronOre => false,
            Tile::StoneGoldOre => false,
            Tile::StoneEmerald => false,
            Tile::StoneRuby => false,
            Tile::StoneSapphire => false,
            
            Tile::StoneBoundaryBottomRight => false,
            Tile::StoneBoundaryBottom => false,
            Tile::StoneBoundaryBottomLeft => false,
            Tile::StoneBoundaryLeft => false,
            Tile::StoneBoundaryTopLeft => false,
            Tile::StoneBoundaryTop => false,
            Tile::StoneBoundaryTopRight => false,
            Tile::StoneBoundaryRight => false,
            Tile::StoneBoundaryTopLeftInverse => false,
            Tile::StoneBoundaryTopRightInverse => false,
            Tile::StoneBoundaryBottomRightInverse => false,
            Tile::StoneBoundaryBottomLeftInverse => false,
        }
    }
    pub fn is_air(&self) -> bool {
        match self {
            Tile::Empty => true,
            
            Tile::ERR => false,
            Tile::UP => false,
            Tile::DOWN => false,
            Tile::GREEN => false,
            Tile::RED => false,
            
            Tile::BackgroundStone => true,
            Tile::BackgroundStoneLadder => true,
            Tile::BackgroundStoneLadderSupport => true,
            Tile::BackgroundStoneElevatorLeft => false,
            Tile::BackgroundStoneElevatorMiddle => false,
            Tile::BackgroundStoneElevatorRight => false,
            
            Tile::Stone => false,
            Tile::StoneDig => false,
            Tile::StoneDontDig => false,
            Tile::BackgroundStoneDontDig => false,
            Tile::BackgroundStoneLadderDontDig => false,
            Tile::HardStone => false,
            Tile::Barrier => false,
            Tile::WorldBoundary => false,
            
            Tile::StoneCopperOre => false,
            Tile::StoneIronOre => false,
            Tile::StoneGoldOre => false,
            Tile::StoneEmerald => false,
            Tile::StoneRuby => false,
            Tile::StoneSapphire => false,
            
            Tile::StoneBoundaryBottomRight => false,
            Tile::StoneBoundaryBottom => false,
            Tile::StoneBoundaryBottomLeft => false,
            Tile::StoneBoundaryLeft => false,
            Tile::StoneBoundaryTopLeft => false,
            Tile::StoneBoundaryTop => false,
            Tile::StoneBoundaryTopRight => false,
            Tile::StoneBoundaryRight => false,
            Tile::StoneBoundaryTopLeftInverse => false,
            Tile::StoneBoundaryTopRightInverse => false,
            Tile::StoneBoundaryBottomRightInverse => false,
            Tile::StoneBoundaryBottomLeftInverse => false,
        }
    }

    pub fn mine_results_tile(&self) -> Tile {
        match self {
            Tile::Empty => Tile::ERR,
            
            Tile::ERR => Tile::ERR,
            Tile::UP => Tile::ERR,
            Tile::DOWN => Tile::ERR,
            Tile::GREEN => Tile::ERR,
            Tile::RED => Tile::ERR,
            
            Tile::BackgroundStone => Tile::ERR,
            Tile::BackgroundStoneLadder => Tile::ERR,
            Tile::BackgroundStoneLadderSupport => Tile::ERR,
            Tile::BackgroundStoneElevatorLeft => Tile::ERR,
            Tile::BackgroundStoneElevatorMiddle => Tile::ERR,
            Tile::BackgroundStoneElevatorRight => Tile::ERR,
            
            Tile::Stone => Tile::BackgroundStone,
            Tile::StoneDig => Tile::BackgroundStone,
            Tile::StoneDontDig => Tile::ERR,
            Tile::BackgroundStoneDontDig => Tile::ERR,
            Tile::BackgroundStoneLadderDontDig => Tile::ERR,
            Tile::HardStone => Tile::BackgroundStone,
            Tile::Barrier => Tile::ERR,
            Tile::WorldBoundary => Tile::ERR,
            
            Tile::StoneCopperOre => Tile::BackgroundStone,
            Tile::StoneIronOre => Tile::BackgroundStone,
            Tile::StoneGoldOre => Tile::BackgroundStone,
            Tile::StoneEmerald => Tile::BackgroundStone,
            Tile::StoneRuby => Tile::BackgroundStone,
            Tile::StoneSapphire => Tile::BackgroundStone,
            
            Tile::StoneBoundaryBottomRight => Tile::ERR,
            Tile::StoneBoundaryBottom => Tile::ERR,
            Tile::StoneBoundaryBottomLeft => Tile::ERR,
            Tile::StoneBoundaryLeft => Tile::ERR,
            Tile::StoneBoundaryTopLeft => Tile::ERR,
            Tile::StoneBoundaryTop => Tile::ERR,
            Tile::StoneBoundaryTopRight => Tile::ERR,
            Tile::StoneBoundaryRight => Tile::ERR,
            Tile::StoneBoundaryTopLeftInverse => Tile::ERR,
            Tile::StoneBoundaryTopRightInverse => Tile::ERR,
            Tile::StoneBoundaryBottomRightInverse => Tile::ERR,
            Tile::StoneBoundaryBottomLeftInverse => Tile::ERR,
        }
    }

    pub fn item_drop(&self) -> ItemKind {
        match self {
            Tile::Empty => ItemKind::Air,
            
            Tile::ERR => ItemKind::Air,
            Tile::UP => ItemKind::Air,
            Tile::DOWN => ItemKind::Air,
            Tile::GREEN => ItemKind::Air,
            Tile::RED => ItemKind::Air,
            
            Tile::BackgroundStone => ItemKind::Air,
            Tile::BackgroundStoneLadder => ItemKind::Air,
            Tile::BackgroundStoneLadderSupport => ItemKind::Air,
            Tile::BackgroundStoneElevatorLeft => ItemKind::Air,
            Tile::BackgroundStoneElevatorMiddle => ItemKind::Air,
            Tile::BackgroundStoneElevatorRight => ItemKind::Air,
            
            Tile::Stone => ItemKind::Air,
            Tile::StoneDig => ItemKind::Air,
            Tile::StoneDontDig => ItemKind::Air,
            Tile::BackgroundStoneDontDig => ItemKind::Air,
            Tile::BackgroundStoneLadderDontDig => ItemKind::Air,
            Tile::HardStone => ItemKind::Air,
            Tile::Barrier => ItemKind::Air,
            Tile::WorldBoundary => ItemKind::Air,

            Tile::StoneCopperOre => ItemKind::CopperOre,
            Tile::StoneIronOre => ItemKind::IronOre,
            Tile::StoneGoldOre => ItemKind::GoldOre,
            Tile::StoneEmerald => ItemKind::RawEmerald,
            Tile::StoneRuby => ItemKind::RawRuby,
            Tile::StoneSapphire => ItemKind::RawSapphire,
            
            Tile::StoneBoundaryBottomRight => ItemKind::Air,
            Tile::StoneBoundaryBottom => ItemKind::Air,
            Tile::StoneBoundaryBottomLeft => ItemKind::Air,
            Tile::StoneBoundaryLeft => ItemKind::Air,
            Tile::StoneBoundaryTopLeft => ItemKind::Air,
            Tile::StoneBoundaryTop => ItemKind::Air,
            Tile::StoneBoundaryTopRight => ItemKind::Air,
            Tile::StoneBoundaryRight => ItemKind::Air,
            Tile::StoneBoundaryTopLeftInverse => ItemKind::Air,
            Tile::StoneBoundaryTopRightInverse => ItemKind::Air,
            Tile::StoneBoundaryBottomRightInverse => ItemKind::Air,
            Tile::StoneBoundaryBottomLeftInverse => ItemKind::Air,
        }
    }

    pub fn durability(&self) -> f32 {
        match self {
            Tile::Empty => NOT_SO_VERY_BIG_NUMBER,
            
            Tile::ERR => NOT_SO_VERY_BIG_NUMBER,
            Tile::UP => NOT_SO_VERY_BIG_NUMBER,
            Tile::DOWN => NOT_SO_VERY_BIG_NUMBER,
            Tile::GREEN => NOT_SO_VERY_BIG_NUMBER,
            Tile::RED => NOT_SO_VERY_BIG_NUMBER,
            
            Tile::BackgroundStone => NOT_SO_VERY_BIG_NUMBER,
            Tile::BackgroundStoneLadder => NOT_SO_VERY_BIG_NUMBER,
            Tile::BackgroundStoneLadderSupport => NOT_SO_VERY_BIG_NUMBER,
            Tile::BackgroundStoneElevatorLeft => NOT_SO_VERY_BIG_NUMBER,
            Tile::BackgroundStoneElevatorMiddle => NOT_SO_VERY_BIG_NUMBER,
            Tile::BackgroundStoneElevatorRight => NOT_SO_VERY_BIG_NUMBER,
            
            Tile::Stone => 1.2,
            Tile::StoneDig => 0.5,
            Tile::StoneDontDig => NOT_SO_VERY_BIG_NUMBER,
            Tile::BackgroundStoneDontDig => NOT_SO_VERY_BIG_NUMBER,
            Tile::BackgroundStoneLadderDontDig => NOT_SO_VERY_BIG_NUMBER,
            Tile::HardStone => 60.0,
            Tile::Barrier => NOT_SO_VERY_BIG_NUMBER,
            Tile::WorldBoundary => NOT_SO_VERY_BIG_NUMBER,

            Tile::StoneCopperOre => 1.4,
            Tile::StoneIronOre => 1.8,
            Tile::StoneGoldOre => 2.5,
            Tile::StoneEmerald => 3.5,
            Tile::StoneRuby => 3.5,
            Tile::StoneSapphire => 3.5,
            
            Tile::StoneBoundaryBottomRight => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryBottom => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryBottomLeft => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryLeft => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryTopLeft => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryTop => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryTopRight => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryRight => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryTopLeftInverse => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryTopRightInverse => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryBottomRightInverse => NOT_SO_VERY_BIG_NUMBER,
            Tile::StoneBoundaryBottomLeftInverse => NOT_SO_VERY_BIG_NUMBER,
        }
    }
}
