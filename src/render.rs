use crate::prelude::*;

use crate::apply_debug_commands;
use crate::sprite::{ draw_sprite, draw_sprite_scaled, draw_sprite_offset, draw_ui };
use crate::ui::*;


pub fn render(game: &mut Game) {
    let statue = &game.statue;
    let minecart = &game.minecart;
    let player = &game.player;
    let crusher = &game.crusher;
    let elevator_cage = &game.elevator_cage;
    let elevator_platform = &game.elevator_platform;
    
    let derived = &game.derived;
    let assets = &game.assets;
    let world = &game.world;
    let visible_chunks = &game.visible_chunks;
    let upgrades = &mut game.upgrades;

    let world_origin = Rect {
        x: -(GAME_WIDTH_F32 / 2.0),
        y: -(GAME_HEIGHT_F32 / 2.0),
        w: GAME_WIDTH_F32,
        h: GAME_HEIGHT_F32,
    };
    
    // draw chunks :::
    for &chunk_pos in visible_chunks {
        let world_pos = chunk_pos_to_world_pos(chunk_pos);
        
        let mut mesh_camera_origin = world_origin;
        mesh_camera_origin.x += player.trans.pos.x - world_pos.x;
        mesh_camera_origin.y += player.trans.pos.y - world_pos.y;

        set_camera(&Camera2D::from_display_rect(mesh_camera_origin));
        
        let mesh = world.mesh_at(chunk_pos);
        draw_mesh(&mesh.0);
    }
    
    let mut camera_origin = world_origin;
    camera_origin.x += player.trans.pos.x;
    camera_origin.y += player.trans.pos.y;
    set_camera(&Camera2D::from_display_rect(camera_origin));
    let tiles = world.tiles();
    for (&tile_pos, &durability) in &game.tile_durability_map {
        let tile = tiles.at_tile_pos(tile_pos);
        let index = usize::clamp(((durability/tile.kind.durability())*4.0) as usize, 0, 3);
        draw_sprite(tile.world_pos(), &assets.cracking[index].derive_sprite());
    }
    let _ = tiles;
    
    // draw_sprite_offset(crusher.trans.pos, crusher.trans.offset, &crusher.sprite);

    let rail_start_sprite = assets.rail_start.derive_sprite();
    let rail_sprite = assets.rail.derive_sprite();
    
    draw_sprite(tile_pos_to_world_pos(RAIL_START), &rail_start_sprite);

    // draw rail :::
    for x in RAIL_START.x+1..=RAIL_END.x {
        let tile_pos = ivec2(x, RAIL_START.y);
        let world_pos = tile_pos_to_world_pos(tile_pos);
        draw_sprite(world_pos, &rail_sprite);
    }

    draw_sprite(minecart.trans.pos, &minecart.sprite);
    draw_sprite(statue.trans.pos, &statue.sprite);

    for coin in &game.coins {
        draw_sprite_scaled(
            coin.trans.pos + vec2(0.0, derived.time_sine_4[coin.sine_index]),
            vec2(0.25, 0.25),
            &coin.sprite,
        );
    }
    
    let sprite_dir = if player.sprite.flip_x { vec2(-1.0, 1.0) } else { vec2(1.0, 1.0) };

    if game.elevator_spawned {
        draw_sprite(elevator_platform.trans.pos, &elevator_platform.sprite);
    }

    // draw player :::
    let mut last_position = player.trans.pos + player.trans.offset*sprite_dir;
    let mut half_y = 2.5;
    for &pos in &player.last_positions[..player.carrying.length] {
        let diff = last_position - pos;

        let dir = diff.normalize_or_zero();

        half_y = f32::min(half_y+0.5, 4.5);

        let local_offset = dir.rotate(vec2(0.0, half_y));
        last_position = pos;
        
        let pos1 = pos - local_offset;
        let pos2 = pos + local_offset;        
    
        draw_line(pos1.x, pos1.y, pos2.x, pos2.y, half_y * 2.0, BROWN);
    }
    
    draw_sprite_offset(player.trans.pos, player.trans.offset, &player.sprite);
    
    if game.elevator_spawned {
        draw_sprite(elevator_cage.trans.pos, &elevator_cage.sprite);
    }
    

    // overlay ui

    if derived.ui_show_statue_key {
        let pos = statue.trans.pos + statue.trans.size/2.0 - assets.ui_keys.texture.size()/2.0 * vec2(0.5, 0.5);
        draw_sprite_scaled(pos, vec2(0.5, 0.5), &assets.ui_keys.derive_sprite());
    }
    
    // actual ui
    
    {
        let mut ui_camera_origin = world_origin;
        ui_camera_origin.x += ui_camera_origin.w/2.0;
        ui_camera_origin.y += ui_camera_origin.h/2.0;
        ui_camera_origin.w *= 4.0;
        ui_camera_origin.h *= 4.0;
        
        let mut camera = Camera2D::from_display_rect(ui_camera_origin);
        camera.zoom.y *= -1.0;
        set_camera(&camera);
    }

    let corner_padding = vec2(2.0, 2.0);
    let padding = vec2(1.0, 1.0);
    let mut cursor = vec2(UI_WIDTH_F32, 0.0);

    {
        cursor.x -= (assets.coin.texture.size()).x*2.0 + corner_padding.x;
        cursor.y += corner_padding.y;
        draw_ui(cursor, vec2(2.0, 2.0), &assets.coin.derive_sprite());

        let text_size = measure_text(&game.money.to_string(), None, 16, 1.0);
        
        cursor.y += assets.coin.texture.size().y*2.0 - 8.0;
        cursor.x -= text_size.width*2.0;
        draw_text(&game.money.to_string(), cursor.x, cursor.y, 32.0, WHITE);
    }
    
    // INFO: Don't forget some textures are scaled 4x
    'show_statue: {
        if !game.ui_show_statue { break 'show_statue; }

        let prev_mouse_div = game.ui_state.mouse_div;
        game.ui_state.mouse_div /= 4.0;
        
        let bg_size = assets.ui_bg.texture.size()*2.0;
        let bg_offset = vec2(UI_WIDTH_F32, UI_HEIGHT_F32) - bg_size;

        let bg_padding = vec2(20.0, 30.0);
        let button_spacing = vec2(8.0, 4.0);

        let mut cursor = vec2(bg_offset.x/2.0, bg_offset.y/2.0);
        draw_ui(cursor, vec2(2.0, 2.0), &assets.ui_bg.derive_sprite());

        cursor += bg_padding;
    
        let button_width = (bg_size.x - bg_padding.x*2.0)/2.0 - button_spacing.x;
        let button_height = assets.ui_button[0][0].frames[0].h;
        let mut lcursor = cursor;
        let mut rcursor = cursor;

        rcursor.x += button_spacing.x + button_width;

        let coin_size = assets.coin.texture.size();

        let mut ui_seq_upgrade_button = |position: Vec2, mut state: SeqUpgrade| {
            let mut disabled = state.reached_count;
            let mut pressing = false;
            let unlocked = state.unlocked;

            let can_afford = game.money >= state.cost;

            let mut name = state.name;

            if !unlocked {
                name = "???";
                disabled = true;
            }
            
            if ui_button(
                &mut game.ui_state,
                name,
                position,
                button_width,
                disabled,
                Some(&mut pressing),
                &assets.ui_button,
            ) && can_afford {
                (state.upgrade)();
                game.money -= state.cost;
            }

            if !disabled {
                let mut coin_pos = position + vec2(button_width, 0.0) - vec2(coin_size.x, 0.0);
                // INFO: Magix value because button texture is not centerable without magic value
                coin_pos += vec2(-4.0, 1.0);
                
                if pressing {
                    coin_pos.y += 1.0;
                }
                
                draw_ui(coin_pos, vec2(1.0, 1.0), &assets.coin.derive_sprite());
                
                let text_size = measure_text(&state.cost.to_string(), None, 16, 1.0);
                coin_pos += vec2(-text_size.width-2.0, button_height/2.0);

                let color = if can_afford { WHITE } else { RED };
                draw_text(&state.cost.to_string(), coin_pos.x, coin_pos.y, 16.0, color);
            }
        };

        ui_seq_upgrade_button(lcursor, upgrades.mining.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.ladder.to_seq());
        
        lcursor.y += button_spacing.y + button_height;
        rcursor.y += button_spacing.y + button_height;
        ui_seq_upgrade_button(lcursor, upgrades.bag.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.climb_momentum.to_seq());
        
        lcursor.y += button_spacing.y + button_height;
        rcursor.y += button_spacing.y + button_height;
        ui_seq_upgrade_button(lcursor, upgrades.dwarfcopter.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.dwarfcopter_fuel.to_seq());
        
        lcursor.y += button_spacing.y + button_height;
        rcursor.y += button_spacing.y + button_height;
        ui_seq_upgrade_button(lcursor, upgrades.dwarfcopter_boost.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.dwarfcopter_storage.to_seq());

        game.ui_state.mouse_div = prev_mouse_div;
    }
    
    // HACK: This shouldn't be inside render, but whatever
    if is_mouse_button_released(MouseButton::Left) {
        game.ui_state.last_clicked_button_hash = None;
    }
    
    {    
        set_camera(&Camera2D::from_display_rect(camera_origin));
    }

    // debug
    apply_debug_commands();
}
