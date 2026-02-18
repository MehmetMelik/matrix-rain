mod config;
mod rain;
mod renderer;

use std::time::{Duration, Instant};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().expect("Failed to init SDL2");
    let video = sdl_context.video().expect("Failed to init video");
    let ttf_context = sdl2::ttf::init().expect("Failed to init SDL2_ttf");

    // Create fullscreen desktop window
    let window = video
        .window("Matrix Rain", 0, 0)
        .fullscreen_desktop()
        .build()
        .expect("Failed to create window");

    // Use software renderer (no GPU acceleration needed)
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .expect("Failed to create canvas");

    // Hide cursor
    sdl_context.mouse().show_cursor(false);

    let (screen_w, screen_h) = canvas.output_size().unwrap();

    // Load font
    let font = ttf_context
        .load_font(config::FONT_PATH, config::FONT_SIZE)
        .expect("Failed to load font");

    // Calculate grid dimensions from font metrics
    let cell_w = font.size_of_char('W').unwrap().0;
    let cell_h = font.height() as u32;
    let grid_cols = (screen_w / cell_w) as usize;
    let grid_rows = (screen_h / cell_h) as usize;

    // Build character set and glyph atlas
    let chars = config::char_set();
    let texture_creator = canvas.texture_creator();
    let mut atlas = renderer::GlyphAtlas::new(&font, &chars, &texture_creator);

    // Initialize rain simulation
    let mut rain_sim = rain::Rain::new(grid_cols, grid_rows, chars.len());

    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");
    let frame_duration = Duration::from_secs(1) / config::TARGET_FPS;
    let start_time = Instant::now();

    // Track initial mouse position for movement threshold
    let mouse_state = event_pump.mouse_state();
    let (init_mx, init_my) = (mouse_state.x(), mouse_state.y());

    'running: loop {
        let frame_start = Instant::now();
        let elapsed = start_time.elapsed();
        let in_grace = elapsed < Duration::from_millis(config::GRACE_PERIOD_MS);

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } if !in_grace => break 'running,
                Event::KeyDown { .. } if !in_grace => break 'running,
                Event::MouseButtonDown { .. } if !in_grace => break 'running,
                Event::MouseMotion { x, y, .. } if !in_grace => {
                    let dx = (x - init_mx).abs();
                    let dy = (y - init_my).abs();
                    if dx > config::MOUSE_THRESHOLD || dy > config::MOUSE_THRESHOLD {
                        break 'running;
                    }
                }
                _ => {}
            }
        }

        // Update simulation
        rain_sim.update();

        // Render
        renderer::render_frame(&mut canvas, &rain_sim, &mut atlas, cell_w, cell_h);

        // Frame pacing
        let frame_elapsed = frame_start.elapsed();
        if frame_elapsed < frame_duration {
            std::thread::sleep(frame_duration - frame_elapsed);
        }
    }
}
