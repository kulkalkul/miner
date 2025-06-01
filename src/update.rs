use crate::prelude::*;

pub fn update(game: &mut Game) {
    game.derived = DerivedState::default();
    game.action.reset();

    game.input_actions = InputActions {
        move_left: is_key_down(KeyCode::A),
        move_right: is_key_down(KeyCode::D),
        move_up: is_key_down(KeyCode::W),
        move_down: is_key_down(KeyCode::S),
        interact: is_key_pressed(KeyCode::E),
        escape: is_key_pressed(KeyCode::Escape),
        toggle_dev_mode: is_key_pressed(KeyCode::Tab),
    };

    let dt = get_frame_time();
    let assets = &game.assets;

    let player = &mut game.player;
    let minecart = &mut game.minecart;
    let crusher = &mut game.crusher;
    let statue = &mut game.statue;
    
    let world = &mut game.world;
    let visible_chunks = &mut game.visible_chunks;
    let derived = &mut game.derived;
    let action = &mut game.action;
    let input_actions = &game.input_actions;
    
    let late_derived = &game.late_derived;

    let tiles = world.tiles();

    let player_last_pos = player.trans.pos;
    
    game.total_time += dt;
    

    {
        let epsilon = std::f32::consts::TAU / 16.0;
        for (i, sine) in derived.time_sine_1.iter_mut().enumerate() {
            *sine = f32::sin((game.total_time * 1.0 + epsilon * i as f32) % std::f32::consts::TAU);
        }
        for (i, sine) in derived.time_sine_2.iter_mut().enumerate() {
            *sine = f32::sin((game.total_time * 2.0 + epsilon * i as f32) % std::f32::consts::TAU);
        }
        for (i, sine) in derived.time_sine_3.iter_mut().enumerate() {
            *sine = f32::sin((game.total_time * 3.0 + epsilon * i as f32) % std::f32::consts::TAU);
        }
        for (i, sine) in derived.time_sine_4.iter_mut().enumerate() {
            *sine = f32::sin((game.total_time * 4.0 + epsilon * i as f32) % std::f32::consts::TAU);
        }
    }
    
    derived.player_anim_finished = player.anim.repeated > 0;

    // actions :::
    let mut player_movement = IVec2::ZERO;

    // player movement :::
    'player_movement: {
        if late_derived.ui_is_active { break 'player_movement; }

        let pos = player.trans.pos;
        let mut new_pos = player.trans.pos;
        let size = player.trans.size;
        let hsize = size / 2.0;
        let tile = tiles.at_world_pos(pos);

        if input_actions.toggle_dev_mode {
            game.dev_mode = !game.dev_mode;
        }

        if input_actions.move_up {
            player_movement.y += 1;
        }
        if input_actions.move_down {
            player_movement.y -= 1;
        }
        if input_actions.move_right {
            player_movement.x += 1;
        }
        if input_actions.move_left {
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
        
        if player_movement.x > 0 {
            player.sprite.flip_x = false;
        } else if player_movement.x < 0 {
            player.sprite.flip_x = true;
        }

        if player_movement != IVec2::ZERO {
            derived.player_moving = true;
        }
    }
    
    derived.player_hit_str = 1.0 / f32::max(player_movement.x.abs() as f32 + player_movement.y.abs() as f32, 1.0);

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
        
        let durability = game.tile_durability_map.entry(tile.pos).or_insert(0.0);
        *durability += dt * 3.0 * derived.player_hit_str;
        
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
        if player.carrying.length >= game.player_max_carrying { break; }
        player.carrying.push(item_kind);        
    }

    // debug_generic(player.trans.collider(), WHITE);
    // debug_generic(minecart.trans.collider(), RED);
    
    if  minecart.movement == MinecartMovement::Idle &&
        player.trans.collider().intersects(minecart.trans.collider()) &&
        player.carrying.length > 0
    {        
        if  minecart.cooldown <= 1.8 &&
            minecart.carrying.length < minecart.carrying.cap()
        {
            minecart.cooldown = 2.0;
            let kind = player.carrying.pop().unwrap();
            
            let trans = Transform {
                pos: minecart.trans.pos,
                size: vec2(0.0, 0.0),
                offset: vec2(0.0, 0.0),
            };
            minecart.carrying.push(Item { trans, kind });
        }
    }

    if player.trans.collider().intersects(statue.trans.collider()) {
        derived.ui_show_statue_key = true;

        if input_actions.interact {
            game.ui_show_statue = !game.ui_show_statue;
        }
    }

    let mut coins_to_remove = Vec::new_in(&game.bump);

    for (i, coin) in &mut game.coins.iter_mut().enumerate().rev() {
        if player.trans.pos.distance_squared(coin.trans.pos) <= 16.0 {
            game.money += coin.amount;
            coins_to_remove.push(i);
        }
        if coin.trans.pos.y <= WORLD_SPAWN_F32.y * TILE_SIDE as f32 {
            coin.trans.pos.y = WORLD_SPAWN_F32.y * TILE_SIDE as f32;
            coin.velocity = vec2(0.0, 0.0);
        } else {
            coin.trans.pos += coin.velocity * dt;
            coin.velocity.y -= 80.0 * dt;
        }
    }
    for i in coins_to_remove {
        // INFO: not using swap_remove because draw order changes and it looks glitchy
        game.coins.remove(i);
    }

    if minecart.movement == MinecartMovement::Idle && minecart.cooldown <= 0.1 && minecart.carrying.length > 0 {
        minecart.movement = MinecartMovement::Forwards;
        minecart.anim = assets.minecart_moving.derive_anim();
    }

    minecart.cooldown = f32::max(minecart.cooldown-dt, 0.0);

    if minecart.movement == MinecartMovement::Forwards {
        minecart.trans.pos.x += 100.0 * dt;
        if minecart.trans.pos.x >= MINECART_END.x as f32 * TILE_SIDE as f32 {
            minecart.movement = MinecartMovement::Backwards;
        }
    }

    if minecart.movement == MinecartMovement::Backwards {
        minecart.trans.pos.x -= 100.0 * dt;
        if minecart.trans.pos.x <= MINECART_START.x as f32 * TILE_SIDE as f32 {
            minecart.movement = MinecartMovement::Idle;
            minecart.anim = assets.minecart_idle.derive_anim();

            let mut sum = 0;
            for item in minecart.carrying.slice() {
                sum += item.kind.value();
            }
            minecart.carrying.clear();

            let mut ones = i32::min(sum/1, 30);
            sum -= ones*1;
            let mut fives = i32::min(sum/5, 20);
            sum -= fives*5;
            let mut fifteens = i32::min(sum/15, 20);
            sum -= fifteens*15;
            let mut twentyfives = i32::min(sum/25, 10);
            sum -= twentyfives*25;

            let hundreds = sum/100;
            sum -= hundreds*100;

            let remaining_twentyfives = sum/25;
            sum -= remaining_twentyfives*25;
            let remaining_fifteens = sum/15;
            sum -= remaining_fifteens*15;
            let remaining_fives = sum/5;
            sum -= remaining_fives*5;
            let remaining_ones = sum/1;
            sum -= remaining_ones*1;

            debug_assert!(sum == 0);

            twentyfives += remaining_twentyfives;
            fifteens += remaining_fifteens;
            fives += remaining_fives;
            ones += remaining_ones;

            let trans = Transform {
                pos: minecart.trans.pos + vec2(0.0, 8.0),
                size: vec2(0.0, 0.0),
                offset: vec2(0.0, 0.0),
            };

            let new_bundle = |amount: i32, asset: &SpriteAsset| {
                // INFO: magic values
                let x = rand::gen_range(-75.0, -100.0);
                let y = rand::gen_range(68.0, 90.0);

                let sine_index = rand::gen_range(0, 16);

                CoinBundle {
                    trans,
                    amount,
                    velocity: vec2(x, y),
                    sprite: asset.derive_sprite(),
                    sine_index,
                }
            };

            for _ in 0..ones {
                game.coins.push(new_bundle(1, &assets.coins[0]));
            }
            for _ in 0..fives {
                game.coins.push(new_bundle(5, &assets.coins[1]));
            }
            for _ in 0..fifteens {
                game.coins.push(new_bundle(15, &assets.coins[2]));
            }
            for _ in 0..twentyfives {
                game.coins.push(new_bundle(25, &assets.coins[3]));
            }
            for _ in 0..hundreds {
                game.coins.push(new_bundle(100, &assets.coins[4]));
            }
        }
    }

    // tick animations :::
    tick_animation(&mut minecart.sprite, &mut minecart.anim, dt);
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

    'escape: {
        if input_actions.escape {
            if game.ui_show_statue {
                game.ui_show_statue = false;
                break 'escape;
            }
        }
    }


    // apply commands & updates :::
    world.apply_commands(world_commands);
    world.apply_updates(&assets.tile_set);
    
    game.late_derived = LateDerivedState::default();
    let late_derived = &mut game.late_derived;

    late_derived.ui_is_active = game.ui_show_statue;
    
    game.bump.reset();
}
