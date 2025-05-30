use crate::prelude::*;

use crate::apply_debug_commands;
use crate::sprite::{ draw_sprite, draw_sprite_offset };

pub fn render(game: &Game) {
    let player = &game.player;
    let crusher = &game.crusher;
    let visible_chunks = &game.visible_chunks;
    let world = &game.world;

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

    let rail_start_sprite = game.assets.rail_start.derive_sprite();
    let rail_sprite = game.assets.rail.derive_sprite();
    
    draw_sprite(tile_pos_to_world_pos(MINECART_START), &rail_start_sprite);

    for x in MINECART_START.x+1..=MINECART_END.x {
        let tile_pos = ivec2(x, MINECART_START.y);
        let world_pos = tile_pos_to_world_pos(tile_pos);
        draw_sprite(world_pos, &rail_sprite);
    }

    draw_sprite(game.minecart.trans.pos, &game.minecart.sprite);
    

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

    // debug
    apply_debug_commands();
}
