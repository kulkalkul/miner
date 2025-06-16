use macroquad::{audio, prelude::*};

use crate::sprite::{ load_anim, load_sheet_cell, load_sheet_cells, load_sprite, load_three_patch };
use crate::sprite::{ Offset, Size, RowCol };
use crate::sprite::{ SpriteAsset };

use crate::tile::{ load_tile_set };
use crate::tile::{ TileSetAsset };

pub struct Assets {
    pub sfx_pickaxe: audio::Sound,
    pub sfx_coin: audio::Sound,
    pub sfx_minecart_transfer: audio::Sound,
    pub sfx_minecart_moving: audio::Sound,
    pub sfx_minecart_throw: audio::Sound,
    pub sfx_elevator: audio::Sound,
    pub sfx_jetpack: audio::Sound,
    pub sfx_demolisher: audio::Sound,
    pub sfx_ui_positive: audio::Sound,
    pub sfx_ui_negative: audio::Sound,
    pub sfx_soundtrack: audio::Sound,

    pub ui_bg: SpriteAsset,
    pub ui_keys: SpriteAsset,
    pub ui_button: [[SpriteAsset; 3]; 3],
    
    pub ui_inventory_bar_background: SpriteAsset,
    pub ui_inventory_bar_frame: SpriteAsset,
    pub ui_inventory_bar_frame_full: SpriteAsset,
    pub ui_inventory_bar_fill: SpriteAsset,
    
    pub ui_fuel_bar_background: SpriteAsset,
    pub ui_fuel_bar_frame: SpriteAsset,
    pub ui_fuel_bar_frame_empty: SpriteAsset,
    pub ui_fuel_bar_fill: SpriteAsset,
    
    pub ui_elevator_arrow: Box<[SpriteAsset]>,
    pub ui_demolisher_arrow: SpriteAsset,

    pub cracking: Box<[SpriteAsset]>,
    pub coin: SpriteAsset,    
    pub coins: Box<[SpriteAsset]>,

    pub sign_sell: SpriteAsset,
    pub sign_descend: SpriteAsset,
    pub sign_jetpack_hint: SpriteAsset,

    pub player_idle: SpriteAsset,
    pub player_walk: SpriteAsset,
    pub player_hit: SpriteAsset,
    pub player_jetpack_idle: SpriteAsset,
    pub player_jetpack_move: SpriteAsset,
    pub player_jetpack_hit: SpriteAsset,

    pub statue: SpriteAsset,

    pub elevator_cage: SpriteAsset,
    pub elevator_platform_idle: SpriteAsset,
    pub elevator_platform_countdown: Box<[SpriteAsset]>,
    pub elevator_platform_moving: SpriteAsset,

    pub minecart_idle: SpriteAsset,
    pub minecart_moving: SpriteAsset,

    pub demolisher_idle: SpriteAsset,
    pub demolisher_working_0: SpriteAsset,
    pub demolisher_working_1: SpriteAsset,
    pub demolisher_working_2: SpriteAsset,
    pub demolisher_working_3: SpriteAsset,
    pub demolisher_working_4: SpriteAsset,
    pub demolisher_working_5: SpriteAsset,
    
    pub rail_start: SpriteAsset,
    pub rail: SpriteAsset,
    pub rail_diagonal: SpriteAsset,

    pub items: Box<[SpriteAsset]>,
    pub tile_set: TileSetAsset,
}

pub struct AssetState {
    pub asset_id: u64,
}

macro_rules! load_asset_texture {
    ($path:literal) => {
        if cfg!(not(target_family = "wasm")) {
            let path = ["asset/", $path, ".png"].join("");
            load_texture(&path).await.expect("Texture should exist")
        } else if cfg!(target_family = "wasm") {
            let bytes = include_bytes!(concat!("../asset/", $path, ".png"));
            Texture2D::from_file_with_format(&bytes[..], None)
        } else {
            unimplemented!();
        }
    }
}

macro_rules! load_asset_sound {
    ($path:literal) => {
        if cfg!(not(target_family = "wasm")) {
            let path = ["asset/", $path, ".wav"].join("");
            audio::load_sound(&path).await.expect("Sound should exist")
        } else if cfg!(target_family = "wasm") {
            let bytes = include_bytes!(concat!("../asset/", $path, ".wav"));
            audio::load_sound_from_bytes(&bytes[..]).await.expect("Sound should exist")
        } else {
            unimplemented!();
        }
    }
}

