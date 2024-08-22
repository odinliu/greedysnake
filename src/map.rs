use crate::prelude::*;

const MIN_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Empty,
    Fence,
    Snake,
    Ball,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub ball_idx: usize,
    random: RandomNumberGenerator,
    pub fence_level: i32,
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

impl Map {
    pub fn new() -> Self {
        let mut tiles: Vec<TileType> = vec![TileType::Floor; MIN_TILES];
        for x in 0..SCREEN_WIDTH {
            tiles[map_idx(x, 0)] = TileType::Empty;
        }
        // 设置围墙
        for x in 0..SCREEN_WIDTH {
            tiles[map_idx(x, 1)] = TileType::Fence;
            tiles[map_idx(x, SCREEN_HEIGHT - 1)] = TileType::Fence;
        }
        for y in 2..SCREEN_HEIGHT - 1 {
            tiles[map_idx(0, y)] = TileType::Fence;
            tiles[map_idx(SCREEN_WIDTH - 1, y)] = TileType::Fence;
        }
        let mut m = Self {
            tiles,
            ball_idx: 0,
            random: RandomNumberGenerator::new(),
            fence_level: 0,
        };
        m.random_ball();
        m
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x, y);
                match self.tiles[idx] {
                    TileType::Fence => {
                        ctx.set_active_console(0);
                        ctx.set(x, y, WHITE, BLACK, 202);
                    }
                    TileType::Snake => {
                        ctx.set_active_console(0);
                        ctx.set(x, y, WHITE, BLACK, 203);
                        ctx.set_active_console(1);
                        ctx.set(x, y, WHITE, BLACK, 204);
                    }
                    TileType::Ball => {
                        ctx.set_active_console(0);
                        ctx.set(x, y, WHITE, BLACK, 203);
                        ctx.set_active_console(1);
                        ctx.set(x, y, WHITE, BLACK, 201);
                    }
                    TileType::Floor => {
                        ctx.set_active_console(0);
                        ctx.set(x, y, WHITE, BLACK, 203);
                    }
                    TileType::Empty => (),
                }
            }
        }
    }

    pub fn upgrade_fence(&mut self) -> bool {
        self.fence_level += 1;
        let mut should_random_ball = false;
        for x in 1..SCREEN_WIDTH {
            let mut idx = map_idx(x, self.fence_level + 1);
            if self.tiles[idx] == TileType::Ball {
                should_random_ball = true;
            } else if self.tiles[idx] == TileType::Snake {
                return false;
            }
            self.tiles[idx] = TileType::Fence;
            idx = map_idx(x, SCREEN_HEIGHT - 1 - self.fence_level);
            if self.tiles[idx] == TileType::Ball {
                should_random_ball = true;
            } else if self.tiles[idx] == TileType::Snake {
                return false;
            }
            self.tiles[idx] = TileType::Fence;
        }
        for y in (1 + self.fence_level)..(SCREEN_HEIGHT - 1 - self.fence_level) {
            let mut idx = map_idx(self.fence_level, y);
            if self.tiles[idx] == TileType::Ball {
                should_random_ball = true;
            } else if self.tiles[idx] == TileType::Snake {
                return false;
            }
            self.tiles[idx] = TileType::Fence;
            idx = map_idx(SCREEN_WIDTH - 1 - self.fence_level, y);
            if self.tiles[idx] == TileType::Ball {
                should_random_ball = true;
            } else if self.tiles[idx] == TileType::Snake {
                return false;
            }
            self.tiles[idx] = TileType::Fence;
        }
        if should_random_ball {
            self.random_ball();
        }
        true
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn get_tile(&self, point: Point) -> Option<TileType> {
        self.try_idx(point).map(|idx| self.tiles[idx])
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    pub fn update_tile(&mut self, point: Point, tile_type: TileType) {
        if let Some(idx) = self.try_idx(point) {
            self.tiles[idx] = tile_type;
        }
    }

    pub fn random_ball(&mut self) {
        loop {
            let idx = self.random.range(
                SCREEN_WIDTH + 1,
                SCREEN_WIDTH * SCREEN_HEIGHT - SCREEN_WIDTH - 2,
            ) as usize;
            if self.tiles[idx] == TileType::Floor {
                self.tiles[idx] = TileType::Ball;
                if self.tiles[self.ball_idx] == TileType::Ball {
                    self.tiles[self.ball_idx] = TileType::Floor;
                }
                self.ball_idx = idx;
                break;
            }
        }
    }
}
