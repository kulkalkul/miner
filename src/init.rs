use std::collections::HashMap;

use crate::prelude::*;

pub async fn init(assets: Assets) -> Game {
    let bump = Bump::new();

    let config = Config {
        LADDERS_TO_RESET: 8,
    };
    
    let mut player = Player {
        trans: Transform {
            pos: WORLD_SPAWN_F32 * Vec2::splat(TILE_SIDE_F32),
            size: vec2(10.0, 12.0),
            offset: vec2(-7.5, 0.0),
        },
        sprite: assets.player_idle.derive_sprite(),
        anim: assets.player_idle.derive_anim(),
        carrying: Array::with_length(ItemKind::Air, 0),
        last_positions: Box::new([vec2(0.0, 0.0); 24]),
        bag_mesh: GameMesh::new(),
    };

    player.last_positions[0] = player.trans.pos;

    let player = player;

    let statue = SimpleEntity {
        trans: Transform {
            pos: (STATUE * ivec2(CHUNK_SIDE_I32, CHUNK_SIDE_I32)).as_vec2(),
            size: vec2(32.0, 48.0),
            offset: vec2(0.0, 0.0),
        },
        sprite: assets.statue.derive_sprite(),
        anim: assets.statue.derive_anim(),
    };

    let crusher = Crusher {
        trans: Transform {
            pos: player.trans.pos,
            size: vec2(256.0, 128.0),
            offset: vec2(0.0, 0.0),
        },
        sprite: assets.crusher_working.derive_sprite(),
        anim: assets.crusher_working.derive_anim(),
    };

    let minecart = Minecart {
        trans: Transform {
            pos: (MINECART_START * ivec2(CHUNK_SIDE_I32, CHUNK_SIDE_I32) + ivec2(0, 2)).as_vec2(),
            size: vec2(15.0, 7.0),
            offset: vec2(0.0, 0.0),
        },
        sprite: assets.minecart_idle.derive_sprite(),
        anim: assets.minecart_idle.derive_anim(),
        carrying: Array::new(Item::default()),
        cooldown: 0.0,
        movement: MinecartMovement::Idle,
    };

    
    let world = World::new(&assets.tile_set, &bump);

    Game {
        total_time: 0.0,
        dev_mode: false,
        bump,
        assets,
        config: config.clone(),
        world,
        visible_chunks: Vec::with_capacity(16),
        money: 0,

        coins: Vec::with_capacity(240),
        
        player,
        statue,
        crusher,
        minecart,

        ui_show_statue: false,
        
        tile_durability_map: HashMap::with_capacity(32),
        
        player_max_carrying: 24,
        
        derived: DerivedState::default(),
        late_derived: LateDerivedState::default(),
        action: ActionState::default(),
        input_actions: InputActions::default(),
    }
}
