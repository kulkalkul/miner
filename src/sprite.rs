use std::sync::Arc;

use macroquad::prelude::*;

use crate::asset::AssetState;

#[derive(Debug)]
pub struct Sprite {
    pub asset_id: u64,
    pub texture: Texture2D,
    pub texture_frame: Rect,
    pub flip_x: bool,
}

#[derive(Debug)]
pub struct Animation {
    pub asset_id: u64,
    pub frames: Arc<[Rect]>,
    pub index: usize,
    pub frame_timer: f32,
    pub accumulated: f32,
    pub repeated: i32,
}

#[derive(Debug)]
pub struct SpriteAsset {
    pub asset_id: u64,
    pub texture: Texture2D,
    pub frames: Arc<[Rect]>,
    pub frame_timer: f32,
}

pub struct Offset(pub i32, pub i32);
pub struct RowCol(pub i32, pub i32);
pub struct Size(pub i32, pub i32);

impl Sprite {
    pub fn local_offset_with_flip(&self, offset: Vec2) -> Vec2 {
        if !self.flip_x {
            vec2(self.texture_frame.w - offset.x, offset.y)
        } else {
            offset
        }
    }
}

impl Animation {
    pub fn is(&self, sprite_asset: &SpriteAsset) -> bool {
        return self.asset_id == sprite_asset.asset_id;
    }
    pub fn is_not(&self, sprite_asset: &SpriteAsset) -> bool {
        return self.asset_id != sprite_asset.asset_id;
    }
}

impl SpriteAsset {
    pub fn derive_sprite(&self) -> Sprite {
        Sprite {
            asset_id: self.asset_id,
            // TODO: use weak
            texture: Texture2D::clone(&self.texture),
            texture_frame: self.frames[0],
            flip_x: false,
        }
    }
    pub fn derive_anim(&self) -> Animation {
        Animation {
            asset_id: self.asset_id,
            // TODO: use something else than arc, probs index
            frames: Arc::clone(&self.frames),
            index: self.frames.len(),
            frame_timer: self.frame_timer,
            accumulated: self.frame_timer,
            repeated: -1,
        }
    }
}

pub fn load_sprite(
    asset_state: &mut AssetState,
    texture: &Texture2D,
    offset: Offset,
    size: Size
) -> SpriteAsset {
    let asset_id = asset_state.asset_id;
    asset_state.asset_id += 1;

    let texture = texture.clone();
    let Offset(offset_x, offset_y) = offset;
    let Size(width, height) = size;

    let offset_x = offset_x as f32;
    let offset_y = offset_y as f32;

    let texture_sizef = texture.size();
    texture.set_filter(FilterMode::Nearest);
    
    let frame = Rect::new(offset_x, offset_y, width as f32, height as f32);

    assert!(frame.x + frame.w <= texture_sizef.x);
    assert!(frame.y + frame.h <= texture_sizef.y);

    SpriteAsset {
        asset_id,
        texture,
        frames: [frame].into(),
        frame_timer: f32::INFINITY,
    }
}

pub fn load_anim(
    asset_state: &mut AssetState,
    texture: &Texture2D,
    row_col: RowCol,
    frame_count: u32,
    size: Size,
    frame_timer_ms: f32,
) -> SpriteAsset {
    // 2.0 is padding due to both borders, while 1.0 is individual padding offset
    let asset_id = asset_state.asset_id;
    asset_state.asset_id += 1;

    let texture = texture.clone();
    let RowCol(row, col) = row_col;
    let Size(width, height) = size;

    let widthf = width as f32;
    let heightf = height as f32;

    let row_offset = row as f32 * (widthf + 2.0);
    let col_offset = col as f32 * (heightf + 2.0);

    let texture_sizef = texture.size();
    let texture_rows = (texture_sizef.x / widthf) as i32;
    let texture_cols = (texture_sizef.y / heightf) as i32;
    texture.set_filter(FilterMode::Nearest);

    assert!(row_offset <= texture_sizef.x);
    assert!(col_offset <= texture_sizef.y);

    let mut frames = Vec::with_capacity(frame_count as usize);

    let remaining_in_first_row = i32::min(texture_rows - row, frame_count as i32);

    let mut x = row_offset;
    let mut y = col_offset;
    
    for _ in 0..remaining_in_first_row {
        frames.push(Rect::new(x+1.0, y+1.0, widthf, heightf));
        x += widthf + 2.0;
    }

    let remaining_in_other_cols = frame_count as i32 - remaining_in_first_row;
    let remaining_cols = remaining_in_other_cols / texture_rows;
    let remaining_rows = remaining_in_other_cols % texture_rows;

    assert!(col + remaining_cols <= texture_cols);

    for _ in 0..remaining_cols {
        x = 0.0;
        y += heightf;
        for _ in 0..texture_rows {
            frames.push(Rect::new(x+1.0, y+1.0, widthf, heightf));
            x += widthf + 2.0;
        }
    }

    y += heightf + 2.0;
    for _ in 0..remaining_rows {
        frames.push(Rect::new(x+1.0, y+1.0, widthf, heightf));
        x += widthf + 2.0;
    }
    
    SpriteAsset {
        asset_id,
        texture,
        frames: frames.into(),
        frame_timer: frame_timer_ms / 1000.0,
    }
}

pub fn tick_animation(sprite: &mut Sprite, anim: &mut Animation, dt: f32) {
    anim.accumulated += dt;
    if anim.accumulated > anim.frame_timer {
        anim.accumulated = anim.accumulated - anim.frame_timer;
        anim.index += 1;
        if anim.index >= anim.frames.len() {
            anim.index = 0;
            anim.repeated += 1;
        }
    } else {
        sprite.texture_frame = anim.frames[anim.index];
    }
}

pub fn draw_sprite(position: Vec2, sprite: &Sprite) {    
    draw_texture_ex(&sprite.texture, position.x, position.y, WHITE, DrawTextureParams {
        source: Some(sprite.texture_frame),
        flip_x: sprite.flip_x,
        flip_y: true, // because I enjoy y+
        ..Default::default()
    });
}

pub fn draw_sprite_scaled(position: Vec2, scale: Vec2, sprite: &Sprite) {    
    draw_texture_ex(&sprite.texture, position.x, position.y, WHITE, DrawTextureParams {
        dest_size: Some(sprite.texture_frame.size()*scale),
        source: Some(sprite.texture_frame),
        flip_x: sprite.flip_x,
        flip_y: true, // because I enjoy y+
        ..Default::default()
    });
}

pub fn draw_sprite_offset(position: Vec2, offset: Vec2, sprite: &Sprite) {
    draw_sprite(position + offset, sprite);
}

pub fn draw_ui(position: Vec2, sprite: &Sprite) {
    draw_texture_ex(&sprite.texture, position.x, position.y, WHITE, DrawTextureParams {
        source: Some(sprite.texture_frame),
        ..Default::default()
    });
}
