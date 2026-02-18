use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

use crate::rain::Rain;

/// Pre-rendered glyph textures (white on transparent) for each character in the set.
pub struct GlyphAtlas {
    /// Each entry: (texture, width, height)
    glyphs: Vec<(sdl2::render::Texture, u32, u32)>,
}

impl GlyphAtlas {
    pub fn new(
        font: &Font,
        chars: &[char],
        texture_creator: &TextureCreator<WindowContext>,
    ) -> Self {
        let glyphs = chars
            .iter()
            .map(|&ch| {
                let surface = font
                    .render_char(ch)
                    .blended(Color::WHITE)
                    .unwrap_or_else(|_| {
                        // Fallback: render '?' if a character fails
                        font.render_char('?').blended(Color::WHITE).unwrap()
                    });
                let w = surface.width();
                let h = surface.height();
                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .unwrap();
                (texture, w, h)
            })
            .collect();
        GlyphAtlas { glyphs }
    }
}

/// Render one frame of the rain simulation.
pub fn render_frame(
    canvas: &mut Canvas<Window>,
    rain: &Rain,
    atlas: &mut GlyphAtlas,
    cell_w: u32,
    cell_h: u32,
) {
    // Clear to black
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    for col in 0..rain.cols {
        for row in 0..rain.rows {
            let cell = &rain.grid[col][row];
            if cell.brightness <= 0.0 {
                continue;
            }

            let (r, g, b, a) = if cell.is_head {
                // Head: bright white-green
                let bright = (cell.brightness * 255.0) as u8;
                (180, 255, 180, bright)
            } else {
                // Trail: matrix green, fading
                let green = (cell.brightness * 230.0) as u8;
                let red = (cell.brightness * 30.0) as u8;
                let alpha = (cell.brightness * 255.0) as u8;
                (red, green, 0, alpha)
            };

            if let Some((ref mut texture, glyph_w, glyph_h)) =
                atlas.glyphs.get_mut(cell.char_idx)
            {
                texture.set_color_mod(r, g, b);
                texture.set_alpha_mod(a);

                // Center glyph in cell
                let x = (col as u32 * cell_w) + (cell_w.saturating_sub(*glyph_w)) / 2;
                let y = row as u32 * cell_h;
                let dst = Rect::new(x as i32, y as i32, *glyph_w, *glyph_h);
                let _ = canvas.copy(texture, None, dst);
            }
        }
    }

    canvas.present();
}
