use crate::prelude::*;
use std::collections::VecDeque;

enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: VecDeque<Point>,
    direction: SnakeDirection,
}

impl Snake {
    pub fn new() -> Self {
        let base_x = SCREEN_WIDTH / 2;
        let base_y = SCREEN_HEIGHT / 2;
        let body = VecDeque::from([
            Point::new(base_x - 2, base_y),
            Point::new(base_x - 1, base_y),
            Point::new(base_x, base_y),
        ]);
        Self {
            direction: SnakeDirection::Right,
            body,
        }
    }

    pub fn render_on_map(&self, map: &mut Map, tail: Option<Point>) {
        for point in &self.body {
            map.update_tile(*point, TileType::Snake);
        }
        if let Some(p) = tail {
            map.update_tile(p, TileType::Floor);
        }
    }

    pub fn change_direction(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Left | VirtualKeyCode::A => {
                    self.direction = SnakeDirection::Left;
                }
                VirtualKeyCode::Right | VirtualKeyCode::D => {
                    self.direction = SnakeDirection::Right;
                }
                VirtualKeyCode::Up | VirtualKeyCode::W => {
                    self.direction = SnakeDirection::Up;
                }
                VirtualKeyCode::Down | VirtualKeyCode::S => {
                    self.direction = SnakeDirection::Down;
                }
                _ => (),
            };
        }
    }

    pub fn move_next(&mut self, map: &mut Map) -> i32 {
        let delta = match self.direction {
            SnakeDirection::Up => Point::new(0, -1),
            SnakeDirection::Down => Point::new(0, 1),
            SnakeDirection::Left => Point::new(-1, 0),
            SnakeDirection::Right => Point::new(1, 0),
        };
        if let Some(head) = self.body.back() {
            let next_move = *head + delta;
            //println!("next move: {:?}", next_move);
            match map.get_tile(next_move) {
                Some(TileType::Fence) => {
                    return -1;
                }
                Some(TileType::Snake) => {
                    return -1;
                }
                Some(TileType::Ball) => {
                    self.body.push_back(next_move);
                    self.render_on_map(map, None);
                    map.random_ball();
                    return 1;
                }
                Some(TileType::Floor) => {
                    let tail = self.body.pop_front();
                    self.body.push_back(next_move);
                    self.render_on_map(map, tail);
                    return 0;
                }
                _ => {
                    return -1;
                }
            }
        }
        -1
    }
}
