use crate::prelude::*;

pub fn update(game: &mut Game) {
    game.derived = DerivedState::default();
    game.action.reset();

    let dt = get_frame_time();
    let assets = &game.assets;

    let player = &mut game.player;
    let minecart = &mut game.minecart;
    let crusher = &mut game.crusher;
    
    let world = &mut game.world;
    let visible_chunks = &mut game.visible_chunks;
    let derived = &mut game.derived;
    let action = &mut game.action;

    let tiles = world.tiles();

    let player_last_pos = player.trans.pos;

    derived.player_anim_finished = player.anim.repeated > 0;

    // actions :::
    let mut player_movement = IVec2::ZERO;

    // player movement :::
    'player_movement: {
        let pos = player.trans.pos;
        let mut new_pos = player.trans.pos;
        let size = player.trans.size;
        let hsize = size / 2.0;
        let tile = tiles.at_world_pos(pos);

        if is_key_pressed(KeyCode::Tab) {
            game.dev_mode = !game.dev_mode;
        }

        if is_key_down(KeyCode::W) {
            player_movement.y += 1;
        }
        if is_key_down(KeyCode::S) {
            player_movement.y -= 1;
        }
        if is_key_down(KeyCode::D) {
            player_movement.x += 1;
        }
        if is_key_down(KeyCode::A) {
            player_movement.x -= 1;
        }

        let mut movement_dir = player_movement.as_vec2() * dt * 50.0;

        if game.dev_mode {
            player.trans.pos += movement_dir * 15.0;
            break 'player_movement;
        }

        let tile_one_down = tile.down(1);
        
        // gravity
        if !tile.kind.can_climb() {
            if !tile_one_down.kind.can_climb() || pos.y - tile.world_pos().y > 1.0 {
                movement_dir.y = -9.8 * TILE_SIDE as f32 * dt;
            }
        }

        new_pos += movement_dir;

        let right_intersection = World::query_intersected_tiles_y(&game.bump,
            new_pos.x+hsize.x, [pos.y, pos.y+size.y]
        );
        let left_intersection = World::query_intersected_tiles_y(&game.bump,
            new_pos.x-hsize.x+1.0, [pos.y, pos.y+size.y]
        );

        let top_intersection = World::query_intersected_tiles_x(&game.bump,
            [pos.x-hsize.x+1.0, pos.x+hsize.x], new_pos.y+size.y
        );
        let bottom_intersection = World::query_intersected_tiles_x(&game.bump,
            [pos.x-hsize.x+1.0, pos.x+hsize.x], new_pos.y
        );

        for tile_pos in bottom_intersection {
            if tiles.at_tile_pos(tile_pos).kind.can_walk_through() { continue; };
            new_pos.y = player.trans.pos.y;
            derived.player_touching_bottom = true;
        }
        for tile_pos in left_intersection {
            if tiles.at_tile_pos(tile_pos).kind.can_walk_through() { continue; };
            new_pos.x = player.trans.pos.x;
            derived.player_touching_left = true;
        }
        for tile_pos in right_intersection {
            if tiles.at_tile_pos(tile_pos).kind.can_walk_through() { continue; };
            new_pos.x = player.trans.pos.x;
            derived.player_touching_right = true;
        }
        for tile_pos in top_intersection {
            if tiles.at_tile_pos(tile_pos).kind.can_walk_through() { continue; };
            new_pos.y = player.trans.pos.y;
            derived.player_touching_top = true;
        }

        player.trans.pos = new_pos;

        if is_key_pressed(KeyCode::Space) {
            player.sprite.flip_x = !player.sprite.flip_x;
        }
        
        if player_movement.x > 0 {
            player.sprite.flip_x = false;
        } else if player_movement.x < 0 {
            player.sprite.flip_x = true;
        }

        if player_movement != IVec2::ZERO {
            derived.player_moving = true;
        }
    }

    // player animations :::
    if player.anim.is( &assets.player_idle ) && derived.player_moving {
        player.anim = assets.player_walk.derive_anim();
    }
    if player.anim.is( &assets.player_walk ) && !derived.player_moving {
        player.anim = assets.player_idle.derive_anim();
    }
    if player.anim.is( &assets.player_hit) && derived.player_anim_finished {
        player.anim = assets.player_idle.derive_anim();
    }
    
    let player_tile = tiles.at_world_pos(player.trans.pos);
    let mut world_commands = world.commands(&game.bump);

    let mut player_added_to_bags = Vec::with_capacity_in(4, &game.bump);

    'block_mine: {
        let touch_vec = if derived.player_touching_left && player_movement.x < 0 {
            ivec2(-1, 0)
        } else if derived.player_touching_right && player_movement.x > 0 {
            ivec2(1, 0)
        } else if derived.player_touching_bottom && player_movement.y < 0 {
            ivec2(0, -1)
        } else {
            IVec2::ZERO
        };
        
        if touch_vec == IVec2::ZERO { break 'block_mine; }

        let tile = player_tile.offset_by(touch_vec);
        let tile_one_up = tile.up(1);
        let tile_one_down = tile.down(1);

        if !tile.kind.can_mine() { break 'block_mine; }

        if touch_vec.x != 0 && tile_one_up.kind.can_climb() { break 'block_mine; }            

        let movement_str = 1.0 / f32::max(player_movement.x.abs() as f32 + player_movement.y.abs() as f32, 1.0);
        
        let durability = game.tile_durability_map.entry(tile.pos).or_insert(0.0);
        *durability += dt * 3.0 * movement_str;
        
        player.anim = assets.player_hit.derive_anim();

        if *durability > 0.5 {
            if tile.kind.item_drop() != ItemKind::Air {
                if player.carrying.length >= game.player_max_carrying { break 'block_mine; }
                player_added_to_bags.push(tile.kind.item_drop());
            }
            if touch_vec.y != 0 {
                if tile_one_down.kind.is_air() {
                    world_commands.set_tile(tile.pos, Tile::BackgroundStoneLadder);
                } else {
                    world_commands.set_tile(tile.pos, Tile::BackgroundStoneLadderSupport);
                }
                if tile_one_up.kind == Tile::BackgroundStoneLadderSupport {
                    world_commands.set_tile(tile_one_up.pos, Tile::BackgroundStoneLadder);
                }
            } else {
                world_commands.set_tile(tile.pos, tile.kind.mine_results_tile());
            }

            *durability = 0.0;
        }
    }

    world.apply_commands(world_commands);
    let mut world_commands = world.commands(&game.bump);
    let tiles = world.tiles();
    let player_tile = tiles.at_world_pos(player.trans.pos);

    'lay_ladder: {
        let tile = player_tile;
        let tile_one_up = tile.up(1);
        let tile_one_down = tile.down(1);
        let tile_two_down = tile.down(2);

        if tile.pos.y >= WORLD_SPAWN_I32.y { break 'lay_ladder; }

        if  player_movement.y > 0 && tile.kind.is_air() && tile_one_up.kind.is_air() &&
            player.trans.pos.y - tile.world_pos().y < 1.0
        {
            if !tile_one_down.kind.is_air() {
                world_commands.set_tile(tile.pos, Tile::BackgroundStoneLadderSupport);
            } else if tile_one_down.kind.can_climb() {
                world_commands.set_tile(tile.pos, Tile::BackgroundStoneLadder);
            }
        }

        if  player_movement.y < 0 && tile.kind.can_climb() &&
            tile_one_down.kind.is_air() && !tile_one_down.kind.can_climb() &&
            player.trans.pos.y - tile.world_pos().y < 1.0
        {
            if !tile_two_down.kind.is_air() {
                world_commands.set_tile(tile_one_down.pos, Tile::BackgroundStoneLadderSupport);
            } else {
                world_commands.set_tile(tile_one_down.pos, Tile::BackgroundStoneLadder);
            }
        }
        
    }

    // bag handling
    let directional_offset =
        if player.sprite.flip_x { player.trans.offset * vec2(-1.0, 1.0) }
        else { player.trans.offset };

    if (player.last_positions[0]).distance(player.trans.pos + directional_offset) > 2.0 {
        player.last_positions.rotate_right(1);
        player.last_positions[0] = player.trans.pos + directional_offset;
    }

    for item_kind in player_added_to_bags {
        if player.amount_carrying >= game.player_max_carrying { break; }
        
        player.carrying[player.amount_carrying] = item_kind;
        player.amount_carrying += 1;
    }

    debug(player.trans.collider(), WHITE);
    debug(minecart.trans.collider(), RED);
    
    if  minecart.movement == MinecartMovement::Idle &&
        player.trans.collider().intersects(minecart.trans.collider()) &&
        player.amount_carrying > 0
    {        
        if  minecart.cooldown <= 1.8 &&
            player.amount_carrying > 0 &&
            minecart.amount_carrying < minecart.carrying.len()
        {
            minecart.cooldown = 2.0;
            player.amount_carrying -= 1;
            let kind = player.carrying[player.amount_carrying];
            let trans = Transform {
                pos: minecart.trans.pos,
                size: vec2(0.0, 0.0),
                offset: vec2(0.0, 0.0),
            };
            minecart.carrying[minecart.amount_carrying] = Item { trans, kind };
            minecart.amount_carrying += 1;
        }
    }

    if minecart.movement == MinecartMovement::Idle && minecart.cooldown <= 0.1 && minecart.amount_carrying > 0 {
        minecart.movement = MinecartMovement::Forwards;
    }

    minecart.cooldown = f32::max(minecart.cooldown-dt, 0.0);

    if minecart.movement == MinecartMovement::Forwards {
        minecart.trans.pos.x += 100.0 * dt;
        if minecart.trans.pos.x >= MINECART_END.x as f32 * TILE_SIDE as f32 {
            minecart.movement = MinecartMovement::Backwards;
            minecart.amount_carrying = 0;
        }
    }

    if minecart.movement == MinecartMovement::Backwards {
        minecart.trans.pos.x -= 100.0 * dt;
        if minecart.trans.pos.x <= MINECART_START.x as f32 * TILE_SIDE as f32 {
            minecart.movement = MinecartMovement::Idle;
        }
    }


    // tick animations :::
    tick_animation(&mut crusher.sprite, &mut crusher.anim, dt);
    tick_animation(&mut player.sprite, &mut player.anim, dt);
    
    // update visible chunks :::
    {
        let player_chunk = world_pos_to_chunk_pos(player.trans.pos);
        *visible_chunks = World::query_chunks_around_chunk_pos(&game.bump, player_chunk, 1).to_vec();
    }
    
    // update tile durability :::
    {
        let mut to_remove = Vec::with_capacity_in(32, &game.bump);

        for (tile_pos, durability) in &mut game.tile_durability_map {
            *durability -= dt;
            if *durability < 0.0 {
                to_remove.push(*tile_pos);
            }
        }

        for tile_pos in to_remove {
            game.tile_durability_map.remove(&tile_pos);
        }
        
    }

    // apply commands & updates :::
    world.apply_commands(world_commands);
    world.apply_updates(&assets.tile_set);
    
    game.bump.reset();
}
