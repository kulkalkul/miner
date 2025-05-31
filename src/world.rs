use std::collections::{BTreeMap};

use crate::prelude::*;

use crate::tile::{ TileSetAsset, TileChunk };

pub mod consts {
    use super::*;

    pub const WORLD_WIDTH: usize = 10;
    pub const WORLD_HEIGHT: usize = 128;
    pub const WORLD_SIZE: usize = WORLD_WIDTH * WORLD_HEIGHT;
    pub const TILE_SIDE: usize = 16;

    pub const WORLD_WIDTH_I32: i32 = WORLD_WIDTH as i32;
    pub const WORLD_HEIGHT_I32: i32 = WORLD_HEIGHT as i32;
    pub const WORLD_SIZE_I32: i32 = WORLD_SIZE as i32;
    pub const TILE_SIDE_I32: i32 = TILE_SIDE as i32;

    pub const WORLD_WIDTH_F32: f32 = WORLD_WIDTH as f32;
    pub const WORLD_HEIGHT_F32: f32 = WORLD_HEIGHT as f32;
    pub const WORLD_SIZE_F32: f32 = WORLD_SIZE as f32;
    pub const TILE_SIDE_F32: f32 = TILE_SIDE as f32;

    pub const WORLD_SPAWN_I32: IVec2 = ivec2(
        ( WORLD_WIDTH * CHUNK_SIDE / 2 ) as i32 + 1,
        ( (WORLD_HEIGHT - 2) * CHUNK_SIDE ) as i32  + 1,
    );
    pub const WORLD_SPAWN_F32: Vec2 = vec2(WORLD_SPAWN_I32.x as f32, WORLD_SPAWN_I32.y as f32);    

    pub const MINE_AREA_WIDTH_I32: i32 = 8;
        
    pub const ROOM_START_I32: IVec2 = ivec2(WORLD_SPAWN_I32.x - CHUNK_SIDE_I32*2, WORLD_SPAWN_I32.y);
    pub const ROOM_END_I32: IVec2 = ROOM_START_I32.wrapping_add(ivec2(CHUNK_SIDE_I32*4, 8));
    
    pub const STATUE: IVec2 = ivec2(WORLD_SPAWN_I32.x-MINE_AREA_WIDTH_I32/2-2, WORLD_SPAWN_I32.y);

    pub const MINECART_START: IVec2 = ivec2(WORLD_SPAWN_I32.x+MINE_AREA_WIDTH_I32/2, WORLD_SPAWN_I32.y);
    pub const MINECART_END: IVec2 = ivec2(ROOM_END_I32.x-1, WORLD_SPAWN_I32.y);
}

use consts::*;

pub struct World {
    pub chunks: Vec<TileChunk>,
    pub meshes: Vec<GameMesh>,
    pub recalculate_all_meshes: bool,
}

#[derive(Clone, Copy)]
pub struct WorldTiles<'w> {
    pub chunks: &'w Vec<TileChunk>,
}

impl<'w> WorldTiles<'w> {
    pub fn at_world_pos(&self, world_pos: Vec2) -> WorldTile<'w> {
        let tile_pos = world_pos_to_tile_pos(world_pos);
        self.at_tile_pos(tile_pos)
    }
    pub fn at_tile_pos(&self, tile_pos: IVec2) -> WorldTile<'w> {
        let chunk_pos = tile_pos_to_chunk_pos(tile_pos);
        let chunk = &self.chunks[chunk_index_at(chunk_pos)];
        
        WorldTile {
            world_tiles: WorldTiles { chunks: self.chunks },
            pos: tile_pos,
            kind: chunk.tiles[tile_index_at(tile_pos)],
        }
    }
}

#[derive(Clone, Copy)]
pub struct WorldTile<'w> {
    pub world_tiles: WorldTiles<'w>,
    pub pos: IVec2,
    pub kind: Tile,
}

impl<'w> WorldTile<'w> {
    pub fn tile_index(&self) -> usize {
        tile_index_at(self.pos)
    }
    pub fn world_pos(&self) -> Vec2 {
        tile_pos_to_world_pos(self.pos)
    }
    pub fn chunk_pos(&self) -> IVec2 {
        tile_pos_to_chunk_pos(self.pos)
    }

