mod map;
mod snake;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 40;
    pub const SCREEN_HEIGHT: i32 = 25;
    pub const FRAME_DURATION: f32 = 120.0;
    pub const UPGRADE_EVERY_SECOND: i32 = 10;
    pub use crate::map::*;
    pub use crate::snake::*;
}

use prelude::*;

const FONT_FILE: &str = "snake.png";

enum GameMode {
    Menu,
    Playing,
    End,
}

struct State {
    map: Map,
    snake: Snake,
    mode: GameMode,
    score: i32,
    frame_time: f32,
    total_time: f32,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            snake: Snake::new(),
            mode: GameMode::Menu,
            score: 0,
            frame_time: 0.0,
            total_time: 0.0,
        }
    }

    fn get_total_time_s(&self) -> i32 {
        self.total_time as i32 / 1000
    }

    fn restart(&mut self) {
        self.snake = Snake::new();
        self.mode = GameMode::Playing;
        self.map = Map::new();
        self.score = 0;
        self.frame_time = 0.0;
        self.total_time = 0.0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, "Welcome to Greedy Snake");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        /*
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(0);
        */
        ctx.cls();
        //ctx.cls_bg(BLACK);
        ctx.print_color_centered(5, RED, BLACK, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        let current_duration = self.get_total_time_s();
        ctx.print(
            1,
            0,
            &format!("Score: {} Time: {}s", self.score, current_duration),
        );
        self.snake.change_direction(ctx);
        self.frame_time += ctx.frame_time_ms;
        self.total_time += ctx.frame_time_ms;
        if (current_duration / UPGRADE_EVERY_SECOND) > self.map.fence_level {
            let success = self.map.upgrade_fence();
            if !success {
                self.mode = GameMode::End;
                return;
            }
        }
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            let result = self.snake.move_next(&mut self.map);
            if result < 0 {
                self.mode = GameMode::End;
                return;
            }
            self.score += result;
        }

        self.map.render(ctx);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Greedy Snake")
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font(FONT_FILE, 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, FONT_FILE)
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, FONT_FILE)
        .build()?;
    main_loop(context, State::new())
}
