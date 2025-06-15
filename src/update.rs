use macroquad::audio;

use crate::prelude::*;

pub fn update(game: &mut Game) {
    // pre update :::
    game.window_to_draw_size = vec2(screen_width(), screen_height()) / vec2(GAME_WIDTH_F32, GAME_HEIGHT_F32);
    game.ui_state.mouse_div = game.window_to_draw_size;

    // reset per-frame :::
    game.derived = DerivedState::default();
    game.action.reset();
    
    // input map :::
    game.input_actions = InputActions {
        move_left      : is_key_down(KeyCode::A) || is_key_down(KeyCode::Left),
        move_right     : is_key_down(KeyCode::D) || is_key_down(KeyCode::Right),
        move_up        : is_key_down(KeyCode::W) || is_key_down(KeyCode::Up),
        move_down      : is_key_down(KeyCode::S) || is_key_down(KeyCode::Down),

        interact       : is_key_pressed(KeyCode::E),
        escape         : is_key_pressed(KeyCode::Escape),
        toggle_dev_mode: is_key_pressed(KeyCode::Tab),
    };

    // frame borrows :::
    let dt = get_frame_time();
    let assets = &game.assets;

    let player = &mut game.player;
    let minecart = &mut game.minecart;
    let statue = &mut game.statue;
    let elevator_cage = &mut game.elevator_cage;
    let elevator_platform = &mut game.elevator_platform;
    let ui_inventory_bar_frame = &mut game.ui_inventory_bar_frame;
    let ui_fuel_bar_frame = &mut game.ui_fuel_bar_frame;
    let demolisher = &mut game.demolisher;
    
    let world = &mut game.world;
    let visible_chunks = &mut game.visible_chunks;
    let derived = &mut game.derived;
    let action = &mut game.action;
    let input_actions = &game.input_actions;
    let upgrades = &mut game.upgrades;
    
    // reset late derived:::
    let late_derived = &game.late_derived;
    let mut next_late_derived = LateDerivedState::default();

    let tiles = world.tiles();

    // upgrades :::
    upgrades.mining.derived_unlocked = true;
    upgrades.ladder.derived_unlocked = true;
    upgrades.bag.derived_unlocked = true;
    upgrades.climb_momentum.derived_unlocked = true;

    upgrades.jetpack.derived_unlocked =
        upgrades.mining.reached(MiningUpgradeKind::AlloyPickaxe) &&
        upgrades.ladder.reached(LadderUpgradeKind::FastClimb) &&
        upgrades.bag.reached(BagUpgradeKind::Sack) &&
        upgrades.climb_momentum.reached(ClimbMomentumUpgradeKind::ClimbMomentum);

    upgrades.jetpack_boost.derived_unlocked = upgrades.jetpack.derived_unlocked;
    upgrades.jetpack_fuel.derived_unlocked = upgrades.jetpack.derived_unlocked;
    upgrades.jetpack_storage.derived_unlocked = upgrades.jetpack.derived_unlocked;

    upgrades.demolisher.derived_unlocked =
        upgrades.jetpack_boost.reached(JetpackBoostUpgradeKind::BigBoost) &&
        upgrades.jetpack_fuel.reached(JetpackFuelUpgradeKind::LongHaulTanks) &&
        upgrades.jetpack_storage.reached(JetpackStorageUpgradeKind::XXLStorage);
        

    // frame start derived :::
    derived.player_at_overworld = player.trans.pos.y >= WORLD_SPAWN_F32.y*TILE_SIDE_F32-0.5;

    derived.player_mining_speed = match upgrades.mining.kind {
        MiningUpgradeKind::DefaultPickaxe => 1.0,
        MiningUpgradeKind::IronPickaxe => 1.5,
        MiningUpgradeKind::HardenedPickaxe => 2.0,
        MiningUpgradeKind::AlloyPickaxe => 3.0,
    };
    
    derived.player_ladder_speed = match upgrades.ladder.kind {
        LadderUpgradeKind::DefaultClimb => 1.0,
        LadderUpgradeKind::FastClimb => 1.6,
    };
    
    if player.mining_fatigue > 0.0 {
        derived.player_ladder_speed = 0.8; 
    }

    derived.player_bag_carry_capacity = match upgrades.bag.kind {
        BagUpgradeKind::DefaultBag => 6,
        BagUpgradeKind::SmallPouch => 10,
        BagUpgradeKind::BiggerPouch => 24,
        BagUpgradeKind::Backpack => 40,
        BagUpgradeKind::Sack => 64,
    };

    derived.player_climb_momentum_max = match upgrades.climb_momentum.kind {
        ClimbMomentumUpgradeKind::NoClimbMomentum => 0.0,
        ClimbMomentumUpgradeKind::ClimbMomentum => 1.5,
    };

    derived.player_has_jetpack = match upgrades.jetpack.kind {
        JetpackUpgradeKind::NoJetpack => false,
        JetpackUpgradeKind::Jetpack => true,
    };

    if derived.player_has_jetpack {
        derived.player_bag_carry_capacity = match upgrades.jetpack_storage.kind {
            JetpackStorageUpgradeKind::DefaultStorage => 6,
            JetpackStorageUpgradeKind::XLStorage => 12,
            JetpackStorageUpgradeKind::XXLStorage => 24,
        };
    }
    

    derived.player_jetpack_fuel_capacity = match upgrades.jetpack_fuel.kind {
        JetpackFuelUpgradeKind::DefaultFuel => 30.0,
        JetpackFuelUpgradeKind::MediumTanks => 50.0,
        JetpackFuelUpgradeKind::DoubleTanks => 75.0,
        JetpackFuelUpgradeKind::LongHaulTanks => 120.0,
    };

    derived.player_jetpack_speed = match upgrades.jetpack_boost.kind {
        JetpackBoostUpgradeKind::NoBoost => 75.0,
        JetpackBoostUpgradeKind::SmallBoost => 115.0,
        JetpackBoostUpgradeKind::BigBoost => 150.0,
    };
    
    derived.bought_demolisher = match upgrades.demolisher.kind {
        DemolisherUpgradeKind::NoDemolisher => false,
        DemolisherUpgradeKind::Demolisher => true,
    };

    derived.player_can_place_ladder = !derived.player_has_jetpack;
    derived.player_can_use_jetpack = derived.player_has_jetpack &&
        player.trans.pos.y <= ELEVATOR_PLATFORM_END.y;

    derived.ui_main_menu = match game.main_ui_state {
        MainUIState::MainMenu => true,
        MainUIState::MainMenuCredits => true,
        MainUIState::InGame => false,
    };
    
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
    
    derived.player_anim_finished = player.anim.repeated > 0 && player.anim.finished;

    // player movement :::

    let mut player_movement = IVec2::ZERO;
    #[allow(unused_assignments)]
    let mut player_movement_f32 = Vec2::ZERO;

    'player_movement: {
        if derived.ui_main_menu { break 'player_movement; }
        if game.demolisher_started { break 'player_movement; }
        if late_derived.ui_is_active { break 'player_movement; }

        let pos = player.trans.pos;
        let mut new_pos = player.trans.pos;
        let size = player.trans.size;
        let tsize = player.tile_size;
        let htsize = tsize / 2.0;
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

        player_movement_f32 = player_movement.as_vec2();

        #[allow(unused_assignments)]
        let mut movement_dir = Vec2::ZERO;
        if derived.player_can_use_jetpack {
            movement_dir = player_movement_f32 * dt * derived.player_jetpack_speed;
        } else {
            movement_dir = player_movement_f32 * dt * 50.0;
        }
        
        if player_movement.x > 0 {
            player.sprite.flip_x = false;
        } else if player_movement.x < 0 {
            player.sprite.flip_x = true;
        }

        if late_derived.travelling_in_elevator {
            player.trans.pos += movement_dir;
            break 'player_movement;
        }

        if game.dev_mode {
            player.trans.pos += movement_dir * 10.0;
            break 'player_movement;
        }

        let tile_one_down = tile.down(1);
        
        // gravity
        if derived.player_can_use_jetpack {
            if player_movement.y <= 0 {
                movement_dir.y += -1.2 * TILE_SIDE as f32 * dt;
            }
        } else if !tile.kind.can_climb() {
            if !tile_one_down.kind.can_climb() || pos.y - tile.world_pos().y > 1.0 {
                movement_dir.y = -9.8 * TILE_SIDE as f32 * dt;
            }
        } else {
            // INFO: We add *2.5 here because we always subtract *2.0. So this actually makes
            // it 0.5. It is multiplied by player_movement_f32.y, so this is no longer a scalar value
            if player_movement_f32.y != 0.0 {
                player.climb_momentum += dt*2.5 * player_movement_f32.y;
                player.climb_momentum = f32::clamp(
                    player.climb_momentum, -derived.player_climb_momentum_max, derived.player_climb_momentum_max
                );
            }
            movement_dir.y *= derived.player_ladder_speed + player.climb_momentum.abs();
 
        }
        
        if player_movement != IVec2::ZERO {
            derived.player_moving = true;
        }

        new_pos += movement_dir;

        let right_intersection = World::query_intersected_tiles_y(&game.bump,
            new_pos.x+htsize.x, [pos.y, pos.y+tsize.y]
        );
        let left_intersection = World::query_intersected_tiles_y(&game.bump,
            new_pos.x-htsize.x+1.0, [pos.y, pos.y+tsize.y]
        );

        let top_intersection = World::query_intersected_tiles_x(&game.bump,
            [pos.x-htsize.x+1.0, pos.x+htsize.x], new_pos.y+tsize.y
        );
        let bottom_intersection = World::query_intersected_tiles_x(&game.bump,
            [pos.x-htsize.x+1.0, pos.x+htsize.x], new_pos.y
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

        let mut trans = player.trans;
        trans.pos = new_pos;
        trans.size = size;

        let elevator_collider = elevator_platform.trans
            .collider_offset_size(vec2(0.0, 4.9), elevator_platform.walk_collider);
        
        let change = trans.pos - player.trans.pos;
        if let Some((_point, normal, time)) = trans.collider().collides(elevator_collider, change) {
            trans.pos += normal * change.abs() * (1.0-time);
        }
        
        player.trans.pos = trans.pos;
    }
    
    // INFO: This allows it to slow down from both directions. It compounds when direction changes due to
    // player.climb_momentum.signum()
    player.climb_momentum = f32::max(player.climb_momentum.abs() - dt*2.0, 0.0) * player.climb_momentum.signum();
    player.mining_fatigue = f32::max(player.mining_fatigue-dt, 0.0);

    // hit str :::
    derived.player_hit_str = 1.0 / f32::max(player_movement.x.abs() as f32 + player_movement.y.abs() as f32, 1.0);
    
    // block mine :::
    let player_tile = tiles.at_world_pos(player.trans.pos);
    let mut world_commands = world.commands(&game.bump);

    let mut player_added_to_bags = Vec::with_capacity_in(4, &game.bump);

    'block_mine: {
        if derived.ui_main_menu { break 'block_mine; }
        if game.demolisher_started { break 'block_mine; }
        if late_derived.travelling_in_elevator { break 'block_mine; }

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

        let mined_pos = player.trans.pos + vec2(0.0, TILE_SIDE_F32/2.0);
        let mined_tile = tiles.at_world_pos(mined_pos);

        let tile = mined_tile.offset_by(touch_vec);
        let tile_one_up = tile.up(1);
        let tile_one_down = tile.down(1);

        if !tile.kind.can_mine() { break 'block_mine; }

        if touch_vec.x != 0 && tile_one_up.kind.can_climb() { break 'block_mine; }            
                
        if tile.kind.item_drop() != ItemKind::Air && player.carrying.length >= derived.player_bag_carry_capacity {
            if ui_inventory_bar_frame.anim.is_not(&assets.ui_inventory_bar_frame_full) {
                ui_inventory_bar_frame.anim = assets.ui_inventory_bar_frame_full.derive_anim();
            }
            break 'block_mine;
        }
        
        let durability = game.tile_durability_map.entry(tile.pos).or_insert(0.0);
        *durability += dt * 3.0 * derived.player_mining_speed * derived.player_hit_str;

        derived.player_mining = true;        

        if *durability > tile.kind.durability() {
            if tile.kind.item_drop() != ItemKind::Air {
                player_added_to_bags.push(tile.kind.item_drop());
            }
            
            if !derived.player_can_use_jetpack && touch_vec.y != 0 {
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

            player.mining_fatigue = 1.0;
            *durability = 0.0;
        }
    }

    world.apply_commands(world_commands);

    // lay ladder :::
    let mut world_commands = world.commands(&game.bump);
    let tiles = world.tiles();
    let player_tile = tiles.at_world_pos(player.trans.pos);

    'lay_ladder: {
        if derived.ui_main_menu { break 'lay_ladder; }
        if game.demolisher_started { break 'lay_ladder; }
        if derived.player_can_use_jetpack { break 'lay_ladder; }
        if !derived.player_can_place_ladder { break 'lay_ladder; }
        if late_derived.travelling_in_elevator { break 'lay_ladder; }

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
    
    // sound effects :::   
    // NOTE: I hate this.
            
    if !derived.player_can_use_jetpack && player.anim.is( &assets.player_hit ) && game.sfx_pickaxe < player.anim.repeated {
        game.sfx_pickaxe = player.anim.repeated;
        audio::play_sound(&assets.sfx_pickaxe, audio::PlaySoundParams { looped: false, volume: 0.2 });
    }
    
    if !derived.player_can_use_jetpack && player.anim.is_not( &assets.player_hit ) && game.sfx_pickaxe > 0 {
        game.sfx_pickaxe = 0;
    }
    
    if derived.player_can_use_jetpack && player.anim.is( &assets.player_jetpack_hit ) && game.sfx_pickaxe < player.anim.repeated {
        game.sfx_pickaxe = player.anim.repeated;
        audio::play_sound(&assets.sfx_pickaxe, audio::PlaySoundParams { looped: false, volume: 0.2 });
    }
    
    if derived.player_can_use_jetpack && player.anim.is_not( &assets.player_jetpack_hit ) && game.sfx_pickaxe > 0 {
        game.sfx_pickaxe = 0;
    }
    
    // player animations :::
    if !derived.player_mining && derived.player_moving {
        if player.anim.is( &assets.player_idle ) {
            player.anim = assets.player_walk.derive_anim();
        } else if player.anim.is( &assets.player_jetpack_idle ) {
            audio::play_sound(&assets.sfx_jetpack, audio::PlaySoundParams { looped: true, volume: 0.1 });
            player.anim = assets.player_jetpack_move.derive_anim();
        }
    }
    if !derived.player_mining && !derived.player_moving {
        if player.anim.is( &assets.player_walk ) {
            player.anim = assets.player_idle.derive_anim();
        } else if player.anim.is( &assets.player_jetpack_move ) {
            audio::stop_sound(&assets.sfx_jetpack);
            player.anim = assets.player_jetpack_idle.derive_anim();
        }
    }
    if !derived.player_mining && derived.player_anim_finished {
        if player.anim.is( &assets.player_hit) {
            player.anim = assets.player_idle.derive_anim();
        } else if player.anim.is( &assets.player_jetpack_hit) {
            audio::stop_sound(&assets.sfx_jetpack);
            player.anim = assets.player_jetpack_idle.derive_anim();
        }
    }
    if derived.player_mining {
        if !derived.player_can_use_jetpack && player.anim.is_not( &assets.player_hit ) {
            player.anim = assets.player_hit.derive_anim();
        } else if derived.player_can_use_jetpack && player.anim.is_not( &assets.player_jetpack_hit ) {
            player.anim = assets.player_jetpack_hit.derive_anim();
        }
    }

    // player animations elevator reset :::
    if derived.player_can_use_jetpack {
        if player.anim.is( &assets.player_idle ) {
            player.anim = assets.player_jetpack_idle.derive_anim();
        }
        if player.anim.is( &assets.player_walk ) {
            player.anim = assets.player_jetpack_move.derive_anim();
        }
        if player.anim.is( &assets.player_hit ) {
            player.anim = assets.player_jetpack_hit.derive_anim();
        }
    } else {
        if player.anim.is( &assets.player_jetpack_idle ) {
            player.anim = assets.player_idle.derive_anim();
        }
        if player.anim.is( &assets.player_jetpack_move) {
            player.anim = assets.player_walk.derive_anim();
        }
        if player.anim.is( &assets.player_jetpack_hit ) {
            player.anim = assets.player_hit.derive_anim();
        }
    }

    
    // jetpack out of fuel :::
    if derived.player_can_use_jetpack && derived.player_moving && player.jetpack_fuel <= 0.0 {
        player.jetpack_out_of_fuel_tick += dt;
        if player.jetpack_out_of_fuel_tick >= 1.0 {
            if let Some(item_kind) = player.carrying.pop() {
                game.dropped_items.push(DroppedItem {
                    trans: Transform {
                        pos: player.trans.pos,
                        size: vec2(0.0, 0.0),
                        offset: vec2(0.0, 0.0),
                    },
                    kind: item_kind,
                    accumulated_tick: 0.0,
                });
            }
            player.jetpack_out_of_fuel_tick = 0.0;
        }
    }

    // overworld refill :::
    if derived.player_at_overworld && derived.player_has_jetpack {
        player.jetpack_fuel = f32::min(player.jetpack_fuel+dt*10.0, derived.player_jetpack_fuel_capacity);
    }

    // jetpack fuel use :::
    if derived.player_can_use_jetpack && derived.player_moving {
        player.jetpack_fuel = f32::max(player.jetpack_fuel-dt, 0.0);
    }

    // jetpack fuel blink :::
    if derived.player_has_jetpack {
        let blink_threshold = derived.player_jetpack_fuel_capacity*0.5;
        let modifier = (1.0 - player.jetpack_fuel/blink_threshold)*1.0;

        if player.jetpack_fuel <= blink_threshold {
            if ui_fuel_bar_frame.anim.is_not(&assets.ui_fuel_bar_frame_empty) {
                ui_fuel_bar_frame.anim = assets.ui_fuel_bar_frame_empty.derive_anim();
            }
            ui_fuel_bar_frame.anim.modifier = modifier;
        } else {
            if ui_fuel_bar_frame.anim.is_not(&assets.ui_fuel_bar_frame) {
                ui_fuel_bar_frame.anim = assets.ui_fuel_bar_frame.derive_anim();
            }
        }        
    }

    // hit str -> mining speed :::
    if player.anim.is( &assets.player_hit ) || player.anim.is( &assets.player_jetpack_hit ) {
        player.anim.modifier = derived.player_hit_str;
    }
    

    // bag handling
    for item_kind in player_added_to_bags {
        if player.carrying.length >= derived.player_bag_carry_capacity { break; }
        player.carrying.push(item_kind);        
    }

    // inside elevator :::
    if  game.elevator_spawned &&
        player.trans.collider().contains(elevator_platform.trans.collider())
    {
        if elevator_platform.player_inside_for < 5.0 {
            let anim = &assets.elevator_platform_countdown[elevator_platform.player_inside_for as usize];
            if elevator_platform.anim.is_not(anim) {
                elevator_platform.anim = anim.derive_anim();
            }
            elevator_platform.player_inside_for += dt;
        }
    }
    
    // riding elevator :::
    if  game.elevator_spawned &&
        elevator_platform.player_inside_for >= 5.0
    {
        let direction = if elevator_platform.down_or_up { -520.0 } else { 520.0 };
        elevator_platform.trans.pos.y -= direction*dt;
                
        if elevator_platform.trans.pos.y <= ELEVATOR_PLATFORM_END_STOP.y {
            elevator_platform.trans.pos.y = ELEVATOR_PLATFORM_END_STOP.y;
            elevator_platform.player_inside_for = 0.0;
            elevator_platform.anim = assets.elevator_platform_idle.derive_anim();
            elevator_platform.down_or_up = !elevator_platform.down_or_up;
        }
        
        if elevator_platform.trans.pos.y >= ELEVATOR_PLATFORM_START.y {
            elevator_platform.trans.pos.y = ELEVATOR_PLATFORM_START.y;
            elevator_platform.player_inside_for = 0.0;
            elevator_platform.anim = assets.elevator_platform_idle.derive_anim();
            elevator_platform.down_or_up = !elevator_platform.down_or_up;
        }

        if elevator_platform.anim.is_not(&assets.elevator_platform_moving) && elevator_platform.player_inside_for > 0.0 {
            audio::play_sound(&assets.sfx_elevator, audio::PlaySoundParams { looped: false, volume: 0.2 });
            elevator_platform.anim = assets.elevator_platform_moving.derive_anim();
        }
        
        if player.trans.collider().p1.x < elevator_platform.trans.collider().p1.x {
            player.trans.pos.x = elevator_platform.trans.collider().p1.x - player.trans.offset.x+1.0;
        }
        if player.trans.collider().p2.x > elevator_platform.trans.collider().p2.x {
            player.trans.pos.x = elevator_platform.trans.collider().p2.x + player.trans.offset.x-2.0;
        }
        
        player.trans.pos.y = elevator_platform.trans.pos.y+elevator_platform.trans.offset.y+5.0;
        
        derived.player_can_use_jetpack = false;
        next_late_derived.travelling_in_elevator = true;
    }
    
    // outside elevator :::
    if  game.elevator_spawned &&
        !player.trans.collider().contains(elevator_platform.trans.collider())
    {
        elevator_platform.player_inside_for = 0.0;
        if elevator_platform.anim.is_not(&assets.elevator_platform_idle) {
            elevator_platform.anim = assets.elevator_platform_idle.derive_anim();
        }
    }

    // minecart collect items :::
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
            audio::play_sound(&assets.sfx_minecart_transfer, audio::PlaySoundParams { looped: false, volume: 0.2 });
        }
    }

    // minecart start move :::
    if minecart.movement == MinecartMovement::Idle && minecart.cooldown <= 0.1 && minecart.carrying.length > 0 {
        minecart.movement = MinecartMovement::Forwards;
        minecart.anim = assets.minecart_moving.derive_anim();
        audio::play_sound(&assets.sfx_minecart_moving, audio::PlaySoundParams { looped: true, volume: 0.2 });
    }

    minecart.cooldown = f32::max(minecart.cooldown-dt, 0.0);

    // minecart moving forwards :::
    if minecart.movement == MinecartMovement::Forwards {
        let mut new_pos = minecart.trans.pos + vec2(minecart.speed, 0.0) * dt;
        let mut new_rotation = 0.0;

        if new_pos.x >= MINECART_STRAIGHT_END.x {
            new_pos = minecart.trans.pos + (MINECART_DIAGONAL_END-MINECART_STRAIGHT_END).normalize()
                * minecart.speed * dt;
            new_rotation = 32.0f32.to_radians();
        }
        minecart.trans.pos = new_pos;
        minecart.rotation = new_rotation;
        minecart.speed = f32::min(minecart.speed + dt*55.0, 180.0);
        
        if new_pos.x >= MINECART_DIAGONAL_END.x {
            minecart.movement = MinecartMovement::Backwards;
        }
    }
    
    // minecart moving backwards :::
    if minecart.movement == MinecartMovement::Backwards {
        let mut new_pos = minecart.trans.pos + (MINECART_STRAIGHT_END-MINECART_DIAGONAL_END).normalize()
            * minecart.speed * dt;
        let mut new_rotation = 22.0f32.to_radians();
        
        if new_pos.x <= MINECART_STRAIGHT_END.x {
            new_pos = minecart.trans.pos - vec2(minecart.speed, 0.0) * dt;
            new_pos.y = MINECART_STRAIGHT_END.y;
            new_rotation = 0.0;
        }
        minecart.trans.pos = new_pos;
        minecart.rotation = new_rotation;
        
        minecart.speed = f32::min(minecart.speed + dt*55.0, 250.0);

        if new_pos.x <= MINECART_START.x {
            audio::stop_sound(&assets.sfx_minecart_moving);
            audio::play_sound(&assets.sfx_minecart_throw, audio::PlaySoundParams { looped: false, volume: 0.2 });
            minecart.trans.pos.x = MINECART_START.x;
            minecart.movement = MinecartMovement::Idle;
            minecart.anim = assets.minecart_idle.derive_anim();
            minecart.speed = 50.0;

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
                let x = rand::gen_range(-75.0, -90.0);
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
    
    // statue interact :::
    if player.trans.collider().intersects(statue.trans.collider()) {
        derived.ui_show_statue_key = true;

        if input_actions.interact {
            game.ui_show_statue = !game.ui_show_statue;
        }
    }

    // demolisher interact :::
    if !game.demolisher_started && player.trans.collider().intersects(demolisher.trans.collider()) {
        derived.ui_show_demolisher_key = true;

        if input_actions.interact {
            game.demolisher_started = true;
            audio::play_sound(&assets.sfx_demolisher, audio::PlaySoundParams { looped: false, volume: 0.2 });
            demolisher.anim = assets.demolisher_working_0.derive_anim();
        }
    }

    // demolisher heat :::
    if game.demolisher_started && demolisher.stage < 5 {
        demolisher.stage_tick += dt;
        demolisher.anim.modifier += dt/4.0;
        if demolisher.stage_tick >= 2.0 {
            demolisher.stage_tick = 0.0;
            demolisher.stage += 1;
            let anims = [
                &assets.demolisher_working_0,
                &assets.demolisher_working_1,
                &assets.demolisher_working_2,
                &assets.demolisher_working_3,
                &assets.demolisher_working_4,
                &assets.demolisher_working_5,
            ];
            let modifier = demolisher.anim.modifier;
            demolisher.anim = anims[demolisher.stage].derive_anim();
            demolisher.anim.modifier = modifier;
        }
    }

    // demolisher moving :::
    if game.demolisher_started && demolisher.stage == 5 {
        demolisher.momentum += 100.0*dt;
        demolisher.trans.pos.x -= demolisher.momentum*dt;
        let tile_start = tiles.at_world_pos(demolisher.trans.pos);
        let tile_end = tiles.at_world_pos(demolisher.prev_pos);        
        
        world_commands.set_tile_area(tile_start.pos, tile_end.pos-tile_start.pos+ivec2(1, 1), Tile::BackgroundStone);
        demolisher.prev_pos = demolisher.trans.pos;
        player.trans.pos.x = demolisher.trans.pos.x;
    }

    // collect coins :::
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
        audio::play_sound(&assets.sfx_coin, audio::PlaySoundParams { looped: false, volume: 0.15 });
    }

    // tick dropped items :::
    let mut dropped_items_to_remove = Vec::new_in(&game.bump);
    
    for (i, item) in &mut game.dropped_items.iter_mut().enumerate().rev() {
        item.trans.pos.y += -9.8 * TILE_SIDE_F32 * dt;
        if item.accumulated_tick >= 10.0 {
            dropped_items_to_remove.push(i);
        }
        item.accumulated_tick += dt;
    }

    for i in dropped_items_to_remove {
        game.dropped_items.swap_remove(i);
    }


    // reset inventory full anim after one repeat :::
    if ui_inventory_bar_frame.anim.is(&assets.ui_inventory_bar_frame_full) && ui_inventory_bar_frame.anim.repeated > 0 {
        ui_inventory_bar_frame.anim = assets.ui_inventory_bar_frame.derive_anim();
    }

    // tick animations :::
    if !derived.ui_main_menu {
        tick_animation(&mut elevator_cage.sprite, &mut elevator_cage.anim, dt);
        tick_animation(&mut elevator_platform.sprite, &mut elevator_platform.anim, dt);
        tick_animation(&mut minecart.sprite, &mut minecart.anim, dt);
        tick_animation(&mut player.sprite, &mut player.anim, dt);
        tick_animation(&mut ui_inventory_bar_frame.sprite, &mut ui_inventory_bar_frame.anim, dt);
        tick_animation(&mut ui_fuel_bar_frame.sprite, &mut ui_fuel_bar_frame.anim, dt);
        tick_animation(&mut ui_fuel_bar_frame.sprite, &mut ui_fuel_bar_frame.anim, dt);
        tick_animation(&mut demolisher.sprite, &mut demolisher.anim, dt);
    }
    
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

    // handle escape key :::
    'escape: {
        if input_actions.escape {
            if game.ui_show_statue {
                game.ui_show_statue = false;
                break 'escape;
            }
        }
    }
    
    // spawn elevator :::
    if derived.player_has_jetpack && !game.elevator_spawned {
        game.elevator_spawned = true;   
        world_commands.set_tile_area(
            world_pos_to_tile_pos(ELEVATOR_PLATFORM_END)+ivec2(0, 1),
            world_pos_to_tile_pos(ELEVATOR_PLATFORM_START-ELEVATOR_PLATFORM_END)+ivec2(1, 0),
            Tile::BackgroundStoneElevatorLeft,
        );
        world_commands.set_tile_area(
            world_pos_to_tile_pos(ELEVATOR_PLATFORM_END)+ivec2(1, 1),
            world_pos_to_tile_pos(ELEVATOR_PLATFORM_START-ELEVATOR_PLATFORM_END)+ivec2(1, 0),
            Tile::BackgroundStoneElevatorMiddle,
        );
        world_commands.set_tile_area(
            world_pos_to_tile_pos(ELEVATOR_PLATFORM_END)+ivec2(2, 1),
            world_pos_to_tile_pos(ELEVATOR_PLATFORM_START-ELEVATOR_PLATFORM_END)+ivec2(1, 0),
            Tile::BackgroundStoneElevatorRight,
        );
    }

    // spawn demolisher :::
    if derived.bought_demolisher && !game.demolisher_spawned {
        game.demolisher_spawned = true;
    }

    // apply commands & updates :::
    world.apply_commands(world_commands);
    world.apply_updates(&assets.tile_set);
    
    // move late derived :::
    next_late_derived.ui_is_active = game.ui_show_statue;
    game.late_derived = next_late_derived;

    // post update :::
    game.bump.reset();
}
