use macroquad::prelude::*;

use crate::sprite::{ load_anim, load_sprite, load_sheet_cell, load_sheet_cells };
use crate::sprite::{ Offset, Size, RowCol };
use crate::sprite::{ SpriteAsset };

use crate::tile::{ load_tile_set };
use crate::tile::{ TileSetAsset };

pub struct Assets {
    pub ui_bg: SpriteAsset,
    pub ui_keys: SpriteAsset,

    pub coin: SpriteAsset,    
    pub coins: Box<[SpriteAsset]>,

    pub player_idle: SpriteAsset,
    pub player_walk: SpriteAsset,
    pub player_hit: SpriteAsset,

    pub statue: SpriteAsset,

    pub crusher_working: SpriteAsset,
    pub minecart_idle: SpriteAsset,
    pub minecart_moving: SpriteAsset,
    
    pub rail_start: SpriteAsset,
    pub rail: SpriteAsset,

    pub items: TileSetAsset,
    pub tile_set: TileSetAsset,
}

pub struct AssetState {
    pub asset_id: u64,
}

pub async fn init_assets() -> Assets {
    let ui_bg_tex = load_asset_texture("ui_bg").await;
    let ui_keys_tex = load_asset_texture("ui_keys").await;
    let coin_tex = load_asset_texture("coin").await;
    let coins_tex = load_asset_texture("coins").await;

    let player_tex = load_asset_texture("player").await;
    let statue_tex = load_asset_texture("statue").await;
    let crusher_tex = load_asset_texture("crusher").await;
    let minecart_tex = load_asset_texture("minecart").await;
    let rail_tex = load_asset_texture("rail").await;
    
    let items_tex = load_asset_texture("items").await;
    let tile_set_tex = load_asset_texture("tile_set").await;

    let mut state = AssetState { asset_id: 0 };

    Assets {
        ui_bg: load_sprite(&mut state, &ui_bg_tex, Offset(0, 0), Size(200, 141)),
        ui_keys: load_sheet_cell(&mut state, &ui_keys_tex, RowCol(0, 0), Size(26, 23)),
        
        coin: load_sprite(&mut state, &coin_tex, Offset(0, 0), Size(16, 16)),
        
        coins: load_sheet_cells(&mut state, &coins_tex, RowCol(0, 0), 5, Size(32, 32)),

        player_idle: load_anim(&mut state, &player_tex, RowCol(0, 0), 2, Size(16, 16), 400.0),
        player_walk: load_anim(&mut state, &player_tex, RowCol(2, 0), 2, Size(16, 16), 150.0),
        player_hit:  load_anim(&mut state, &player_tex, RowCol(4, 0), 1, Size(16, 16), 150.0),

        statue: load_sheet_cell(&mut state, &statue_tex, RowCol(0, 0), Size(32, 48)),
        
        crusher_working: load_anim(&mut state, &crusher_tex , RowCol(0, 0), 3, Size(256, 128), 200.0),
        
        minecart_idle:   load_anim(&mut state, &minecart_tex, RowCol(0, 0), 1, Size(15 , 16)  , 400.0),
        minecart_moving: load_anim(&mut state, &minecart_tex, RowCol(1, 0), 4, Size(15 , 16)  , 150.0),
        
        rail_start:      load_sheet_cell(&mut state, &rail_tex    , RowCol(0, 0), Size(16 , 6)),
        rail:            load_sheet_cell(&mut state, &rail_tex    , RowCol(1, 0), Size(16 , 6)),

        items:    load_tile_set(&mut state, &items_tex   , ivec2(16, 16)),
        tile_set: load_tile_set(&mut state, &tile_set_tex, ivec2(16, 16)),
    }
}


pub async fn load_asset_texture(path: &str) -> Texture2D {
    #[cfg(not(target_family = "wasm"))]
    let path = ["asset/", path, ".png"].join("");
    #[cfg(target_family = "wasm")]
    let path = ["./asset/", path, ".png"].join("");    
    load_texture(&path).await.expect("Texture should exist")
}
