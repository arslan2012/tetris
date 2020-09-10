use crate::texture_group::texture_with_relative_position::TextureWithRelativePosition;
use sdl2::render::{Texture, Canvas, BlendMode};
use sdl2::video::Window;
use sdl2::rect::Rect;

mod texture_with_relative_position;

pub struct TextureGroup<'a> {
    textures: Vec<TextureWithRelativePosition<'a>>
}

impl<'a> TextureGroup<'a> {
    pub fn new() -> TextureGroup<'a> {
        TextureGroup {
            textures: Vec::new()
        }
    }
    pub fn add(&mut self, texture: Texture<'a>, x: i32, y: i32) {
        self.textures.push(TextureWithRelativePosition { texture, x, y })
    }

    pub fn copy_to_canvas(&self, canvas: &mut Canvas<Window>, x: i32, y: i32) {
        for texture_with_relative_position in &self.textures {
            let dimensions = texture_with_relative_position.texture.query();
            canvas.copy(&texture_with_relative_position.texture,
                        None,
                        Rect::new(x + texture_with_relative_position.x,
                                  y + texture_with_relative_position.y,
                                  dimensions.width, dimensions.height))
                .expect("Couldn't copy texture into window");
        }
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        for texture_with_relative_position in &mut self.textures {
            texture_with_relative_position.texture.set_alpha_mod(alpha);
            texture_with_relative_position.texture.set_blend_mode(BlendMode::Blend);
        }
    }
}