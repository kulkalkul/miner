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
            size: vec2(16.0, 16.0),
            offset: vec2(-7.5, 0.0),
        },
        tile_size: vec2(10.0, 12.0),
        sprite: assets.player_idle.derive_sprite(),
        anim: assets.player_idle.derive_anim(),
        carrying: Array::with_length(ItemKind::Air, 0),
        last_positions: Box::new([vec2(0.0, 0.0); 24]),
        bag_mesh: GameMesh::new(),
        mining_fatigue: 0.0,
        climb_momentum: 0.0,
        jetpack_fuel: 0.0,
        jetpack_out_of_fuel_tick: 0.0,
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
            pos: MINECART_START,
            size: vec2(15.0, 7.0),
            offset: vec2(0.0, 0.0),
        },
        rotation: 0.0,
        speed: 50.0,
        sprite: assets.minecart_idle.derive_sprite(),
        anim: assets.minecart_idle.derive_anim(),
        carrying: Array::new(Item::default()),
        cooldown: 0.0,
        movement: MinecartMovement::Idle,
    };

    let elevator_cage = SimpleEntity {
        trans: Transform {
            pos: (ELEVATOR_CAGE * ivec2(CHUNK_SIDE_I32, CHUNK_SIDE_I32)).as_vec2(),
            size: vec2(0.0, 0.0),
            offset: vec2(0.0, 0.0),
        },
        sprite: assets.elevator_cage.derive_sprite(),
        anim: assets.elevator_cage.derive_anim(),
    };

    let elevator_platform_transform = Transform {
        pos: ELEVATOR_PLATFORM_START,
        size: vec2(44.0, 42.0),
        offset: vec2(2.0, 0.0),
    };

    let elevator_platform = ElevatorPlatform {
        trans: elevator_platform_transform,
        sprite: assets.elevator_platform_idle.derive_sprite(),
        anim: assets.elevator_platform_idle.derive_anim(),
        velocity: vec2(0.0, 0.0),
        down_or_up: false,
        player_inside_for: 0.0,
        walk_collider: vec2(44.0, -5.0),
    };

    let ui_inventory_bar_frame = UIEntity {
        sprite: assets.ui_inventory_bar_frame.derive_sprite(),
        anim: assets.ui_inventory_bar_frame.derive_anim(),
    };
    
    let ui_fuel_bar_frame = UIEntity {
        sprite: assets.ui_fuel_bar_frame.derive_sprite(),
        anim: assets.ui_fuel_bar_frame.derive_anim(),
    };
        
    let world = World::new(&assets.tile_set, &bump);
    
    let game_render_target = render_target(screen_width() as u32, screen_height() as u32);
    let shadow_material = load_material(
        ShaderSource::Glsl { vertex: SHADOW_VERTEX_SHADER, fragment: SHADOW_FRAGMENT_SHADER },
        Default::default()
    ).expect("load shader");

    Game {
        total_time: 0.0,
        window_to_draw_size: vec2(1.0, 1.0),
        ui_state: UIState {
            mouse_div: vec2(1.0, 1.0),
            last_clicked_button_hash: None,
        },
        game_render_target,
        shadow_material,
        dev_mode: false,
        bump,
        assets,
        config: config.clone(),
        world,
        visible_chunks: Vec::with_capacity(16),
        money: 0,

        coins: Vec::with_capacity(240),
        dropped_items: Vec::with_capacity(16),
        
        player,
        statue,
        crusher,
        minecart,

        elevator_cage,
        elevator_platform,
        elevator_spawned: false,

        ui_inventory_bar_frame,
        ui_fuel_bar_frame,
        ui_show_statue: false,
        
        tile_durability_map: HashMap::with_capacity(32),
        
        derived: DerivedState::default(),
        late_derived: LateDerivedState::default(),
        action: ActionState::default(),
        input_actions: InputActions::default(),
        upgrades: Default::default(),
    }
}

const SHADOW_VERTEX_SHADER: &'static str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0;
    uv = texcoord;
}
"#;

const SHADOW_FRAGMENT_SHADER: &'static str = r#"#version 100
precision mediump float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;

void main() {
    vec3 res = texture2D(Texture, uv).rgb;
    gl_FragColor = vec4(res, 1.0);
}
"#;
