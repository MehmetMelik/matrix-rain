use rand::Rng;

use crate::config;

/// A single cell in the character grid.
#[derive(Clone)]
pub struct CharCell {
    /// Index into the character set (rendered via atlas)
    pub char_idx: usize,
    /// Brightness 0.0 (invisible) to 1.0 (head, full bright)
    pub brightness: f32,
    /// Whether this cell is the head of a drop (rendered white-green)
    pub is_head: bool,
}

impl Default for CharCell {
    fn default() -> Self {
        Self {
            char_idx: 0,
            brightness: 0.0,
            is_head: false,
        }
    }
}

/// A single falling stream.
struct Drop {
    /// Fractional row position of the head
    y: f32,
    /// Speed in rows per frame
    speed: f32,
    /// How many cells trail behind the head
    trail_len: usize,
    /// Character indices for the head + trail (pre-picked, mutated over time)
    chars: Vec<usize>,
}

/// Manages all drops for a single column.
struct Column {
    drops: Vec<Drop>,
}

/// The full rain simulation grid.
pub struct Rain {
    pub cols: usize,
    pub rows: usize,
    pub grid: Vec<Vec<CharCell>>,
    columns: Vec<Column>,
    char_count: usize,
}

impl Rain {
    pub fn new(cols: usize, rows: usize, char_count: usize) -> Self {
        let grid = vec![vec![CharCell::default(); rows]; cols];
        let columns = (0..cols).map(|_| Column { drops: Vec::new() }).collect();
        Self {
            cols,
            rows,
            grid,
            columns,
            char_count,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // Clear the grid
        for col in &mut self.grid {
            for cell in col.iter_mut() {
                cell.brightness = 0.0;
                cell.is_head = false;
            }
        }

        for col_idx in 0..self.cols {
            let column = &mut self.columns[col_idx];

            // Maybe spawn a new drop at the top
            if rng.gen::<f32>() < config::SPAWN_CHANCE {
                let speed =
                    rng.gen::<f32>() * (config::SPEED_MAX - config::SPEED_MIN) + config::SPEED_MIN;
                let trail_len = rng.gen_range(config::TRAIL_MIN..=config::TRAIL_MAX);
                let chars: Vec<usize> = (0..trail_len + 1)
                    .map(|_| rng.gen_range(0..self.char_count))
                    .collect();
                column.drops.push(Drop {
                    y: 0.0,
                    speed,
                    trail_len,
                    chars,
                });
            }

            // Update each drop
            for drop in &mut column.drops {
                drop.y += drop.speed;

                // Mutate some trail characters for flicker effect
                for ch in &mut drop.chars {
                    if rng.gen::<f32>() < config::MUTATE_CHANCE {
                        *ch = rng.gen_range(0..self.char_count);
                    }
                }
            }

            // Remove drops that have fully scrolled off screen
            column.drops.retain(|drop| {
                let tail_y = drop.y as isize - drop.trail_len as isize;
                tail_y < self.rows as isize
            });

            // Write drops into grid
            let grid_col = &mut self.grid[col_idx];
            for drop in &column.drops {
                let head_row = drop.y as isize;

                for i in 0..=drop.trail_len {
                    let row = head_row - i as isize;
                    if row < 0 || row >= self.rows as isize {
                        continue;
                    }
                    let row = row as usize;

                    let brightness = if i == 0 {
                        1.0
                    } else {
                        // Linear fade from 0.9 down to ~0.05 over the trail
                        let t = i as f32 / drop.trail_len as f32;
                        (1.0 - t) * 0.9 + 0.05
                    };

                    // Use the brighter value if overlapping
                    if brightness > grid_col[row].brightness {
                        grid_col[row].brightness = brightness;
                        grid_col[row].char_idx = drop.chars[i % drop.chars.len()];
                        grid_col[row].is_head = i == 0;
                    }
                }
            }
        }
    }
}
