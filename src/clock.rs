use crate::config;

/// 5×7 sci-fi bitmap font for digits 0-9 and colon.
/// Sharp angles, slashed zero, crossbar 7 — cyberpunk HUD style.
const DIGITS: [&[u8]; 10] = [
    // 0 — slashed zero
    &[
        0, 1, 1, 1, 0,
        1, 0, 0, 0, 1,
        1, 0, 0, 1, 1,
        1, 0, 1, 0, 1,
        1, 1, 0, 0, 1,
        1, 0, 0, 0, 1,
        0, 1, 1, 1, 0,
    ],
    // 1 — angular stem
    &[
        0, 0, 1, 0, 0,
        0, 1, 1, 0, 0,
        1, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        1, 1, 1, 1, 1,
    ],
    // 2 — sharp descender
    &[
        0, 1, 1, 1, 0,
        1, 0, 0, 0, 1,
        0, 0, 0, 0, 1,
        0, 0, 1, 1, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 1, 1, 1,
    ],
    // 3 — angular mid
    &[
        1, 1, 1, 1, 0,
        0, 0, 0, 0, 1,
        0, 0, 0, 0, 1,
        0, 1, 1, 1, 0,
        0, 0, 0, 0, 1,
        0, 0, 0, 0, 1,
        1, 1, 1, 1, 0,
    ],
    // 4 — angular build-up
    &[
        0, 0, 0, 1, 0,
        0, 0, 1, 1, 0,
        0, 1, 0, 1, 0,
        1, 0, 0, 1, 0,
        1, 1, 1, 1, 1,
        0, 0, 0, 1, 0,
        0, 0, 0, 1, 0,
    ],
    // 5 — hard edges
    &[
        1, 1, 1, 1, 1,
        1, 0, 0, 0, 0,
        1, 1, 1, 1, 0,
        0, 0, 0, 0, 1,
        0, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        0, 1, 1, 1, 0,
    ],
    // 6 — curved entry
    &[
        0, 0, 1, 1, 0,
        0, 1, 0, 0, 0,
        1, 0, 0, 0, 0,
        1, 1, 1, 1, 0,
        1, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        0, 1, 1, 1, 0,
    ],
    // 7 — crossbar
    &[
        1, 1, 1, 1, 1,
        0, 0, 0, 0, 1,
        0, 0, 0, 1, 0,
        0, 1, 1, 1, 0,
        0, 0, 1, 0, 0,
        0, 1, 0, 0, 0,
        0, 1, 0, 0, 0,
    ],
    // 8 — pinched middle
    &[
        0, 1, 1, 1, 0,
        1, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        0, 1, 1, 1, 0,
        1, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        0, 1, 1, 1, 0,
    ],
    // 9 — curved exit
    &[
        0, 1, 1, 1, 0,
        1, 0, 0, 0, 1,
        1, 0, 0, 0, 1,
        0, 1, 1, 1, 1,
        0, 0, 0, 0, 1,
        0, 0, 0, 1, 0,
        0, 1, 1, 0, 0,
    ],
];

// Colon — 1 wide, 7 tall
const COLON: &[u8] = &[
    0,
    0,
    1,
    0,
    1,
    0,
    0,
];

const DIGIT_W: usize = 5;
const DIGIT_H: usize = 7;
const COLON_W: usize = 1;
const GAP: usize = 1;

/// Clock layout result: digit cells + dark zone bounding box.
pub struct ClockLayout {
    /// Grid cells that form the digit shapes
    pub cells: Vec<(usize, usize)>,
    /// Dark zone: (col_start, row_start, col_end, row_end) — exclusive end
    pub zone: (usize, usize, usize, usize),
}

/// Compute clock layout for the current time.
pub fn clock_layout(grid_cols: usize, grid_rows: usize) -> Option<ClockLayout> {
    let now = chrono::Local::now();
    let time_str = now.format("%H:%M:%S").to_string();

    let scale = config::CLOCK_SCALE;
    let pad = config::CLOCK_MARGIN;

    // HH:MM:SS = 6 digits (5 wide) + 2 colons (1 wide) + 7 gaps
    let bitmap_w = 6 * DIGIT_W + 2 * COLON_W + 7 * GAP;
    let total_w = bitmap_w * scale;
    let total_h = DIGIT_H * scale;

    // Position: bottom-right with padding
    let origin_col = if grid_cols > total_w + config::CLOCK_PAD_X {
        grid_cols - total_w - config::CLOCK_PAD_X
    } else {
        return None;
    };
    let origin_row = if grid_rows > total_h + config::CLOCK_PAD_Y {
        grid_rows - total_h - config::CLOCK_PAD_Y
    } else {
        return None;
    };

    // Dark zone with margin around the digits
    let zone_col_start = origin_col.saturating_sub(pad);
    let zone_row_start = origin_row.saturating_sub(pad);
    let zone_col_end = (origin_col + total_w + pad).min(grid_cols);
    let zone_row_end = (origin_row + total_h + pad).min(grid_rows);

    let mut cells = Vec::new();
    let mut cursor_x: usize = 0;

    for ch in time_str.chars() {
        let (bitmap, bw) = if ch == ':' {
            (COLON as &[u8], COLON_W)
        } else if let Some(d) = ch.to_digit(10) {
            (DIGITS[d as usize] as &[u8], DIGIT_W)
        } else {
            continue;
        };

        for by in 0..DIGIT_H {
            for bx in 0..bw {
                if bitmap[by * bw + bx] == 1 {
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let col = origin_col + (cursor_x + bx) * scale + sx;
                            let row = origin_row + by * scale + sy;
                            cells.push((col, row));
                        }
                    }
                }
            }
        }

        cursor_x += bw + GAP;
    }

    Some(ClockLayout {
        cells,
        zone: (zone_col_start, zone_row_start, zone_col_end, zone_row_end),
    })
}
