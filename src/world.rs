use std::collections::{BTreeMap};

use crate::prelude::*;

use crate::tile::{ TileSetAsset, TileChunk };

pub mod consts {
    use super::*;

    pub const WORLD_WIDTH: usize = 32;
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

    pub const RAIL_START: IVec2 = ivec2(WORLD_SPAWN_I32.x+MINE_AREA_WIDTH_I32/2, WORLD_SPAWN_I32.y);
    pub const RAIL_STRAIGHT_END: IVec2 = ivec2(RAIL_START.x+6, WORLD_SPAWN_I32.y);
    pub const RAIL_DIAGONAL_END: IVec2 = ivec2(RAIL_START.x+26, ROOM_END_I32.y+1);
    
    pub const BARRIER_HEIGHT: i32 = 13;
    pub const BARRIER_POS: IVec2 = ivec2(0, (WORLD_HEIGHT_I32-BARRIER_HEIGHT)*CHUNK_SIDE_I32);
    
    pub const MINECART_START: Vec2 = vec2(
        RAIL_START.x as f32*TILE_SIDE_F32 + 3.0,
        RAIL_START.y as f32*TILE_SIDE_F32,
    );
    pub const MINECART_STRAIGHT_END: Vec2 = vec2(
        RAIL_STRAIGHT_END.x as f32*TILE_SIDE_F32 + TILE_SIDE_F32/2.0,
        RAIL_STRAIGHT_END.y as f32*TILE_SIDE_F32,
    );
    pub const MINECART_DIAGONAL_END: Vec2 = vec2(
        RAIL_DIAGONAL_END.x as f32*TILE_SIDE_F32-24.0,
        RAIL_DIAGONAL_END.y as f32*TILE_SIDE_F32-1.0,
    );
    pub const ELEVATOR_CAGE: IVec2 = ivec2(
        STATUE.x-3,
        STATUE.y
    );
    pub const ELEVATOR_PLATFORM_START: Vec2 = vec2(
        ELEVATOR_CAGE.x as f32*TILE_SIDE_F32,
        ELEVATOR_CAGE.y as f32*TILE_SIDE_F32 - 5.0,
    );
    pub const ELEVATOR_PLATFORM_END: Vec2 = vec2(
        ELEVATOR_PLATFORM_START.x,
        (BARRIER_POS.y - 2) as f32*TILE_SIDE_F32,
    );
    pub const ELEVATOR_PLATFORM_END_STOP: Vec2 = vec2(
        ELEVATOR_PLATFORM_START.x,
        (BARRIER_POS.y - 2) as f32*TILE_SIDE_F32-21.0,
    );
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
            tiles: [Tile::BackgroundStone; CHUNK_SIZE],
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
                tile: Tile::Stone,
                x: BARRIER_POS.x, y: BARRIER_POS.y,
                // BUG: When presented with x=0 and w=160, it should be able to query tiles *between* [0 and 160).
                // But due to the chunk_pos calculation resulting chunk at the end is not a valid chunk. While
                // SetTileArea is guarded from that with min/max checks, chunk index calculation is not, hence
                // for that reason this is doing -1 in both dimensions, painting 1 tile less. It isn't noticeable
                // by player, but must be fixed at some point.
                width: WORLD_WIDTH_I32*CHUNK_SIDE_I32-1, height: BARRIER_HEIGHT*CHUNK_SIDE_I32-1,
            },
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
        

        // INFO: Min >= 1 always spawns, Min < 1 it spawns with Max/abs(diff). So, if Min(0) Max(1), it is 1/2,
        // Min(-1) Max(1), it is 1/3 etc.
        struct Min(i32);
        struct Max(u32);

        let mut ores_map = BTreeMap::new();

        let ores = [
            ( 2 , Tile::StoneCopperOre, Min(4 ), Max(8 ) ),
            ( 2 , Tile::StoneIronOre  , Min(3 ), Max(6 ) ),
            
            ( 3 , Tile::StoneCopperOre, Min(6 ), Max(10) ),
            ( 3 , Tile::StoneIronOre  , Min(5 ), Max(10) ),
            
            ( 4 , Tile::StoneCopperOre, Min(10), Max(14) ),
            ( 4 , Tile::StoneIronOre  , Min(8 ), Max(14) ),
            
            ( 5 , Tile::StoneGoldOre  , Min(0 ), Max(1 ) ),
            ( 5 , Tile::StoneCopperOre, Min(8 ), Max(12) ),
            ( 5 , Tile::StoneIronOre  , Min(14), Max(14) ),

            ( 7 , Tile::StoneGoldOre  , Min(1 ), Max(2 ) ),
            
            ( 8 , Tile::StoneCopperOre, Min(5 ), Max(8 ) ),
            ( 8 , Tile::StoneIronOre  , Min(18), Max(18) ),
            ( 8 , Tile::StoneEmerald  , Min(-7), Max(1 ) ),
            
            ( 9 , Tile::StoneGoldOre  , Min(1 ), Max(7 ) ),
            ( 9 , Tile::StoneIronOre  , Min(14), Max(10) ),
            ( 12, Tile::StoneGoldOre  , Min(4 ), Max(8 ) ),            
        ];

        let mut ores_i = 0;
        
        for chunk_y in 0..BARRIER_HEIGHT {
            while ores_i < ores.len() {
                let (wanted_chunk_y, tile, Min(min), Max(max)) = ores[ores_i];
                
                if wanted_chunk_y > chunk_y { break; }

                ores_i += 1;

                ores_map.insert(tile, (Min(min), Max(max)));
            }
            
            for chunk_x in 0..WORLD_WIDTH_I32 {
                let chunk_pos = ivec2(chunk_x, WORLD_HEIGHT_I32-chunk_y-1);

                for (&tile, &(Min(min), Max(max))) in ores_map.iter() {
                    let gen_count = i32::max(rand::gen_range(min, max as i32 + 1), 0);
                    let mut local_tile_poses = Vec::with_capacity_in(gen_count as usize, bump);
                    for _ in 0..gen_count {
                        let x = rand::gen_range(0, CHUNK_SIDE_I32);
                        let y = rand::gen_range(0, CHUNK_SIDE_I32);
                        local_tile_poses.push(ivec2(x, y));
                    }
                    commands.push_commands(&[ WorldCommand::SetTilesInChunk { chunk_pos, local_tile_poses, tile } ]);
                }

            }
        }

        struct HMin(usize);
        struct HMax(usize);
        struct Width(usize);

        let hard_stone_pass = [
            (4 , HMin(8 ), HMax(24), Width(3 )),
            (5 , HMin(8 ), HMax(24), Width(3 )),
            (6 , HMin(10), HMax(48), Width(4 )),
            (7 , HMin(10), HMax(48), Width(4 )),
            (8 , HMin(12), HMax(80), Width(6 )),
            (10, HMin(48), HMax(80), Width(10)),
            // (12, HMin(48), HMax(80), Width(10)),
            // (14, HMin(48), HMax(80), Width(10)),
            // (16, HMin(48), HMax(80), Width(10)),
            // (18, HMin(30), HMax(50), Width(4 )),
            // (20, HMin(10), HMax(20), Width(4 )),
        ];

        for (chunk_y, HMin(min), HMax(max), Width(width)) in hard_stone_pass {
            let mut tile_y = (WORLD_HEIGHT_I32 - chunk_y) * CHUNK_SIDE_I32 + 8;
            let mut guide_points = Vec::with_capacity_in(WORLD_WIDTH * CHUNK_SIDE, bump);

            let chunk_tile_begin_y = chunk_pos_to_tile_pos(ivec2(0, chunk_y)).y;
            let chunk_tile_end_y = chunk_pos_to_tile_pos(ivec2(0, chunk_y+1)).y - 1;
            
            for tile_x in 0..WORLD_WIDTH_I32 * CHUNK_SIDE_I32 {
                const MAX: i32 = 5;
                let direction_rand = rand::gen_range(0, MAX);

                let direction = if direction_rand < 1 {
                    -1
                } else if direction_rand >= MAX-1 {
                    1
                } else {
                    0
                };

                tile_y += direction;

                // INFO: Ensures any layer is not outside of its own chunk
                if tile_y <= chunk_tile_begin_y || tile_y >= chunk_tile_end_y {
                    tile_y += -direction*2;
                }
                
                guide_points.push(ivec2(tile_x, tile_y));
            }

            let guide_points = &guide_points[..WORLD_WIDTH*CHUNK_SIDE];

            let mut i = 0;
            while i < WORLD_WIDTH*CHUNK_SIDE {
                let stride = rand::gen_range(min, max);
                let space = rand::gen_range(1, 6);

                let random_x = rand::gen_range(-8, 9);
                let random_y = rand::gen_range(-4, 5);

                let offset = ivec2(random_x, random_y);

                for stride_i in i..usize::min(i+stride, WORLD_WIDTH*CHUNK_SIDE) {
                    let guide_point = guide_points[stride_i];
                    let width = rand::gen_range(width-1, width+1) as i32;
                    for width_i in 0..width {
                        commands.set_tile(offset+guide_point+ivec2(0, width_i), Tile::HardStone);
                    }
                }
                i += stride + space;
            }
        }

        let mut stepping_stone_i = 0;

        while stepping_stone_i < WORLD_WIDTH_I32*CHUNK_SIDE_I32 {
            let x = rand::gen_range(4, 10);
            stepping_stone_i = i32::min(stepping_stone_i+x, WORLD_WIDTH_I32*CHUNK_SIDE_I32);

            let height = rand::gen_range(2, 5);
            let smallest_width = rand::gen_range(2, 4);
            let mut x_offsets = Vec::with_capacity_in(height as usize, &bump);
            let mut widths = Vec::with_capacity_in(height as usize, &bump);

            let mut next_width = smallest_width;
            let mut next_x_offset = 0;
            let to_mid = height/2;
            let to_end = height-to_mid;

            for _ in 0..to_mid {
                widths.push(next_width);
                x_offsets.push(next_x_offset);
                next_width += 2;
                next_x_offset -= 1;
            }
            for _ in 0..to_end {
                widths.push(next_width);
                x_offsets.push(next_x_offset);
                next_width -= 2;
                next_x_offset += 1;
            }

            let barrier_point = (WORLD_HEIGHT_I32-BARRIER_HEIGHT)*CHUNK_SIDE_I32 - 16;
            let local_y_offset = rand::gen_range(-4, 4);

            for (i, y) in (barrier_point..barrier_point+height).enumerate() {
                let local_x_offset = rand::gen_range(-2, 2);
                let width = widths[i];
                let x_offset = local_x_offset + x_offsets[i];
                let x = local_x_offset + x_offsets[i] + stepping_stone_i;
                let y = local_y_offset + y;

                commands.set_tile_area(ivec2(x, y), ivec2(width, 1), Tile::Stone);
            }
        }

        let mut scatter_i = 0;
        let mut smallest_scatter_y = WORLD_HEIGHT_I32*CHUNK_SIDE_I32;

        while scatter_i < WORLD_WIDTH_I32*CHUNK_SIDE_I32 {
            scatter_i += rand::gen_range(2, 8);

            let mut y = (WORLD_HEIGHT_I32-BARRIER_HEIGHT)*CHUNK_SIDE_I32 - 16;
            for _ in 0..32 {
                y -= rand::gen_range(8, 24);
                let offset_x = rand::gen_range(-4, 4);
                let offset_y = rand::gen_range(-4, 4);
                
                let center_top = ivec2(offset_x+scatter_i, offset_y+y);

                let half_width = rand::gen_range(1, 4);
                let height = rand::gen_range(2, 5);

                let height = i32::min(height, half_width);

                let mut start = center_top-ivec2(-half_width, 0);
                let mut size = ivec2(half_width*2, height);

                for _ in 0..height {
                    commands.set_tile_area(start, size, Tile::Stone);
                    start.y -= 1;
                    start.x -= 1;
                    size.x -= 1;
                }
            }
            
            smallest_scatter_y = i32::min(smallest_scatter_y, y);
        }
        world.apply_commands(commands);

        let mut commands = world.commands(&bump);
        let tiles = world.tiles();
                
        for y in smallest_scatter_y..(WORLD_HEIGHT_I32-BARRIER_HEIGHT)*CHUNK_SIDE_I32 {
            for x in 0..WORLD_WIDTH_I32*CHUNK_SIDE_I32 {
                if tiles.at_tile_pos(ivec2(x, y)).kind == Tile::Stone {
                    let chance = rand::gen_range(0, 300);
                    if chance <= 4 {
                        commands.set_tile(ivec2(x, y), Tile::StoneSapphire);
                    } else if chance <= 6 {
                        commands.set_tile(ivec2(x, y), Tile::StoneRuby);
                    } else if chance <= 10 {
                        commands.set_tile(ivec2(x, y), Tile::StoneEmerald);
                    } else if chance <= 40 {
                        commands.set_tile(ivec2(x, y), Tile::StoneGoldOre);
                    }
                }
            }
        }
        
        commands.set_tile_area(
            ivec2(0, (WORLD_HEIGHT_I32-BARRIER_HEIGHT) * CHUNK_SIDE_I32 - 1),
            ivec2(WORLD_WIDTH_I32*CHUNK_SIDE_I32, 1),
            Tile::Barrier,
        );
        
        // INFO: 32 because if player ever goes that down it should be easy to return from
        // there, so it helps with providing landmarkds.
        commands.set_tile_area(
            ivec2(0, smallest_scatter_y),
            ivec2(WORLD_WIDTH_I32*CHUNK_SIDE_I32, 32),
            Tile::HardStone,
        );
        commands.set_tile_area(
            ivec2(0, smallest_scatter_y+32),
            ivec2(WORLD_WIDTH_I32*CHUNK_SIDE_I32, 1),
            Tile::Barrier,
        );
        
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
