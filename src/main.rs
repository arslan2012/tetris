mod create_texture;
mod file_io;
mod tetrimino;
mod tetris;
mod event;
mod texture_group;

use create_texture::{create_texture_rect, create_tetrimino_texture, display_game_information};
use tetris::{Tetris, is_time_over};
use tetrimino::create_new_tetrimino;
use file_io::{save_highscores_and_lines, load_highscores_and_lines};
use event::handle_events;

extern crate sdl2;

use sdl2::pixels::Color;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use sdl2::render::{TextureCreator};
use sdl2::image::{LoadTexture, InitFlag as ImageFlag};
use sdl2::mixer::{
    Music,
    open_audio,
    InitFlag as MixerFlag,
    DEFAULT_FREQUENCY,
    DEFAULT_FORMAT,
    DEFAULT_CHANNELS,
};
use crate::texture_group::TextureGroup;

const TETRIS_HEIGHT: u32 = 40;
const NB_HIGHSCORES: usize = 5;
const WINDOW_WIDTH: u32 = 1600;
const WINDOW_HEIGHT: u32 = 900;
const HOLD_X: u32 = 80;
const BLOCK_Y: i32 = 300;
const BLOCK_WIDTH: u32 = TETRIS_HEIGHT * 4 + 20;
const ARENA_X: u32 = HOLD_X + BLOCK_WIDTH + 190;
const ARENA_WIDTH: u32 = TETRIS_HEIGHT * 10;
const ARENA_HEIGHT: u32 = TETRIS_HEIGHT * 16;
const NEXT_X: u32 = ARENA_X + ARENA_WIDTH + 10;


fn print_game_information(tetris: &Tetris) {
    let mut new_highest_highscore = true;
    let mut new_highest_lines_sent = true;
    if let Some((mut highscores, mut lines_sent)) = load_highscores_and_lines() {
        new_highest_highscore = update_vec(&mut highscores, tetris.score);
        new_highest_lines_sent = update_vec(&mut lines_sent, tetris.nb_lines);
        if new_highest_highscore || new_highest_lines_sent {
            save_highscores_and_lines(&highscores, &lines_sent);
        }
    } else {
        save_highscores_and_lines(&[tetris.score], &[tetris.nb_lines]);
    }
    println!("Game over...");
    println!("Score:           {}{}",
             tetris.score,
             if new_highest_highscore { " [NEW HIGHSCORE]" } else { "" });
    println!("Number of lines: {}{}",
             tetris.nb_lines,
             if new_highest_lines_sent { " [NEW HIGHSCORE]" } else { "" });
    println!("Current level:   {}", tetris.current_level);
}

