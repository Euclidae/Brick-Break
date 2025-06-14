// My redemption from that atrocity I committed that one time with C++. 
// Brick Breaker with Rust
// TODO use space to start game.
// Update, TODO complete
// N.B be as clean as possible. Will reference this code for future rust SDL3 shenanigans.
// Shout out to this guy -> https://github.com/vhspace/sdl3-rs 

use sdl3::event::Event;
use sdl3::keyboard::{Keycode, Scancode};
use sdl3::pixels::Color;
use sdl3::rect::Rect;
use sdl3::render::Canvas;
use sdl3::video::Window;
use std::time::{Duration, Instant};

// The consts here behave like #defines. I can't beleive didn't think of this earlier
const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

const PADDLE_W: i32 = 120;
const PADDLE_H: i32 = 15;
const PADDLE_SPEED: f32 = 450.0; // pixels per second
const BALL_SIZE: i32 = 12;
const INITIAL_BALL_SPEED: f32 = 300.0;

const BRICK_W: i32 = 60;
const BRICK_H: i32 = 20;
const BRICK_PADDING: i32 = 4;
const BRICK_ROWS: usize = 6;
const BRICK_COLS: usize = 12;
const BRICK_OFFSET_Y: i32 = 80;

struct GameBall {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    stuck_to_paddle: bool, // Classic "launch from paddle" mechanic. Release it with space.
}

impl GameBall {
    fn new() -> Self {
        Self {
            pos_x: SCREEN_W as f32 / 2.0 - BALL_SIZE as f32 / 2.0,
            pos_y: SCREEN_H as f32 - 60.0,
            vel_x: 0.0,
            vel_y: 0.0,
            stuck_to_paddle: true,
        }
    }

    fn launch(&mut self) {
        if self.stuck_to_paddle {
            // Launch with slight angle for interesting gameplay
            self.vel_x = 150.0;
            self.vel_y = -INITIAL_BALL_SPEED;
            self.stuck_to_paddle = false;
        }
    }

    fn update(&mut self, dt: f32, paddle_x: f32) {
        if self.stuck_to_paddle {
            // Stick to paddle until launched
            self.pos_x = paddle_x + PADDLE_W as f32 / 2.0 - BALL_SIZE as f32 / 2.0;
            return;
        }

        // Physics update
        self.pos_x += self.vel_x * dt;
        self.pos_y += self.vel_y * dt;

        // Wall collisions
        if self.pos_x <= 0.0 {
            self.pos_x = 0.0;
            self.vel_x = -self.vel_x;
        } else if self.pos_x >= (SCREEN_W as i32 - BALL_SIZE) as f32 {
            self.pos_x = (SCREEN_W as i32 - BALL_SIZE) as f32;
            self.vel_x = -self.vel_x;
        }

        // Ceiling collision
        if self.pos_y <= 0.0 {
            self.pos_y = 0.0;
            self.vel_y = -self.vel_y;
        }
    }

    fn get_rect(&self) -> Rect {
        Rect::new(
            self.pos_x as i32,
            self.pos_y as i32,
            BALL_SIZE as u32,
            BALL_SIZE as u32,
        )
    }

    fn reset(&mut self) {
        *self = GameBall::new();
    }
}

struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    fn new() -> Self {
        Self {
            x: (SCREEN_W as i32 / 2 - PADDLE_W / 2) as f32,
            y: (SCREEN_H as i32 - PADDLE_H - 30) as f32,
        }
    }

    fn update(&mut self, dt: f32, move_left: bool, move_right: bool) {
        // Movement handling
        if move_left {
            self.x -= PADDLE_SPEED * dt;
        }
        if move_right {
            self.x += PADDLE_SPEED * dt;
        }

        // Keep paddle on screen
        self.x = self.x.max(0.0).min((SCREEN_W as i32 - PADDLE_W) as f32);
    }

    fn get_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, PADDLE_W as u32, PADDLE_H as u32)
    }

    fn handle_ball_collision(&self, ball: &mut GameBall) {
        // TODO fix sticking
        // Update fixed. See comment below/
        let paddle_rect = self.get_rect();
        let ball_rect = ball.get_rect();

        if ball_rect.has_intersection(paddle_rect) && ball.vel_y > 0.0 {
            
            let paddle_center = self.x + PADDLE_W as f32 / 2.0;
            let ball_center = ball.pos_x + BALL_SIZE as f32 / 2.0;
            let hit_offset = (ball_center - paddle_center) / (PADDLE_W as f32 / 2.0);

            ball.vel_y = -ball.vel_y.abs();
            ball.vel_x = hit_offset * 250.0; 

            
            // Prevent sticking
            ball.pos_y = self.y - BALL_SIZE as f32;
        }
    }
}

#[derive(Clone, Copy)]
struct BrickTile {
    active: bool,
    row: usize, // For color coding
}

struct BrickField {
    bricks: [[BrickTile; BRICK_COLS]; BRICK_ROWS],
    active_count: usize,
}

