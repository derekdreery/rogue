use mint::Point2;
use rand::prelude::*;
use std::{thread, time::Duration};
use tiler::{App, Context, Grid, TileSet, KeyMods, KeyCode};

#[derive(Debug, Copy, Clone, TileSet)]
pub enum Tiles {
    #[tileset(char = '·', fg_color = "green", default)]
    Grass,
    #[tileset(char = '·', fg_color = "lightgreen")]
    LightGrass,
    #[tileset(char = '☺')]
    Character,
    #[tileset(char = '║', fg_color = "gray")]
    WallNS,
    #[tileset(char = '═', fg_color = "gray")]
    WallEW,
    #[tileset(char = '╔', fg_color = "gray")]
    WallNW,
    #[tileset(char = '╗', fg_color = "gray")]
    WallNE,
    #[tileset(char = '╚', fg_color = "gray")]
    WallSW,
    #[tileset(char = '╝', fg_color = "gray")]
    WallSE,
    #[tileset(char = '·', fg_color = "white")]
    Floor,
}

struct State {
    world: Grid<Tiles>,
    player: Point2<usize>,
}

impl State {
    pub fn new() -> Self {
        State {
            world: Grid::from_fn(<Self as App>::WIDTH, <Self as App>::HEIGHT, |x, y| {
                *(&[Tiles::Grass, Tiles::LightGrass][..]).choose(&mut rand::thread_rng()).unwrap()
            }),
            player: Point2 { x: 5, y: 5 },
        }
    }
}

impl App for State {
    const NAME: &'static str = "rogue";
    const AUTHOR: &'static str = "dodj";
    const WIDTH: usize = 80;
    const HEIGHT: usize = 30;
    type TileSet = Tiles;

    fn draw(&self, frame: &mut Grid<Self::TileSet>, ctx: Context) {
        frame.clone_from(&self.world);
        let room = Room::new(0, 0, 5, 7);
        room.draw(frame);
        let room = Room::new(20, 15, 28, 18);
        //room.draw(frame);
        frame[(self.player.x, self.player.y)] = Tiles::Character;
    }

    fn update(&mut self, ctx: Context) {}

    fn key_down_event(&mut self, ctx: Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {
        match keycode {
            KeyCode::Left => {
                if self.player.x > 0 {
                    self.player.x -= 1;
                }
            }
            KeyCode::Up => {
                if self.player.y > 0 {
                    self.player.y -= 1;
                }
            }
            KeyCode::Right => {
                if self.player.x < 79 {
                    self.player.x += 1;
                }
            }
            KeyCode::Down => {
                if self.player.y < 29 {
                    self.player.y += 1;
                }
            }
            _ => (), // ignore
        }
    }
}

/// The indexes of the parameters are the walls.
pub struct Room(Rect);

impl Room {
    fn new(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Room(Rect::from_parts(left, top, right, bottom))
    }

    fn draw(&self, frame: &mut Grid<Tiles>) {
        <Self as NinePatch>::draw(frame, self.0)
    }
}

impl NinePatch for Room {
    type TileSet = Tiles;
    const TOP_LEFT: Self::TileSet = Tiles::WallNW;
    const TOP: Self::TileSet = Tiles::WallEW;
    const TOP_RIGHT: Self::TileSet = Tiles::WallNE;
    const LEFT: Self::TileSet = Tiles::WallNS;
    const MIDDLE: Self::TileSet = Tiles::Floor;
    const RIGHT: Self::TileSet = Tiles::WallNS;
    const BOTTOM_LEFT: Self::TileSet = Tiles::WallSW;
    const BOTTOM: Self::TileSet = Tiles::WallEW;
    const BOTTOM_RIGHT: Self::TileSet = Tiles::WallSE;
}

pub trait NinePatch {
    type TileSet;
    const TOP_LEFT: Self::TileSet;
    const TOP: Self::TileSet;
    const TOP_RIGHT: Self::TileSet;
    const LEFT: Self::TileSet;
    const MIDDLE: Self::TileSet;
    const RIGHT: Self::TileSet;
    const BOTTOM_LEFT: Self::TileSet;
    const BOTTOM: Self::TileSet;
    const BOTTOM_RIGHT: Self::TileSet;

    fn draw(frame: &mut Grid<Self::TileSet>, rect: Rect) {
        let Rect {
            top_left,
            bottom_right,
        } = rect;

        for x in top_left.x + 1..bottom_right.x - 1 {
            frame[(x, top_left.y)] = Self::TOP;
            frame[(x, bottom_right.y - 1)] = Self::BOTTOM;
        }
        for y in top_left.y + 1..bottom_right.y - 1 {
            frame[(top_left.x, y)] = Self::LEFT;
            frame[(bottom_right.x - 1, y)] = Self::RIGHT;
        }
        frame[(top_left.x, top_left.y)] = Self::TOP_LEFT;
        frame[(bottom_right.x - 1, top_left.y)] = Self::TOP_RIGHT;
        frame[(top_left.x, bottom_right.y - 1)] = Self::BOTTOM_LEFT;
        frame[(bottom_right.x - 1, bottom_right.y - 1)] = Self::BOTTOM_RIGHT;
        for x in (top_left.x + 1)..(bottom_right.x - 1) {
            for y in (top_left.y + 1)..(bottom_right.y - 1) {
                frame[(x, y)] = Self::MIDDLE;
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Rect {
    top_left: Point2<usize>,
    bottom_right: Point2<usize>,
}

impl Rect {
    fn from_parts(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Self {
            top_left: Point2 { x: left, y: top },
            bottom_right: Point2 {
                x: right,
                y: bottom,
            },
        }
    }
}

pub fn main() {
    tiler::run(State::new());
}