    pub fn another_tile(&self, tile_pos: IVec2) -> WorldTile<'w> {
        self.world_tiles.at_tile_pos(tile_pos)
    }
    pub fn offset_by(&self, amount: IVec2) -> WorldTile<'w> {
        self.another_tile(self.pos + amount)
    }
    pub fn up(&self, amount: i32) -> WorldTile<'w> {
        self.another_tile(self.pos + ivec2(0, amount))
    }
    pub fn down(&self, amount: i32) -> WorldTile<'w> {
        self.another_tile(self.pos + ivec2(0, -amount))
    }
    pub fn left(&self, amount: i32) -> WorldTile<'w> {
        self.another_tile(self.pos + ivec2(-amount, 0))
    }
    pub fn right(&self, amount: i32) -> WorldTile<'w> {
        self.another_tile(self.pos + ivec2(amount, 0))
    }
}

impl<'w> From<WorldTile<'w>> for IVec2 {
    fn from(value: WorldTile<'w>) -> Self {
        value.pos
    }
}

pub mod conversions {
    use super::*;

    // index_at
    pub fn tile_index_at(tile_pos: IVec2) -> usize {
        tile_pos.y as usize % CHUNK_SIDE * CHUNK_SIDE + tile_pos.x as usize % CHUNK_SIDE
    }
    pub fn local_tile_index_at(local_tile_pos: IVec2) -> usize {
        local_tile_pos.y as usize * CHUNK_SIDE + local_tile_pos.x as usize
    }
    pub fn chunk_index_at(chunk_pos: IVec2) -> usize {
        chunk_pos.y as usize * WORLD_WIDTH + chunk_pos.x as usize
    }
    
    // chunk_pos ->
    pub fn chunk_pos_to_world_pos(chunk_pos: IVec2) -> Vec2 {
        vec2(
            (chunk_pos.x * CHUNK_SIDE_I32 * TILE_SIDE_I32) as f32,
            (chunk_pos.y * CHUNK_SIDE_I32 * TILE_SIDE_I32) as f32,
        )
    }
    pub fn chunk_pos_to_tile_pos(chunk_pos: IVec2) -> IVec2 {
        ivec2(chunk_pos.x * CHUNK_SIDE_I32, chunk_pos.y * CHUNK_SIDE_I32)
    }

    // tiles_pos ->
    pub fn tile_pos_to_chunk_pos(tile_pos: IVec2) -> IVec2 {
        ivec2(tile_pos.x / CHUNK_SIDE_I32, tile_pos.y / CHUNK_SIDE_I32)
    }
    pub fn tile_pos_to_world_pos(tile_pos: IVec2) -> Vec2 {
        vec2((tile_pos.x * TILE_SIDE_I32) as f32, (tile_pos.y * TILE_SIDE_I32) as f32)
    }
    
    // world_pos ->
    pub fn world_pos_to_chunk_pos(world_pos: Vec2) -> IVec2 {
        ivec2(
            world_pos.x as i32 / TILE_SIDE_I32 / CHUNK_SIDE_I32,
            world_pos.y as i32 / TILE_SIDE_I32 / CHUNK_SIDE_I32,
        )
    }
    pub fn world_pos_to_tile_pos(world_pos: Vec2) -> IVec2 {
        ivec2(
            world_pos.x as i32 / TILE_SIDE_I32,
            world_pos.y as i32 / TILE_SIDE_I32,
        )
    }
}

pub use conversions::*;

pub struct WorldCommands<'b> {
    bump: &'b Bump,
    commands: Vec<WorldCommand<'b>, &'b Bump>,
}