impl BrickField {
    fn new() -> Self {
        let mut field = Self {
            bricks: [[BrickTile {
                active: false,
                row: 0
            }; BRICK_COLS]; BRICK_ROWS],
            active_count: BRICK_ROWS * BRICK_COLS,
        };

        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                field.bricks[row][col] = BrickTile {
                    active: true,
                    row,
                };
            }
        }

        field
    }

    fn get_brick_rect(&self, row: usize, col: usize) -> Rect {
        // Center brick field horizontally
        let total_width = BRICK_COLS as i32 * (BRICK_W + BRICK_PADDING) - BRICK_PADDING;
        let start_x = (SCREEN_W as i32 - total_width) / 2;

        Rect::new(
            start_x + col as i32 * (BRICK_W + BRICK_PADDING),
            BRICK_OFFSET_Y + row as i32 * (BRICK_H + BRICK_PADDING),
            BRICK_W as u32,
            BRICK_H as u32,
        )
    }

    fn check_collision(&mut self, ball: &mut GameBall) -> Option<u32> {
        let ball_rect = ball.get_rect();

        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                if !self.bricks[row][col].active {
                    continue;
                }

                let brick_rect = self.get_brick_rect(row, col);
                if ball_rect.has_intersection(brick_rect) {
                    // Deactivate brick and update count
                    self.bricks[row][col].active = false;
                    self.active_count -= 1;

                    // Simple bounce physics
                    ball.vel_y = -ball.vel_y;

                    // Higher rows = more points
                    return Some((BRICK_ROWS - row) as u32 * 10);
                }
            }
        }
        None
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                if !self.bricks[row][col].active {
                    continue;
                }

                // Row-based color scheme. Probably should have used Hex colors.
                let color = match row {
                    0 => Color::RGB(255, 100, 100), 
                    1 => Color::RGB(255, 165, 0),   
                    2 => Color::RGB(255, 255, 100), 
                    3 => Color::RGB(100, 255, 100), 
                    4 => Color::RGB(100, 100, 255),
                    _ => Color::RGB(255, 100, 255), 
                };

                canvas.set_draw_color(color);
                canvas.fill_rect(self.get_brick_rect(row, col))?;
            }
        }
        Ok(())
    }

    fn all_destroyed(&self) -> bool {
        self.active_count == 0
    }
}

struct GameState {
    ball: GameBall,
    paddle: Paddle,
    bricks: BrickField,
    score: u32,
    lives: i32,
    game_over: bool,
    victory: bool,
    last_frame_time: Instant,
}

impl GameState {
    fn new() -> Self {
        Self {
            ball: GameBall::new(),
            paddle: Paddle::new(),
            bricks: BrickField::new(),
            score: 0,
            lives: 3, // Classic arcade style
            game_over: false,
            victory: false,
            last_frame_time: Instant::now(),
        }
    }

    fn update(&mut self, left_pressed: bool, right_pressed: bool) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        if self.game_over || self.victory {
            return;
        }

        // Update game objects
        self.paddle.update(dt, left_pressed, right_pressed);
        self.ball.update(dt, self.paddle.x);

        self.paddle.handle_ball_collision(&mut self.ball);
        if let Some(points) = self.bricks.check_collision(&mut self.ball) {
            self.score += points;
        }

        // Ball lost - lose life
        if self.ball.pos_y > SCREEN_H as f32 {
            self.lives -= 1;
            if self.lives <= 0 {
                self.game_over = true;
            } else {
                self.ball.reset(); 
            }
        }

        if self.bricks.all_destroyed() {
            self.victory = true;
        }
    }

    fn render(&self, canvas: &mut Canvas<Window>) -> Result<(), Box<dyn std::error::Error>> {
        
        canvas.set_draw_color(Color::RGB(20, 20, 40));
        canvas.clear();

        // Render game objects
        self.bricks.render(canvas)?;

        canvas.set_draw_color(Color::RGB(200, 200, 220));
        canvas.fill_rect(self.paddle.get_rect())?;

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(self.ball.get_rect())?;

        canvas.present();
        Ok(())
    }

    fn handle_space_press(&mut self) {
        if self.ball.stuck_to_paddle {
            self.ball.launch();
        } else if self.game_over || self.victory {
            // Full game reset
            *self = GameState::new();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create window with explicit u32 dimensions
    let window = video_subsystem
        .window("Brick Breaker", SCREEN_W, SCREEN_H)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Create canvas
    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump()?;

    let mut game = GameState::new();
    let mut frame_counter = 0;
    let mut last_fps_update = Instant::now();

    'game_loop: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => game.handle_space_press(),
                _ => {}
            }
        }

        // Get keyboard state directly from event pump
        let keyboard_state = event_pump.keyboard_state();
        let left_pressed = keyboard_state.is_scancode_pressed(Scancode::Left)
            || keyboard_state.is_scancode_pressed(Scancode::A);
        let right_pressed = keyboard_state.is_scancode_pressed(Scancode::Right)
            || keyboard_state.is_scancode_pressed(Scancode::D);

        // Update game state
        game.update(left_pressed, right_pressed);
        game.render(&mut canvas)?;

        // Simple FPS counter
        frame_counter += 1;
        if last_fps_update.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frame_counter);
            frame_counter = 0;
            last_fps_update = Instant::now();
        }

        // Status messages
        if game.game_over {
            println!("GAME OVER! Final Score: {}", game.score);
        } else if game.victory {
            println!("VICTORY! Score: {}", game.score);
        }

        // Maintain 60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}