use macroquad::prelude::*;
use ::rand::prelude::*;


const TILE_SIZE: f32 = 32.0;
const TILE_COUNT: i32 = 10;
const TILE_SPACING: f32 = 2.0;
const X_OFFSET: f32 = 100.0;
const Y_OFFSET: f32 = 100.0;
const MINE_PROBABILITY: f64 = 0.1;

/// Atlas is 4×3 tiles of 32×32:
/// [1][2][3][4]
/// [5][6][7][8]
/// [empty][hidden][mine][flag]
#[derive(Clone, Copy, PartialEq, Eq)]
enum TileState {
    Number(u8), // 1..=8
    Empty,
    Hidden,
    Mine,
    Flag,
}

impl TileState {
    fn atlas_offset(self) -> (f32, f32) {
        let (col, row) = match self {
            TileState::Number(n @ 1..=8) => ((n - 1) % 4, (n - 1) / 4),
            TileState::Number(_) => (0, 2),
            TileState::Empty => (0, 2),
            TileState::Hidden => (1, 2),
            TileState::Mine => (2, 2),
            TileState::Flag => (3, 2),
        };
        (col as f32 * TILE_SIZE, row as f32 * TILE_SIZE)
    }
}

struct Tile {
    x: f32,
    y: f32,
    state: TileState,
    revealed: bool,
    flagged: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameResult {
    Won,
    Lost,
}

#[macroquad::main("Minesweeper")]
async fn main() {
    let texture: Texture2D = load_texture("minesweeper.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let mut tiles = generate_board();
    let mut result: Option<GameResult> = None;

    loop {
        clear_background(BLACK);
        draw_text("PSZ-01: Minesweeper", 10.0, 25.0, 30.0, WHITE);

        if result.is_none() {
            if let Some(tile) = process_mouse_input(&mut tiles) {
                if check_loss(tile) {
                    reveal_all_mines(&mut tiles);
                    result = Some(GameResult::Lost);
                } else if check_win(&tiles) {
                    result = Some(GameResult::Won);
                }
            }
        } else if is_mouse_button_pressed(MouseButton::Left)
            || is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::Enter)
        {
            tiles = generate_board();
            result = None;
        }

        for tile in &tiles {
            draw_tile(tile, &texture);
        }

        if let Some(result) = result {
            draw_end_screen(result);
        }

        next_frame().await
    }
}

fn reveal_all_mines(tiles: &mut [Tile]) {
    for tile in tiles {
        if tile.state == TileState::Mine {
            tile.flagged = false;
            tile.revealed = true;
        }
    }
}

fn draw_end_screen(result: GameResult) {
    let (message, color) = match result {
        GameResult::Won => ("You won!", GREEN),
        GameResult::Lost => ("You lost!", RED),
    };

    let overlay = Color::new(0.0, 0.0, 0.0, 0.55);
    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), overlay);

    let msg_size = 50.0;
    let msg_width = measure_text(message, None, msg_size as u16, 1.0).width;
    draw_text(
        message,
        (screen_width() - msg_width) / 2.0,
        screen_height() / 2.0 - 10.0,
        msg_size,
        color,
    );

    let hint = "Click or press Space to play again";
    let hint_size = 24.0;
    let hint_width = measure_text(hint, None, hint_size as u16, 1.0).width;
    draw_text(
        hint,
        (screen_width() - hint_width) / 2.0,
        screen_height() / 2.0 + 30.0,
        hint_size,
        WHITE,
    );
}

fn tile_at(tiles: &[Tile], mouse_pos: (f32, f32)) -> Option<usize> {
    tiles.iter().position(|tile| {
        tile.x <= mouse_pos.0
            && tile.x + TILE_SIZE >= mouse_pos.0
            && tile.y <= mouse_pos.1
            && tile.y + TILE_SIZE >= mouse_pos.1
    })
}

fn process_mouse_input(tiles: &mut Vec<Tile>) -> Option<&mut Tile> {
    if is_mouse_button_pressed(MouseButton::Left) {
        let mouse_pos = mouse_position();
        if let Some(index) = tile_at(tiles, mouse_pos) {
            if !tiles[index].flagged {
                reveal_from(tiles, index);
            }
            return Some(&mut tiles[index]);
        }
    } else if is_mouse_button_pressed(MouseButton::Right) {
        let mouse_pos = mouse_position();
        if let Some(index) = tile_at(tiles, mouse_pos) {
            let tile = &mut tiles[index];
            if !tile.revealed {
                tile.flagged = !tile.flagged;
            }
            return Some(tile);
        }
    }
    None
}

/// Reveal a cell. Empty cells flood-fill through neighbors and also reveal
/// bordering numbered cells (classic Minesweeper behavior).
fn reveal_from(tiles: &mut [Tile], index: usize) {
    if tiles[index].revealed || tiles[index].flagged {
        return;
    }

    tiles[index].revealed = true;

    if tiles[index].state != TileState::Empty {
        return;
    }

    let col = index as i32 / TILE_COUNT;
    let row = index as i32 % TILE_COUNT;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = col + dx;
            let ny = row + dy;
            if (0..TILE_COUNT).contains(&nx) && (0..TILE_COUNT).contains(&ny) {
                reveal_from(tiles, (nx * TILE_COUNT + ny) as usize);
            }
        }
    }
}

fn check_loss(tile: &Tile) -> bool {
    if tile.revealed && tile.state == TileState::Mine {
        return true;
    }
    
    false
}

fn check_win(tiles: &[Tile]) -> bool {
    for tile in tiles {
        if !tile.revealed && tile.state != TileState::Mine {
            return false;
        }
    }
    true
}

fn generate_board() -> Vec<Tile> {
    let mut rng = ::rand::thread_rng();

    let mut mine_count = ((TILE_COUNT * TILE_COUNT) as f64 * MINE_PROBABILITY) as i32;
    let mut mine_positions = Vec::new();
    while mine_count > 0 {
        let x = rng.gen_range(0..TILE_COUNT);
        let y = rng.gen_range(0..TILE_COUNT);
        if !mine_positions.contains(&(x, y)) {
            mine_positions.push((x, y));
            mine_count -= 1;
        }
    }
    
    let mut tiles = Vec::new();
    for x in 0..TILE_COUNT {
        for y in 0..TILE_COUNT {
            let is_mine = mine_positions.contains(&(x, y));
            let state = if is_mine {
                TileState::Mine
            } else {
                match count_adjacent_mines(x, y, &mine_positions) {
                    0 => TileState::Empty,
                    n => TileState::Number(n),
                }
            };

            tiles.push(Tile {
                x: x as f32 * TILE_SIZE + x as f32 * TILE_SPACING + X_OFFSET,
                y: y as f32 * TILE_SIZE + y as f32 * TILE_SPACING + Y_OFFSET,
                state,
                revealed: false,
                flagged: false,
            });
        }
    }
    tiles
}

fn count_adjacent_mines(x: i32, y: i32, mine_positions: &[(i32, i32)]) -> u8 {
    let mut count = 0u8;
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if mine_positions.contains(&(x + dx, y + dy)) {
                count += 1;
            }
        }
    }
    count
}

fn draw_tile(tile: &Tile, texture: &Texture2D) {
    // `state` is the underlying cell; Hidden/Flag are display overlays.
    let state = if tile.flagged {
        TileState::Flag
    } else if tile.revealed {
        tile.state
    } else {
        TileState::Hidden
    };
    let (sx, sy) = state.atlas_offset();
    draw_texture_ex(
        texture,
        tile.x,
        tile.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
            source: Some(Rect::new(sx, sy, TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        },
    );
}
