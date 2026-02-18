pub const FONT_PATH: &str =
    "/home/melik/.local/share/fonts/JetBrainsMonoNerdFontMono-Regular.ttf";
pub const FONT_SIZE: u16 = 14;

// Target frame rate
pub const TARGET_FPS: u32 = 30;

// Drop speed range (rows per tick)
pub const SPEED_MIN: f32 = 0.3;
pub const SPEED_MAX: f32 = 1.2;

// Trail length range (cells)
pub const TRAIL_MIN: usize = 8;
pub const TRAIL_MAX: usize = 50;

// Probability of spawning a new drop per column per frame
pub const SPAWN_CHANCE: f32 = 0.015;

// Probability of a trail cell mutating its character per frame
pub const MUTATE_CHANCE: f32 = 0.03;

// Grace period in ms to ignore input events after launch
pub const GRACE_PERIOD_MS: u64 = 500;

// Mouse movement threshold (pixels) to avoid jitter exits
pub const MOUSE_THRESHOLD: i32 = 10;

// Character set: half-width Katakana (U+FF66..U+FF9D) + Latin A-Z + digits + some symbols
pub fn char_set() -> Vec<char> {
    let mut chars: Vec<char> = Vec::new();
    // Half-width Katakana
    for c in '\u{FF66}'..='\u{FF9D}' {
        chars.push(c);
    }
    // Latin uppercase
    for c in 'A'..='Z' {
        chars.push(c);
    }
    // Digits
    for c in '0'..='9' {
        chars.push(c);
    }
    // Symbols
    for c in ['<', '>', '=', '+', '-', '*', ':', ';', '|', '&', '%', '$', '#', '@', '!'] {
        chars.push(c);
    }
    chars
}
