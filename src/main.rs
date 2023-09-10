use rand::Rng;
use raylib::consts::KeyboardKey::*;
use raylib::{ffi::GetTime, prelude::*};

const BACKGROUND_COLOR: Color = Color::new(173, 204, 96, 255);
const SNAKE_COLOR: Color = Color::new(43, 51, 24, 255);
const GRID_SIZE: f32 = 16.0;
const GRID_AMOUNT: f32 = 32.0;
const OFFSET: f32 = 75.0;

struct Snake {
    body: Vec<Vector2>,
    size: f32,
    velocity: Vector2,
    has_eaten: bool,
    last_velocity: Vector2,
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
            has_eaten: false,
            last_velocity: Vector2::new(1.0, 1.0),
        }
    }

    pub fn update(&mut self) {
        if !self.has_eaten {
            self.body.pop();
        } else {
            self.has_eaten = false;
        }

        if self.last_velocity.x == 1.0 && self.velocity.x == -1.0 {
            self.velocity = Vector2::new(self.last_velocity.x, self.velocity.y);
        } else if self.last_velocity.x == -1.0 && self.velocity.x == 1.0 {
            self.velocity = Vector2::new(self.last_velocity.x, self.velocity.y);
        } else if self.last_velocity.y == 1.0 && self.velocity.y == -1.0 {
            self.velocity = Vector2::new(self.velocity.x, self.last_velocity.y);
        } else if self.last_velocity.y == -1.0 && self.velocity.y == 1.0 {
            self.velocity = Vector2::new(self.velocity.x, self.last_velocity.y);
        }

        self.body.insert(0, self.body[0] + self.velocity);
        self.last_velocity = self.velocity;
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        for e in self.body.iter() {
            let rec = Rectangle::new(
                OFFSET + e.x * GRID_SIZE,
                OFFSET + e.y * GRID_SIZE,
                self.size,
                self.size,
            );

            d.draw_rectangle_rounded(rec, 0.5, 6, SNAKE_COLOR);
        }
    }

    pub fn reset(&mut self) {
        self.body = vec![
            Vector2::new(3.0, 6.0),
            Vector2::new(2.0, 6.0),
            Vector2::new(1.0, 6.0),
        ];
        self.velocity = Vector2::new(1.0, 0.0);
    }
}

struct Square {
    position: Vector2,
    // velocity: Vector2,
    size: Vector2,
    color: Color,
}

impl Square {
    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_rectangle_v(
            Vector2::new(OFFSET, OFFSET) + self.position * GRID_SIZE,
            self.size,
            self.color,
        );
    }

    pub fn respawn(&mut self) {
        let new_x = rand::thread_rng().gen_range(0, (GRID_AMOUNT) as i32) as f32;

        let new_y = rand::thread_rng().gen_range(0, (GRID_AMOUNT) as i32) as f32;

        self.position = Vector2::new(new_x, new_y);
    }
}

struct Game {
    square: Square,
    snake: Snake,
    interval: f64,
    score: i32,
}

impl Game {
    pub fn update(&mut self) {
        self.snake.update();
        self.check_collision_with_food();
        self.check_collision_with_wall();
        self.check_collision_with_body();
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        self.square.draw(d);
        self.snake.draw(d);
    }

    pub fn check_collision_with_food(&mut self) {
        if self.square.position == self.snake.body[0] {
            self.score += 1;
            self.snake.has_eaten = true;
            self.square.respawn();
            self.interval = if self.interval * 0.9 > (1.0 / GRID_AMOUNT as f64) {
                self.interval * 0.9
            } else {
                (1.0 / GRID_AMOUNT) as f64
            }
        }
    }

    pub fn check_collision_with_wall(&mut self) {
        if self.snake.body[0].x == GRID_AMOUNT || self.snake.body[0].x == -1.0 {
            self.game_over();
        }

        if self.snake.body[0].y == GRID_AMOUNT || self.snake.body[0].y == -1.0 {
            self.game_over();
        }
    }

    pub fn check_collision_with_body(&mut self) {
        let mut headless_body = self.snake.body.to_vec();
        headless_body.remove(0);

        for i in headless_body.iter() {
            if Vector2::new(i.x, i.y) == self.snake.body[0] {
                self.game_over();
            }
        }
    }

    pub fn game_over(&mut self) {
        self.snake.reset();
        self.interval = 0.2;
        self.score = 0;
        self.square.respawn();
    }
}

static mut LAST_UPDATE_TIME: f64 = 0.0;

fn interval(time: f64) -> bool {
    unsafe {
        let current_time = GetTime();
        if current_time - LAST_UPDATE_TIME < time {
            return false;
        }
        LAST_UPDATE_TIME = current_time;
        return true;
    }
}

fn main() {
    println!("Hello, world!");
    let (mut rl, thread) = raylib::init()
        .size(
            (OFFSET * 2.0 + GRID_SIZE * GRID_AMOUNT) as i32,
            (OFFSET * 2.0 + GRID_SIZE * GRID_AMOUNT) as i32,
        )
        .title("Snake Game 🦊 @Rust🦀")
        .vsync()
        .build();

    let square = Square {
        position: Vector2::new(3.0, 3.0),
        // velocity: Vector2::new(1.0, 0.0),
        size: Vector2::new(GRID_SIZE as f32, GRID_SIZE as f32),
        color: SNAKE_COLOR,
    };

    let snake = Snake::new();

    let mut game = Game {
        square,
        snake,
        interval: 0.2,
        score: 0,
    };

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

        if interval(game.interval) {
            game.update();
        }

        d.draw_text(
            format!("Score: {}", game.score).as_str(),
            (OFFSET as i32) - 5,
            20,
            40,
            SNAKE_COLOR,
        );

        // DrawRectangleLinesEx(Rectangle{ constants::OFFSET - 5, constants::OFFSET - 5, constants::CELL_SIZE * constants::CELL_AMOUNT + 10,  constants::CELL_SIZE * constants::CELL_AMOUNT + 10 }, 5, constants::SNAKE_COLOR);
        let rec = Rectangle::new(
            OFFSET - 5.0,
            OFFSET - 5.0,
            GRID_SIZE * GRID_AMOUNT + 10.0,
            GRID_SIZE * GRID_AMOUNT + 10.0,
        );

        d.draw_rectangle_lines_ex(rec, 5, SNAKE_COLOR);

        game.draw(&mut d);
    }
}