impl<'b> WorldCommands<'b> {
    pub fn bump(&self) -> &Bump {
        self.bump
    }
    fn push_commands(&mut self, commands: &[WorldCommand<'b>]) {
        self.commands.extend_from_slice(commands);
    }
    pub fn recalculate_all_meshes(&mut self) {
        self.commands.push(WorldCommand::RecalculateAllMeshes);
    }
    pub fn set_tile(&mut self, tile_pos: IVec2, tile: Tile) {
        self.commands.push(WorldCommand::SetTile { x: tile_pos.x, y: tile_pos.y, tile });
    }
    pub fn set_tiles(&mut self, tile_poses: Vec<IVec2, &'b Bump>, tile: Tile) {
        self.commands.push(WorldCommand::SetTiles { tile_poses, tile });
    }
    pub fn set_tiles_in_chunk(&mut self, chunk_pos: IVec2, local_tile_poses: Vec<IVec2, &'b Bump>, tile: Tile) {
        self.commands.push(WorldCommand::SetTilesInChunk { chunk_pos, local_tile_poses, tile });
    }
    pub fn set_tile_area(&mut self, begin_pos: IVec2, size: IVec2, tile: Tile) {
        self.commands.push(
            WorldCommand::SetTileArea { x: begin_pos.x, y: begin_pos.y, width: size.x, height: size.y, tile }
        );
    }
}

#[derive(Clone)]
enum WorldCommand<'b> {
    RecalculateAllMeshes,
    SetTile { x: i32, y: i32, tile: Tile },
    SetTiles { tile_poses: Vec<IVec2, &'b Bump>, tile: Tile },
    SetTilesInChunk { chunk_pos: IVec2, local_tile_poses: Vec<IVec2, &'b Bump>, tile: Tile },
    SetTileArea { x: i32, y: i32, width: i32, height: i32, tile: Tile },
}

impl World {
    pub fn new(tile_set: &TileSetAsset, bump: &Bump) -> Self {
        let tile_chunk = TileChunk {
            dirty: true,
            tiles: [Tile::Stone; CHUNK_SIZE],
        };

        let mut tile_mesh = Mesh {
            vertices: vec![Vertex::new(0.0, 0.0, 0.0, 0.0, 0.0, [0, 0, 0, 0].into()); CHUNK_SIZE * 4],
            indices: vec![0; CHUNK_SIZE * 6],
            texture: Some(tile_set.texture.clone()),
        };


        {
            let vertices = &mut tile_mesh.vertices[..];
            let mut x = 0.0;
            let mut y = 0.0;
            let mut i = 0;
            for _ in 0..CHUNK_SIDE {
                for _ in 0..CHUNK_SIDE {
                    vertices[i+0].position = vec3(x              , y              , 0.0);
                    vertices[i+1].position = vec3(x+TILE_SIDE_F32, y              , 0.0);
                    vertices[i+2].position = vec3(x+TILE_SIDE_F32, y+TILE_SIDE_F32, 0.0);
                    vertices[i+3].position = vec3(x              , y+TILE_SIDE_F32, 0.0);
                    
                    vertices[i+0].color = [255, 255, 255, 255];
                    vertices[i+1].color = [255, 255, 255, 255];
                    vertices[i+2].color = [255, 255, 255, 255];
                    vertices[i+3].color = [255, 255, 255, 255];
                                        
                    i += 4;
                    x += TILE_SIDE_F32;
                }
                y += TILE_SIDE_F32;
                x = 0.0;
            }
        }
        
        {
            let indices = &mut tile_mesh.indices[..];
            let mut i = 0;
            let mut vi = 0;
            for _ in 0..CHUNK_SIZE {
                indices[i+0] = vi+0;
                indices[i+1] = vi+1;
                indices[i+2] = vi+2;
                indices[i+3] = vi+0;
                indices[i+4] = vi+2;
                indices[i+5] = vi+3;

                vi += 4;
                i += 6;
            }
        }        
        
        let room_x = ROOM_START_I32.x;
        let room_y = ROOM_START_I32.y;
        let room_w = ROOM_END_I32.x - ROOM_START_I32.x;
        let room_h = ROOM_END_I32.y - ROOM_START_I32.y;

        let mut world = World {
            chunks: vec![tile_chunk; WORLD_SIZE],
            meshes: vec![GameMesh(tile_mesh); WORLD_SIZE],
            recalculate_all_meshes: false,
        };

        let mut commands = world.commands(bump);

        commands.push_commands(&[
            WorldCommand::SetTileArea {
                tile: Tile::BackgroundStone,
                x: room_x, y: room_y, width: room_w, height: room_h,
            },
            WorldCommand::SetTile {
                tile: Tile::StoneBoundaryBottomLeftInverse,
                x: room_x-1, y: room_y-1,
            },
            WorldCommand::SetTile {
                tile: Tile::StoneBoundaryTopLeftInverse,
                x: room_x-1, y: room_y+room_h,
            },
            WorldCommand::SetTile {
                tile: Tile::StoneBoundaryBottomRightInverse,
                x: room_x+room_w, y: room_y-1,
            },
            WorldCommand::SetTile {
                tile: Tile::StoneBoundaryTopRightInverse,
                x: room_x+room_w, y: room_y+room_h,
            },
            WorldCommand::SetTileArea {
                tile: Tile::StoneBoundaryBottom,
                x: room_x, y: room_y-1, width: room_w, height: 1,
            },
            WorldCommand::SetTileArea {
                tile: Tile::StoneBoundaryLeft,
                x: room_x-1, y: room_y, width: 1, height: room_h,
            },
            WorldCommand::SetTileArea {
                tile: Tile::StoneBoundaryTop,
                x: room_x, y: room_y+room_h, width: room_w, height: 1,
            },
            WorldCommand::SetTileArea {
                tile: Tile::StoneBoundaryRight,
                x: room_x+room_w, y: room_y, width: 1, height: room_h,
            },
            WorldCommand::SetTileArea {
                tile: Tile::StoneDig,
                x: room_x + room_w/2 - 4, y: room_y-1, width: 8, height: 1,
            },
        ]);

        struct Min(usize);
        struct Max(usize);

        let mut tiles_map = BTreeMap::new();

        let tiles = [
            ( 0, Tile::StoneCopperOre, Min(8) , Max(12)  ),
            ( 0, Tile::StoneIronOre  , Min(0) , Max(3)  ),
            ( 0, Tile::StoneGoldOre  , Min(0) , Max(1)  ),
            
            ( 1, Tile::StoneIronOre  , Min(3) , Max(6)  ),
            ( 0, Tile::StoneGoldOre  , Min(1) , Max(2)  ),
            
            ( 1, Tile::StoneCopperOre, Min(5) , Max(9)  ),
            ( 0, Tile::StoneIronOre  , Min(5) , Max(9)  ),

            ( 1, Tile::StoneGoldOre  , Min(1) , Max(4)  ),
            
            ( 1, Tile::StoneCopperOre, Min(6) , Max(12)  ),
            ( 0, Tile::StoneIronOre  , Min(6) , Max(12)  ),
            
            ( 2, Tile::StoneGoldOre  , Min(1) , Max(7)  ),
            ( 0, Tile::StoneEmerald  , Min(0) , Max(1)  ),
            ( 0, Tile::StoneRuby     , Min(0) , Max(1)  ),
            ( 0, Tile::StoneSapphire , Min(0) , Max(1)  ),
        ];

        let mut tiles_i = 0;
        let mut last_generated_y = WORLD_HEIGHT_I32 - WORLD_SPAWN_I32.y / CHUNK_SIDE_I32;
        
        for chunk_y in 0..WORLD_HEIGHT_I32 {
            while tiles_i < tiles.len() {
                let (delta_y, tile, Min(min), Max(max)) = tiles[tiles_i];
                
                if last_generated_y + delta_y > chunk_y { break; }
                last_generated_y += delta_y;

                tiles_i += 1;

                tiles_map.insert(tile, (Min(min), Max(max)));
            }
            
            for chunk_x in 0..WORLD_WIDTH_I32 {
                let chunk_pos = ivec2(chunk_x, WORLD_HEIGHT_I32-chunk_y-1);

                for (&tile, &(Min(min), Max(max))) in tiles_map.iter() {
                    let gen_count = rand::gen_range(min, max + 1);
                    let mut local_tile_poses = Vec::with_capacity_in(gen_count, bump);
                    for _ in 0..gen_count {
                        let x = rand::gen_range(0, CHUNK_SIDE_I32);
                        let y = rand::gen_range(0, CHUNK_SIDE_I32);
                        local_tile_poses.push(ivec2(x, y));
                    }
                    commands.push_commands(&[ WorldCommand::SetTilesInChunk { chunk_pos, local_tile_poses, tile } ]);
                }

            }
        }

        commands.push_commands(&[ WorldCommand::RecalculateAllMeshes ]);
        world.apply_commands(commands);

        world
    }

    pub fn tiles<'w>(&'w self) -> WorldTiles<'w> {
        WorldTiles {
            chunks: &self.chunks,
        }
    }
    pub fn commands<'b>(&self, bump: &'b Bump) -> WorldCommands<'b> {
        let commands = Vec::with_capacity_in(32, bump);
        WorldCommands { bump, commands }
    }
    pub fn apply_commands<'b>(&mut self, world_commands: WorldCommands<'b>) {
        
        for command in world_commands.commands {
            match command {
            | WorldCommand::RecalculateAllMeshes => {
                self.recalculate_all_meshes = true;
            },
            | WorldCommand::SetTile { x, y, tile } => {
                let chunk_pos = tile_pos_to_chunk_pos(ivec2(x, y));
                let tile_index = tile_index_at(ivec2(x, y));
                
                let chunk = &mut self.chunks[chunk_index_at(chunk_pos)];
                chunk.tiles[tile_index] = tile;
                chunk.dirty = true;
            },
            | WorldCommand::SetTiles { tile_poses, tile } => {
                let chunks = &mut self.chunks[..];
                for tile_pos in tile_poses {
                    let chunk_pos = tile_pos_to_chunk_pos(tile_pos);
                    let tile_index = tile_index_at(ivec2(tile_pos.x, tile_pos.y));
                    
                    let chunk = &mut chunks[chunk_index_at(chunk_pos)];
                    chunk.tiles[tile_index] = tile;
                    chunk.dirty = true;
                }
            },
            | WorldCommand::SetTilesInChunk { chunk_pos, local_tile_poses, tile } => {
                let chunk = &mut self.chunks[chunk_index_at(chunk_pos)];
                for tile_pos in local_tile_poses {
                    let tile_index = local_tile_index_at(ivec2(tile_pos.x, tile_pos.y));
                    
                    chunk.tiles[tile_index] = tile;
                    chunk.dirty = true;
                }
            },
            | WorldCommand::SetTileArea { x, y, width, height, tile } => {
                let chunks = &mut self.chunks[..];

                let chunk_positions = World::query_chunks_between_two_tiles(
                    world_commands.bump, ivec2(x, y), ivec2(x+width, y+height)
                );
                for chunk_pos in chunk_positions {
                    let begin_x = i32::max(chunk_pos.x * CHUNK_SIDE_I32, x);
                    let begin_y = i32::max(chunk_pos.y * CHUNK_SIDE_I32, y);
                    
                    let end_x = i32::min((chunk_pos.x+1) * CHUNK_SIDE_I32, x+width);
                    let end_y = i32::min((chunk_pos.y+1) * CHUNK_SIDE_I32, y+height);

                    let chunk = &mut chunks[chunk_index_at(chunk_pos)];
                    chunk.dirty = true;
                    let tiles = &mut chunk.tiles[..];


                    for local_y in begin_y..end_y {
                        for local_x in begin_x..end_x {
                            tiles[tile_index_at(ivec2(local_x, local_y))] = tile;
                        }
                    }
                }
            },
            }
        }
    }
    
    pub fn chunk_at(&self, chunk_pos: IVec2) -> &TileChunk {
        &self.chunks[chunk_index_at(chunk_pos)]
    }
    pub fn mesh_at(&self, chunk_pos: IVec2) -> &GameMesh {
        &self.meshes[chunk_index_at(chunk_pos)]
    }
    pub fn query_chunks_around_chunk_pos(bump: &Bump, origin: IVec2, half_distance: u32) -> Vec<IVec2, &Bump> {
        let half_distance = half_distance as i32;
        let distance = half_distance * 2 + 1;
        
        let mut chunks = Vec::with_capacity_in((distance * distance) as usize, bump);
        
        let x_begin = i32::max(origin.x - half_distance, 0);
        let x_end = i32::min(origin.x + half_distance, WORLD_WIDTH as i32 - 1);

        let y_begin = i32::max(origin.y - half_distance, 0);
        let y_end = i32::min(origin.y + half_distance, WORLD_HEIGHT as i32 - 1);
        
        for y in y_begin..=y_end {
            for x in x_begin..=x_end {
                chunks.push(ivec2(x, y));
            }
        }

        chunks
    }
    pub fn query_chunks_between_two_tiles(bump: &Bump, tile_begin: IVec2, tile_end: IVec2) -> Vec<IVec2, &Bump> {
        let chunk_begin = tile_begin / IVec2::splat(CHUNK_SIDE as i32);
        let chunk_end = tile_end / IVec2::splat(CHUNK_SIDE as i32);
        let chunks_height = chunk_end.y - chunk_begin.y;
        let chunks_width = chunk_end.x - chunk_begin.x;

        let mut chunks = Vec::with_capacity_in((chunks_height * chunks_width) as usize, bump);

        for y in chunk_begin.y..=chunk_end.y {
            for x in chunk_begin.x..=chunk_end.x {
                chunks.push(ivec2(x, y));
            }
        }
        
        chunks
    }

    pub fn query_intersected_tiles_y(bump: &Bump, x: f32, y_range: [f32; 2]) -> Vec<IVec2, &Bump> {
        // Just reusing world_pos_to_tile_pos because lazy
        let begin_tile = world_pos_to_tile_pos(vec2(x, y_range[0]));
        let end_tile = world_pos_to_tile_pos(vec2(x, y_range[1]));

        let mut tiles = Vec::with_capacity_in((y_range[1] - y_range[0]) as usize, bump);

        for y in begin_tile.y..=end_tile.y {
            tiles.push(ivec2(begin_tile.x, y));
        }

        tiles
    }
    pub fn query_intersected_tiles_x(bump: &Bump, x_range: [f32; 2], y: f32) -> Vec<IVec2, &Bump> {
        // Just reusing world_pos_to_tile_pos because lazy
        let begin_tile = world_pos_to_tile_pos(vec2(x_range[0], y));
        let end_tile = world_pos_to_tile_pos(vec2(x_range[1], y));

        let mut tiles = Vec::with_capacity_in((x_range[1] - x_range[0]) as usize, bump);

        for x in begin_tile.x..=end_tile.x {
            tiles.push(ivec2(x, begin_tile.y));
        }

        tiles
    }
    
    // INFO: I should probably embed tile_set either as Arc or some sort of reverse asset_id lookup
    // or I could add texture and bounds predraw. Use that texture without cloning in Mesh via
    // mem::swap. So both draw and apply_updates would be sync in which tileset to use.
    pub fn apply_updates(&mut self, tile_set: &TileSetAsset) {
        if self.recalculate_all_meshes {
            for (mesh, chunk) in self.meshes.iter_mut().zip(self.chunks.iter()) {
                if !chunk.dirty { continue; };
                if mesh.0.texture.is_none() { continue; };

                let vertices = &mut mesh.0.vertices[..];

                let mut i = 0;
                for tile in &chunk.tiles {
                    let bounds = tile_set.bounds[*tile as usize];
                    
                    vertices[i+0].uv = vec2(bounds.begin.x, bounds.end.y);
                    vertices[i+1].uv = vec2(bounds.end.x  , bounds.end.y);
                    vertices[i+2].uv = vec2(bounds.end.x  , bounds.begin.y);
                    vertices[i+3].uv = vec2(bounds.begin.x, bounds.begin.y);
                    
                    i += 4;
                }
            }
        }
    }
}