pub async fn init_assets() -> Assets {
    let ui_bg_tex = load_asset_texture!("ui_bg");
    let ui_keys_tex = load_asset_texture!("ui_keys");
    let ui_button_tex = load_asset_texture!("ui_button");
    let ui_inventory_bar_tex = load_asset_texture!("ui_inventory_bar");
    let ui_fuel_bar_tex = load_asset_texture!("ui_fuel_bar");
    let ui_elevator_arrow = load_asset_texture!("ui_elevator_arrow");
    let ui_demolisher_arrow = load_asset_texture!("ui_demolisher_arrow");

    let cracking_tex = load_asset_texture!("cracking");
    let coin_tex = load_asset_texture!("coin");
    let coins_tex = load_asset_texture!("coins");

    let elevator_cage_tex = load_asset_texture!("elevator_cage");
    let elevator_platform_tex = load_asset_texture!("elevator_platform");

    let player_tex = load_asset_texture!("player");
    let statue_tex = load_asset_texture!("statue");
    let signs_tex = load_asset_texture!("signs");
    
    let minecart_tex = load_asset_texture!("minecart");
    let rail_tex = load_asset_texture!("rail");
    let demolisher_tex = load_asset_texture!("demolisher");
    
    let items_tex = load_asset_texture!("items");
    let tile_set_tex = load_asset_texture!("tile_set");

    let sfx_pickaxe = load_asset_sound!("pickaxe");
    let sfx_coin = load_asset_sound!("coin");
    let sfx_minecart_transfer = load_asset_sound!("minecart_transfer");
    let sfx_minecart_moving = load_asset_sound!("minecart_moving");
    let sfx_minecart_throw = load_asset_sound!("minecart_throw");
    let sfx_elevator = load_asset_sound!("elevator");
    let sfx_jetpack = load_asset_sound!("jetpack");
    let sfx_demolisher = load_asset_sound!("demolisher");
    let sfx_ui_positive = load_asset_sound!("ui_positive");
    let sfx_ui_negative = load_asset_sound!("ui_negative");
    let sfx_soundtrack = load_asset_sound!("soundtrack");

    let mut state = AssetState { asset_id: 0 };

    Assets {
        sfx_pickaxe,
        sfx_coin,
        sfx_minecart_transfer,
        sfx_minecart_moving,
        sfx_minecart_throw,
        sfx_elevator,
        sfx_jetpack,
        sfx_demolisher,
        sfx_ui_positive,
        sfx_ui_negative,
        sfx_soundtrack,

        ui_bg: load_sprite(&mut state, &ui_bg_tex, Offset(0, 0), Size(270, 190)),
        ui_keys: load_sheet_cell(&mut state, &ui_keys_tex, RowCol(0, 0), Size(26, 23)),
        ui_button: load_three_patch(&mut state, &ui_button_tex),

        ui_inventory_bar_background: load_sheet_cell(&mut state, &ui_inventory_bar_tex, RowCol(0, 0), Size(16, 58)),
        ui_inventory_bar_frame: load_sheet_cell(&mut state, &ui_inventory_bar_tex, RowCol(1, 0), Size(16, 58)),
        ui_inventory_bar_frame_full: load_anim(&mut state, &ui_inventory_bar_tex, RowCol(2, 0), 3, Size(16, 58), 150.0),
        ui_inventory_bar_fill: load_sheet_cell(&mut state, &ui_inventory_bar_tex, RowCol(5, 0), Size(16, 58)),
        
        ui_fuel_bar_background: load_sheet_cell(&mut state, &ui_fuel_bar_tex, RowCol(0, 0), Size(16, 58)),
        ui_fuel_bar_frame: load_sheet_cell(&mut state, &ui_fuel_bar_tex, RowCol(1, 0), Size(16, 58)),
        ui_fuel_bar_frame_empty: load_anim(&mut state, &ui_fuel_bar_tex, RowCol(2, 0), 3, Size(16, 58), 150.0),
        ui_fuel_bar_fill: load_sheet_cell(&mut state, &ui_fuel_bar_tex, RowCol(5, 0), Size(16, 58)),

        ui_elevator_arrow: load_sheet_cells(&mut state, &ui_elevator_arrow, RowCol(0, 0), 2, Size(39, 9)),
        ui_demolisher_arrow: load_sheet_cell(&mut state, &ui_demolisher_arrow, RowCol(0, 0), Size(47, 9)),
        
        cracking: load_sheet_cells(&mut state, &cracking_tex, RowCol(0, 0), 4, Size(16, 16)),
        coin: load_sprite(&mut state, &coin_tex, Offset(0, 0), Size(16, 16)),
        coins: load_sheet_cells(&mut state, &coins_tex, RowCol(0, 0), 5, Size(32, 32)),

        sign_sell: load_sheet_cell(&mut state, &signs_tex, RowCol(0, 0), Size(36, 15)),
        sign_descend: load_sheet_cell(&mut state, &signs_tex, RowCol(1, 0), Size(36, 15)),
        sign_jetpack_hint: load_sheet_cell(&mut state, &signs_tex, RowCol(2, 0), Size(36, 15)),

        player_idle: load_anim(&mut state, &player_tex, RowCol(0, 0), 2, Size(16, 16), 400.0),
        player_walk: load_anim(&mut state, &player_tex, RowCol(2, 0), 2, Size(16, 16), 150.0),
        player_hit:  load_anim(&mut state, &player_tex, RowCol(4, 0), 2, Size(16, 16), 150.0),
        player_jetpack_idle: load_anim(&mut state, &player_tex, RowCol(6, 0), 1, Size(16, 16), 150.0),
        player_jetpack_move: load_anim(&mut state, &player_tex, RowCol(7, 0), 2, Size(16, 16), 150.0),
        player_jetpack_hit:  load_anim(&mut state, &player_tex, RowCol(9, 0), 2, Size(16, 16), 150.0),

        statue: load_sheet_cell(&mut state, &statue_tex, RowCol(0, 0), Size(32, 48)),
        
        elevator_cage: load_anim(&mut state, &elevator_cage_tex , RowCol(0, 0), 12, Size(48, 41), 100.0),
        elevator_platform_idle: load_anim(
            &mut state, &elevator_platform_tex , RowCol(0, 0), 1, Size(48, 41), 400.0
        ),
        elevator_platform_countdown: load_sheet_cells(
            &mut state, &elevator_platform_tex, RowCol(1, 0), 5, Size(48 , 41)
        ),
        elevator_platform_moving: load_anim(
            &mut state, &elevator_platform_tex , RowCol(6, 0), 4, Size(48, 41), 100.0
        ),
        
        minecart_idle:   load_anim(&mut state, &minecart_tex, RowCol(0, 0), 1, Size(15 , 16)  , 400.0),
        minecart_moving: load_anim(&mut state, &minecart_tex, RowCol(1, 0), 4, Size(15 , 16)  , 150.0),

        demolisher_idle: load_anim(&mut state, &demolisher_tex, RowCol(0, 0), 1, Size(52, 10), 400.0),
        demolisher_working_0: load_anim(&mut state, &demolisher_tex, RowCol(1 , 0), 5, Size(52, 10), 150.0),
        demolisher_working_1: load_anim(&mut state, &demolisher_tex, RowCol(6 , 0), 5, Size(52, 10), 150.0),
        demolisher_working_2: load_anim(&mut state, &demolisher_tex, RowCol(11, 0), 5, Size(52, 10), 150.0),
        demolisher_working_3: load_anim(&mut state, &demolisher_tex, RowCol(16, 0), 5, Size(52, 10), 150.0),
        demolisher_working_4: load_anim(&mut state, &demolisher_tex, RowCol(21, 0), 5, Size(52, 10), 150.0),
        demolisher_working_5: load_anim(&mut state, &demolisher_tex, RowCol(26, 0), 5, Size(52, 10), 150.0),
        
        rail_start:      load_sheet_cell(&mut state, &rail_tex, RowCol(0, 0), Size(16 , 9)),
        rail:            load_sheet_cell(&mut state, &rail_tex, RowCol(1, 0), Size(16 , 9)),
        rail_diagonal:   load_sheet_cell(&mut state, &rail_tex, RowCol(2, 0), Size(16 , 9)),

        items:    load_sheet_cells(&mut state, &items_tex, RowCol(0, 0), 7, Size(16, 16)),
        tile_set: load_tile_set(&mut state, &tile_set_tex, ivec2(16, 16)),
    }
}
