use crate::prelude::*;

use crate::apply_debug_commands;
use crate::sprite::{ draw_sprite, draw_sprite_scaled, draw_sprite_offset, draw_ui };


pub fn render(game: &Game) {
    let statue = &game.statue;
    let minecart = &game.minecart;
    let player = &game.player;
    let crusher = &game.crusher;
    
    let derived = &game.derived;
    let assets = &game.assets;
    let world = &game.world;
    let visible_chunks = &game.visible_chunks;

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
    
    // draw_sprite_offset(crusher.trans.pos, crusher.trans.offset, &crusher.sprite);

    let rail_start_sprite = assets.rail_start.derive_sprite();
    let rail_sprite = assets.rail.derive_sprite();
    
    draw_sprite(tile_pos_to_world_pos(MINECART_START), &rail_start_sprite);

    for x in MINECART_START.x+1..=MINECART_END.x {
        let tile_pos = ivec2(x, MINECART_START.y);
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
        
        let mut camera = Camera2D::from_display_rect(ui_camera_origin);
        camera.zoom.y *= -1.0;
        set_camera(&camera);
    }

    let corner_padding = vec2(2.0, 2.0);
    let padding = vec2(1.0, 1.0);
    let mut cursor = vec2(GAME_WIDTH_F32, 0.0);

    {
        cursor.x -= assets.coin.texture.size().x + corner_padding.x;
        cursor.y += corner_padding.y;
        draw_ui(cursor, &assets.coin.derive_sprite());

        let text_size = measure_text(&game.money.to_string(), None, 16, 1.0);
        
        cursor.y += assets.coin.texture.size().y - 4.0;
        cursor.x -= text_size.width;
        draw_text(&game.money.to_string(), cursor.x, cursor.y, 16.0, WHITE);
    }

    
    {
        let bg_offset = vec2(GAME_WIDTH_F32, GAME_HEIGHT_F32) - assets.ui_bg.texture.size();
        if game.ui_show_statue {
            draw_sprite(vec2(bg_offset.x/2.0, bg_offset.y/2.0), &assets.ui_bg.derive_sprite());
        }
    }


    // debug
    apply_debug_commands();
}
