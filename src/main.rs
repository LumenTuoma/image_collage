use std::{fs, io};

extern crate sdl2;

use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use sdl2::image::{LoadTexture, InitFlag};

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Load JPG images.
    let _ = sdl2::image::init(InitFlag::JPG);

    let window = video_subsystem.window("Image Collage", 1000, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().software().build().unwrap();

    let paths = fs::read_dir("images/").unwrap();

    let names: Vec<String> =
        paths.filter_map(|entry: Result<fs::DirEntry, io::Error>| {
        entry.ok().and_then(|e: fs::DirEntry| e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s))))})
        .collect::<Vec<String>>();
    
    let texture_creator = canvas.texture_creator();

    let mut loaded_images = Vec::<Texture>::new();
    for name in names
    {
        // If the file is not image, skip it.
        if !name.ends_with(".jpg") { continue; }
        // If the file is image, push it to our Texture array.
        loaded_images.push(texture_creator.load_texture(format!("images/{}", name)).unwrap());
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Go through every image.
        i = (i + 1) % loaded_images.len();

        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        canvas.clear();

        let image = loaded_images.get(i);
        if !image.is_none()
        {
            // Copy the image to the current canvas.
            canvas.copy(image.unwrap(), None, None).unwrap();
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(2, 0));
    }
}