fn update_vec(v: &mut Vec<u32>, value: u32) -> bool {
    if v.len() < NB_HIGHSCORES {
        v.push(value);
        v.sort();
        true
    } else {
        for entry in v.iter_mut() {
            if value > *entry {
                *entry = value;
                return true;
            }
        }
        false
    }
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context.video().expect("Couldn't get SDL video subsystem");
    let mut timer = SystemTime::now();
    let mut event_pump = sdl_context.event_pump().expect("Failed to get SDL event pump");

    let grid_x = ARENA_X as i32;
    let grid_y = (WINDOW_HEIGHT - ARENA_HEIGHT) as i32 / 2;
    let mut tetris = Tetris::new();

    let window =
        video_subsystem
            .window("Tetris", WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .build()
            .expect("Failed to create window");

    let mut canvas =
        window
            .into_canvas()
            .target_texture()
            .present_vsync()
            .build()
            .expect("Couldn't get window's canvas");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    sdl2::image::init(ImageFlag::JPG).expect("Couldn't initialize image context");

    let image_texture =
        texture_creator
            .load_texture("assets/wallhaven-ymoo2x.jpg")
            .expect("Couldn't load image");

    sdl2::mixer::init(MixerFlag::OGG).expect("Couldn't initialize mixer context");
    open_audio(
        DEFAULT_FREQUENCY,
        DEFAULT_FORMAT,
        DEFAULT_CHANNELS,
        256,
    ).expect("Couldn't open audio");
    let music = Music::from_file("assets/theme.ogg").expect("Couldn't load theme song");
    music.play(-1).expect("Couldn't play theme song");

    let mut arena = TextureGroup::new();
    arena.add(create_texture_rect(
        &mut canvas,
        &texture_creator,
        255, 255, 255,
        ARENA_WIDTH + 20,
        ARENA_HEIGHT + 20,
    ).expect("Failed to create a texture"), -10, -10);
    arena.add(create_texture_rect(
        &mut canvas,
        &texture_creator,
        0, 0, 0,
        ARENA_WIDTH,
        ARENA_HEIGHT,
    ).expect("Failed to create a texture"), 0, 0);

    let mut small_preview_area = TextureGroup::new();
    small_preview_area.add(create_texture_rect(
        &mut canvas,
        &texture_creator,
        255, 255, 255,
        BLOCK_WIDTH + 20,
        BLOCK_WIDTH + 20,
    ).expect("Failed to create a texture"), -10, -10);

    small_preview_area.add(create_texture_rect(
        &mut canvas,
        &texture_creator,
        0, 0, 0,
        BLOCK_WIDTH,
        BLOCK_WIDTH,
    ).expect("Failed to create a texture"), 0, 0);

    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed");
    let font = ttf_context.load_font(
        "assets/JetBrainsMonoNL-Regular.ttf",
        128,
    ).expect("Couldn't load the font");

    macro_rules! texture {
        ($r:expr, $g:expr, $b:expr) => (
            create_tetrimino_texture(
                &mut canvas,
                &texture_creator,
                $r, $g, $b
            )
        )
      }

    let colour_of_piece = [
        (255, 69, 69), (255, 220, 69),
        (237, 150, 37), (171, 99, 237),
        (77, 149, 239), (39, 218, 225),
        (45, 216, 47)
    ];

    let textures: Vec<TextureGroup> = colour_of_piece
        .iter()
        .map(|c| texture!(c.0, c.1, c.2))
        .collect();

    let textures_alpha: Vec<TextureGroup> = colour_of_piece
        .iter()
        .map(|c| {
            let mut t = texture!(c.0, c.1, c.2);
            t.set_alpha(70);
            t
        })
        .collect();

    loop {
        if is_time_over(&tetris, &timer) {
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetris.game_map,
                                                        x, y);
            }
            if make_permanent {
                tetris.make_permanent();
            }
            timer = SystemTime::now();
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        //background
        canvas.copy(&image_texture, None, None).expect("Render failed");

        //hold piece
        small_preview_area.copy_to_canvas(&mut canvas, HOLD_X as i32, BLOCK_Y);
        //arena
        arena.copy_to_canvas(&mut canvas, ARENA_X as i32, (WINDOW_HEIGHT - ARENA_HEIGHT) as i32 / 2);
        //next piece
        small_preview_area.copy_to_canvas(&mut canvas, NEXT_X as i32, BLOCK_Y);

        if tetris.current_piece.is_none() {
            if !&tetris.next_piece.test_current_position(&tetris.game_map) {
                print_game_information(&tetris);
                break;
            }
            tetris.current_piece = Some(tetris.next_piece);
            tetris.next_piece = create_new_tetrimino();
        }
        let mut quit = false;

        // current piece
        if !handle_events(&mut tetris, &mut quit, &mut timer,
                          &mut event_pump) {
            if let Some(ref mut piece) = tetris.current_piece {
                for (line_nb, line) in piece.states[piece.current_state
                    as usize].iter().enumerate() {
                    for (case_nb, case) in line.iter().enumerate() {
                        if *case == 0 {
                            continue;
                        }
                        textures[*case as usize - 1].copy_to_canvas(
                            &mut canvas,
                            grid_x + (piece.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                            grid_y + (piece.y + line_nb) as i32 * TETRIS_HEIGHT as i32);
                    }
                }
            }
        }
        if quit {
            print_game_information(&tetris);
            break;
        }
        // hold_piece
        if let Some(ref mut piece) = tetris.holding_piece {
            for (line_nb, line) in piece.states[piece.current_state
                as usize].iter().enumerate() {
                for (case_nb, case) in line.iter().enumerate() {
                    if *case == 0 {
                        continue;
                    }
                    textures[*case as usize - 1].copy_to_canvas(
                        &mut canvas,
                        HOLD_X as i32 + 20 + case_nb as i32 * TETRIS_HEIGHT as i32,
                        BLOCK_Y + 20 + line_nb as i32 * TETRIS_HEIGHT as i32);
                }
            }
        }

        // next_piece
        for (line_nb, line) in tetris.next_piece.states[tetris.next_piece.current_state
            as usize].iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue;
                }
                textures[*case as usize - 1].copy_to_canvas(
                    &mut canvas,
                    NEXT_X as i32 + 20 + case_nb as i32 * TETRIS_HEIGHT as i32,
                    BLOCK_Y + 20 + line_nb as i32 * TETRIS_HEIGHT as i32);
            }
        }
        // ghost
        if let Some(ref current_piece) = tetris.current_piece {
            let mut piece = current_piece.clone();
            let mut y = piece.y;
            while piece.change_position(&tetris.game_map, piece.x, y + 1)
                == true {
                y += 1;
            }
            for (line_nb, line) in piece.states[piece.current_state
                as usize].iter().enumerate() {
                for (case_nb, case) in line.iter().enumerate() {
                    if *case == 0 {
                        continue;
                    }
                    textures_alpha[*case as usize - 1].copy_to_canvas(
                        &mut canvas,
                        grid_x + (piece.x + case_nb as isize) as i32 * TETRIS_HEIGHT as i32,
                        grid_y + (piece.y + line_nb) as i32 * TETRIS_HEIGHT as i32);
                }
            }
        }

        // fallen pieces
        for (line_nb, line) in tetris.game_map.iter().enumerate() {
            for (case_nb, case) in line.iter().enumerate() {
                if *case == 0 {
                    continue;
                }
                textures[*case as usize - 1].copy_to_canvas(
                    &mut canvas,
                    grid_x + case_nb as i32 * TETRIS_HEIGHT as i32,
                    grid_y + line_nb as i32 * TETRIS_HEIGHT as i32);
            }
        }
        display_game_information(&tetris, &mut canvas, &texture_creator, &font,
                                 NEXT_X as i32, BLOCK_Y - 120);
        canvas.present();

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}