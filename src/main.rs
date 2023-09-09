use raylib::{ffi::GetTime, prelude::*};
use raylib::consts::KeyboardKey::*;

const BACKGROUND_COLOR: Color = Color::new(173, 204, 96, 255);
const SNAKE_COLOR: Color = Color::new(43, 51, 24, 255);
const GRID_SIZE: f32 = 16.0;
const GRID_AMOUNT: f32 = 32.0;

struct Snake {
    body: Vec<Vector2>,
    size: f32,
    velocity: Vector2,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Vector2::new(3.0, 6.0),
                Vector2::new(2.0, 6.0),
                Vector2::new(1.0, 6.0),
            ],
            velocity: Vector2::new(1.0, 0.0),
            size: GRID_SIZE,
        }
    }

    pub fn update(&mut self) {
        self.body.pop();
        self.body.insert(0, self.body[0] + self.velocity);
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for e in self.body.iter() {
            let rec = Rectangle::new(e.x * GRID_SIZE, e.y * GRID_SIZE, self.size, self.size);

            d.draw_rectangle_rounded(rec, 0.5, 6, SNAKE_COLOR);
        }
    }
}

struct Square {
    position: Vector2,
    velocity: Vector2,
    size: Vector2,
    color: Color,
}

impl Square {
    pub fn update(&mut self) {
        // self.position += self.velocity * self.size;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(self.position, self.size, self.color);
    }
}

struct Game {
    square: Square,
    snake: Snake,
}

impl Game {
    pub fn update(&mut self) {
        // self.square.update();
        self.snake.update();
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.square.draw(d);
        self.snake.draw(d);
    }
}

static mut last_update_time: f64 = 0.0;

fn interval(time: f64) -> bool {
    unsafe {
        let current_time = GetTime();
        if current_time - last_update_time < time {
            return false;
        }
        last_update_time = current_time;
        return true;
    }
}

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(
            (GRID_SIZE * GRID_AMOUNT) as i32,
            (GRID_SIZE * GRID_AMOUNT) as i32,
        )
        .title("Snake Game 🦊 @Rust🦀")
        .vsync()
        .build();

    let square = Square {
        position: Vector2::new(30.0, 30.0),
        velocity: Vector2::new(1.0, 0.0),
        size: Vector2::new(GRID_SIZE as f32, GRID_SIZE as f32),
        color: SNAKE_COLOR,
    };

    let snake = Snake::new();

    let mut game = Game { square, snake };

    while !rl.window_should_close() {
        if rl.is_key_down(KEY_W) {
            game.snake.velocity = Vector2::new(0.0, -1.0);
        }

        if rl.is_key_down(KEY_S) {
            game.snake.velocity = Vector2::new(0.0, 1.0);
        }

        if rl.is_key_down(KEY_D) {
            game.snake.velocity = Vector2::new(1.0, 0.0);
        }

        if rl.is_key_down(KEY_A) {
            game.snake.velocity = Vector2::new(-1.0, 0.0);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        if interval(0.3) {
            game.update();
        }

        game.draw(&mut d);
    }
}
