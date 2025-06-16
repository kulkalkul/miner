use macroquad::audio;

use crate::prelude::*;

use crate::apply_debug_commands;
use crate::sprite::{
    draw_sprite, draw_sprite_scaled, draw_sprite_rotated, draw_sprite_offset,
    draw_ui_partial, draw_ui_rotated, draw_ui,
};
use crate::ui::*;


pub fn render(game: &mut Game) {
    let statue = &game.statue;
    let minecart = &game.minecart;
    let ui_inventory_bar_frame = &game.ui_inventory_bar_frame;
    let ui_fuel_bar_frame = &game.ui_fuel_bar_frame;
    let player = &game.player;
    let elevator_cage = &game.elevator_cage;
    let elevator_platform = &game.elevator_platform;
    let demolisher = &game.demolisher;
    
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

        let mut camera = Camera2D::from_display_rect(mesh_camera_origin);
        set_camera(&camera);
        
        let mesh = world.mesh_at(chunk_pos);
        draw_mesh(&mesh.0);
    }
    
    let mut camera_origin = world_origin;
    camera_origin.x += player.trans.pos.x;
    camera_origin.y += player.trans.pos.y;
    
    {
        let mut camera = Camera2D::from_display_rect(camera_origin);
        set_camera(&camera);
    }

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
    let rail_diagonal_sprite = assets.rail_diagonal.derive_sprite();
    
    draw_sprite(tile_pos_to_world_pos(RAIL_START), &rail_start_sprite);

    // draw rail :::
    for x in RAIL_START.x+1..=RAIL_STRAIGHT_END.x {
        let tile_pos = ivec2(x, RAIL_START.y);
        let world_pos = tile_pos_to_world_pos(tile_pos);
        draw_sprite(world_pos, &rail_sprite);
    }
    
    for (i, x) in (RAIL_STRAIGHT_END.x+1..=RAIL_DIAGONAL_END.x).enumerate() {
        let tile_pos = ivec2(x, RAIL_START.y);
        let world_pos = tile_pos_to_world_pos(tile_pos);
        draw_sprite(world_pos+vec2(i as f32 * -2.0, i as f32 * 7.0), &rail_diagonal_sprite);
    }

    draw_sprite_rotated(minecart.trans.pos, minecart.rotation, &minecart.sprite);
    draw_sprite(statue.trans.pos, &statue.sprite);
    
    draw_sprite_scaled(
        // INFO: *0.5 and /2.0 because as two separate operations because 0.5 is for scaling, 2.0 is for offsetting
        // by half. This makes the intent clear.
        statue.trans.pos+vec2(statue.trans.size.x-assets.sign_sell.derive_sprite().texture_frame.w*0.5/2.0, 0.0)-vec2(0.5, 0.0),
        vec2(0.5, 0.5),
        &assets.sign_sell.derive_sprite(),
    );
    
    if game.elevator_spawned {
        draw_sprite_scaled(
            statue.trans.pos-vec2(assets.sign_descend.derive_sprite().texture_frame.w*0.5/2.0, 0.0)-vec2(0.5, 0.0),
            vec2(0.5, 0.5),
            &assets.sign_descend.derive_sprite(),
        );
    }

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

    if game.demolisher_spawned {
        draw_sprite(demolisher.trans.pos, &demolisher.sprite);
    }

    // draw dropped items
    for item in &game.dropped_items {
        draw_sprite_scaled(item.trans.pos, vec2(1.0, 1.0), &assets.items[item.kind as usize].derive_sprite());
    }
    
    
    // draw player :::
    if !game.demolisher_started {
        draw_sprite_offset(player.trans.pos, player.trans.offset, &player.sprite);
    }
    
    if game.elevator_spawned {
        draw_sprite(elevator_cage.trans.pos, &elevator_cage.sprite);
    }
        
    // ui    
    {    
        set_camera(&Camera2D::from_display_rect(camera_origin));
    }

    // overlay ui
    if derived.ui_show_statue_key {
        let pos = statue.trans.pos + statue.trans.size/2.0 - assets.ui_keys.texture.size()/2.0 * vec2(0.5, 0.5);
        draw_sprite_scaled(pos, vec2(0.5, 0.5), &assets.ui_keys.derive_sprite());
    }
    if game.demolisher_spawned && derived.ui_show_demolisher_key {
        let mut pos = demolisher.trans.pos
            + vec2(demolisher.trans.size.x/2.0, 0.0)
            - vec2(assets.ui_keys.texture.size().x/2.0, 0.0) * vec2(0.5, 0.5);
        pos.y += assets.ui_keys.texture.size().y/2.0;
        pos.y += demolisher.trans.size.y;
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

    if game.main_ui_state == MainUIState::MainMenu {
        let prev_mouse_div = game.ui_state.mouse_div;
        game.ui_state.mouse_div /= 4.0;

        let mut pos = vec2(UI_WIDTH_F32/2.0, UI_HEIGHT_F32/4.0)-vec2(240.0/2.0, 0.0);

        if ui_button(&mut game.ui_state, "Start Game", pos, 240.0, false, None, &assets.ui_button) {
            game.main_ui_state = MainUIState::InGame;
            audio::play_sound(&assets.sfx_ui_positive, audio::PlaySoundParams { looped: false, volume: 0.1 });
        }
        
        pos += vec2(0.0, 32.0);

        if ui_button(&mut game.ui_state, "Show Credits", pos, 240.0, false, None, &assets.ui_button) {
            game.main_ui_state = MainUIState::MainMenuCredits;
            audio::play_sound(&assets.sfx_ui_positive, audio::PlaySoundParams { looped: false, volume: 0.1 });
        }

        game.ui_state.mouse_div = prev_mouse_div;
    }
    
    if game.main_ui_state == MainUIState::MainMenuCredits {
        let prev_mouse_div = game.ui_state.mouse_div;
        game.ui_state.mouse_div /= 4.0;

        draw_rectangle(0.0, 0.0, UI_WIDTH_F32, UI_HEIGHT_F32, Color::from_rgba(0, 0, 0, 125));

        let mut pos = vec2(UI_WIDTH_F32/2.0, UI_HEIGHT_F32/4.0)-vec2(300.0/2.0, 0.0);

        draw_text("A Game by kulkalkul.", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 32.0);
        draw_text("Programming, sprites, music by kulkalkul.", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 32.0);
        
        draw_text("Special thanks to;", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 24.0);
        draw_text("Artem Arbatsky for helping with sound", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 16.0);
        
        draw_text("design and providing sound assets.", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 24.0);
        
        draw_text("FilmCow for CC0 sound assets.", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 24.0);
        
        draw_text("dustyroomgames for CC0 sound assets.", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 24.0);
        draw_text("Built with macroquad & bumpalo using Rust.", pos.x, pos.y, 16.0, WHITE);
        pos += vec2(0.0, 24.0);

        if ui_button(&mut game.ui_state, "Back", pos, 300.0, false, None, &assets.ui_button) {
            game.main_ui_state = MainUIState::MainMenu;
            audio::play_sound(&assets.sfx_ui_positive, audio::PlaySoundParams { looped: false, volume: 0.1 });
        }

        game.ui_state.mouse_div = prev_mouse_div;
    }

    if game.elevator_spawned && derived.player_can_use_jetpack {
        let elevator_pos = elevator_platform.trans.pos + elevator_platform.trans.size/2.0;
        if elevator_pos.distance(player.trans.pos) >= 128.0 {
            let dir = (elevator_pos-player.trans.pos).normalize();            

            let sprite;
            if dir.x <= 0.0 {
                sprite = assets.ui_elevator_arrow[1].derive_sprite();
            } else {
                sprite = assets.ui_elevator_arrow[0].derive_sprite();
            }
            
            let mut pos = vec2(UI_WIDTH_F32/2.0, UI_HEIGHT_F32/2.0);
            
            pos += dir*vec2(UI_WIDTH_F32/2.0, UI_HEIGHT_F32/2.0)*0.75*vec2(1.0, -1.0);
            // INFO: This is no-op, but it shows the intent, we need half of it, but scale is double.
            pos.x -= sprite.texture_frame.w/2.0*2.0;
        
            draw_ui_rotated(pos, vec2(2.0, 2.0), -dir.to_angle(), &sprite);
        }
    }

    if game.demolisher_spawned && derived.player_at_overworld {
        let demolisher_pos = demolisher.trans.pos;
        if demolisher_pos.distance(player.trans.pos) >= 128.0 {
            let pos = vec2(UI_WIDTH_F32/2.0-128.0, 32.0);
            draw_ui(pos, vec2(2.0, 2.0), &assets.ui_demolisher_arrow.derive_sprite());
        }
    }
    
    let corner_padding = vec2(2.0, 2.0);
    {
        let mut cursor = vec2(UI_WIDTH_F32, 0.0);
    
        cursor.x -= (assets.coin.texture.size()).x*2.0 + corner_padding.x;
        cursor.y += corner_padding.y;
        draw_ui(cursor, vec2(2.0, 2.0), &assets.coin.derive_sprite());

        let text_size = measure_text(&game.money.to_string(), None, 16, 1.0);
        
        cursor.y += assets.coin.texture.size().y*2.0 - 8.0;
        cursor.x -= text_size.width*2.0;
        draw_text(&game.money.to_string(), cursor.x, cursor.y, 32.0, WHITE);

        cursor.y = UI_HEIGHT_F32 - 16.0;
        cursor.x = UI_WIDTH_F32 - assets.items[0].derive_sprite().texture_frame.w*4.0 - corner_padding.x;

        for variant in ItemKind::VARIANTS {
            let unlocked = game.unlocked_ores[variant as usize];
            
            if !unlocked { continue; }
            if variant  == ItemKind::Air { continue; }

            let sprite = &assets.items[variant as usize].derive_sprite();
            let value = variant.value();
            let size = sprite.texture_frame.size();
            draw_ui(cursor - vec2(0.0, size.y)*2.5, vec2(4.0, 4.0), &sprite);
            
            let text_size = measure_text(&value.to_string(), None, 16, 1.0);
            draw_text(value.to_string(), cursor.x - text_size.width*2.0, cursor.y, 32.0, WHITE);
            cursor.y -= 32.0;
        }
    }

    {
        let mut cursor = vec2(4.0, 4.0);
        
        if derived.player_has_jetpack {
            let ratio = player.jetpack_fuel as f32 / derived.player_jetpack_fuel_capacity as f32;
            draw_ui(cursor, vec2(2.0, 2.0), &assets.ui_fuel_bar_background.derive_sprite());
            draw_ui_partial(cursor, vec2(2.0, 2.0), vec2(1.0, ratio), &assets.ui_fuel_bar_fill.derive_sprite());
            draw_ui(cursor, vec2(2.0, 2.0), &ui_fuel_bar_frame.sprite);
            cursor += vec2(ui_fuel_bar_frame.sprite.texture_frame.w*2.0, 0.0);
            cursor += vec2(8.0, 0.0);
        }
        let ratio = player.carrying.length as f32 / derived.player_bag_carry_capacity as f32;
        draw_ui(cursor, vec2(2.0, 2.0), &assets.ui_inventory_bar_background.derive_sprite());
        draw_ui_partial(cursor, vec2(2.0, 2.0), vec2(1.0, ratio), &assets.ui_inventory_bar_fill.derive_sprite());
        draw_ui(cursor, vec2(2.0, 2.0), &ui_inventory_bar_frame.sprite);        
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
    
        let bg_inner_width = bg_size.x - bg_padding.x*2.0;
        let button_width = bg_inner_width/2.0 - button_spacing.x;
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
            ) {
                if can_afford {
                    (state.upgrade)();
                    game.money -= state.cost;
                    audio::play_sound(&assets.sfx_ui_positive, audio::PlaySoundParams { looped: false, volume: 0.1 });
                } else {
                    audio::play_sound(&assets.sfx_ui_negative, audio::PlaySoundParams { looped: false, volume: 0.1 });
                }
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
        
        lcursor.y += 4.0;
        rcursor.y += 4.0;
        
        draw_text("Phase I", lcursor.x, lcursor.y, 16.0, Color::from_hex(0xc7cfcc));
        
        lcursor.y += 2.0;
        rcursor.y += 2.0;

        draw_rectangle(lcursor.x, lcursor.y, bg_inner_width, 1.0, Color::from_hex(0xc7cfcc));
        
        lcursor.y += 6.0;
        rcursor.y += 6.0;

        ui_seq_upgrade_button(lcursor, upgrades.mining.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.ladder.to_seq());
        
        lcursor.y += button_spacing.y + button_height;
        rcursor.y += button_spacing.y + button_height;
        ui_seq_upgrade_button(lcursor, upgrades.bag.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.climb_momentum.to_seq());
        
        lcursor.y += button_spacing.y + button_height;
        rcursor.y += button_spacing.y + button_height;
        
        lcursor.y += 8.0;
        rcursor.y += 8.0;
        
        draw_text("Phase II", lcursor.x, lcursor.y, 16.0, Color::from_hex(0xc7cfcc));
        
        lcursor.y += 2.0;
        rcursor.y += 2.0;

        draw_rectangle(lcursor.x, lcursor.y, bg_inner_width, 1.0, Color::from_hex(0xc7cfcc));
        
        lcursor.y += 6.0;
        rcursor.y += 6.0;
        
        ui_seq_upgrade_button(lcursor, upgrades.jetpack.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.jetpack_fuel.to_seq());
        
        lcursor.y += button_spacing.y + button_height;
        rcursor.y += button_spacing.y + button_height;
        ui_seq_upgrade_button(lcursor, upgrades.jetpack_boost.to_seq());
        ui_seq_upgrade_button(rcursor, upgrades.jetpack_storage.to_seq());
        
        lcursor.y += button_spacing.y + button_height;
        rcursor.y += button_spacing.y + button_height;
        
        lcursor.y += 8.0;
        rcursor.y += 8.0;
        
        draw_text("Phase III", lcursor.x, lcursor.y, 16.0, Color::from_hex(0xc7cfcc));
        
        lcursor.y += 2.0;
        rcursor.y += 2.0;

        draw_rectangle(lcursor.x, lcursor.y, bg_inner_width, 1.0, Color::from_hex(0xc7cfcc));
        
        lcursor.y += 6.0;
        rcursor.y += 6.0;
        ui_seq_upgrade_button(lcursor, upgrades.demolisher.to_seq());

        game.ui_state.mouse_div = prev_mouse_div;
    }
    
    if game.demolisher_started && demolisher.stage == 5 {
        let transparency = f32::min(demolisher.stage_tick / 5.0 * 255.0, 255.0);
        draw_rectangle(0.0, 0.0, UI_WIDTH_F32, UI_HEIGHT_F32, Color::from_rgba(0, 0, 0, transparency as u8));
    }
    if game.demolisher_started && demolisher.stage == 6 {
        draw_rectangle(0.0, 0.0, UI_WIDTH_F32, UI_HEIGHT_F32, Color::from_rgba(0, 0, 0, 255));
        
        let mut pos = vec2(UI_WIDTH_F32/2.0, UI_HEIGHT_F32/3.0);

        let mut draw_center = |text: &str, size: u16, offset: f32| {
            let measurement = measure_text(text, None, size, 1.0);
            pos.x -= measurement.width/2.0;
            draw_text(text, pos.x, pos.y, size as f32, WHITE);
            pos += vec2(measurement.width/2.0, offset);
        };

        draw_center("The End", 32, 32.0);
        
        draw_center("A Game by kulkalkul.", 16, 32.0);
        draw_center("Programming, sprites, music by kulkalkul.", 16, 32.0);
        draw_center("Special thanks to;", 16, 24.0);
        draw_center("Artem Arbatsky for helping with sound", 16, 16.0);
        draw_center("design and providing sound assets.", 16, 24.0);
        draw_center("FilmCow for CC0 sound assets.", 16, 24.0);
        draw_center("dustyroomgames for CC0 sound assets.", 16, 24.0);
        draw_center("Built with macroquad & bumpalo using Rust.", 16, 24.0);
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
