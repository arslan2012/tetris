use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::tetris::Tetris;

pub fn create_texture_rect<'a>(
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    r: u8, g: u8, b: u8,
    width: u32, height: u32,
) -> Option<Texture<'a>> {
    if let Ok(mut square_texture) =
    texture_creator.create_texture_target(None, width, height) {
        canvas.with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::RGB(r, g, b));
            texture.clear();
        }).expect("Failed to color a texture");
        Some(square_texture)
    } else {
        None
    }
}

fn create_texture_from_text<'a>(
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    text: &str,
    r: u8, g: u8, b: u8,
) -> Option<Texture<'a>> {
    if let Ok(surface) = font.render(text)
        .blended(Color::RGB(r, g, b)) {
        texture_creator.create_texture_from_surface(&surface).ok()
    } else {
        None
    }
}

fn get_rect_from_text(text: &str, x: i32, y: i32) -> Option<Rect> {
    Some(Rect::new(x, y, text.len() as u32 * 20, 30))
}

pub fn display_game_information<'a>(
    tetris: &Tetris,
    canvas: &mut Canvas<Window>,
    texture_creator: &'a TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
    x: i32, y: i32,
) {
    let score_text = format!("Score: {}", tetris.score);
    let lines_sent_text = format!("Lines sent: {}", tetris.nb_lines);
    let level_text = format!("Level: {}", tetris.current_level);

    let score = create_texture_from_text(&texture_creator, &font,
                                         &score_text, 255, 255, 255)
        .expect("Cannot render text");
    let lines_sent = create_texture_from_text(&texture_creator, &font,
                                              &lines_sent_text, 255, 255, 255)
        .expect("Cannot render text");
    let level = create_texture_from_text(&texture_creator, &font,
                                         &level_text, 255, 255, 255)
        .expect("Cannot render text");

    canvas.copy(&score, None, get_rect_from_text(&score_text,
                                                 x, y))
        .expect("Couldn't copy text");
    canvas.copy(&lines_sent, None, get_rect_from_text(&score_text,
                                                      x, y + 35))
        .expect("Couldn't copy text");
    canvas.copy(&level, None, get_rect_from_text(&score_text,
                                                 x, y + 70))
        .expect("Couldn't copy text");
}