use sdl2::render::Texture;

pub struct TextureWithRelativePosition<'a> {
    pub texture: Texture<'a>,
    pub x: i32,
    pub y: i32,
}