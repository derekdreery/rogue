use mint::Point2;
use rand::prelude::*;
use std::{thread, time::Duration};
use tiler::{App, Frame, TileSet};

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
    player: Point2<usize>,
}

impl State {
    pub fn new() -> Self {
        State {
            player: Point2 { x: 5, y: 5 },
        }
    }
}

impl App for State {
    const NAME: &'static str = "rogue";
    const SIZE: Point2<usize> = Point2 { x: 80, y: 30 };

    fn update(&mut self, frame: &mut Frame) {
        let room = Room::new(0, 0, 5, 7);
        room.draw(frame);
        let room = Room::new(20, 15, 28, 18);
        //room.draw(&mut frame.grid);
        frame[(self.player.x, self.player.y)] = Tiles::Character.to_char();
    }

    /*
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
    */
}

/// The indexes of the parameters are the walls.
pub struct Room(Rect);

impl Room {
    fn new(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Room(Rect::from_parts(left, top, right, bottom))
    }

    fn draw(&self, frame: &mut Frame) {
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
    type TileSet: TileSet;
    const TOP_LEFT: Self::TileSet;
    const TOP: Self::TileSet;
    const TOP_RIGHT: Self::TileSet;
    const LEFT: Self::TileSet;
    const MIDDLE: Self::TileSet;
    const RIGHT: Self::TileSet;
    const BOTTOM_LEFT: Self::TileSet;
    const BOTTOM: Self::TileSet;
    const BOTTOM_RIGHT: Self::TileSet;

    fn draw(frame: &mut Frame, rect: Rect) {
        let Rect {
            top_left,
            bottom_right,
        } = rect;

        for x in top_left.x + 1..bottom_right.x - 1 {
            frame[(x, top_left.y)] = Self::TOP.to_char();
            frame[(x, bottom_right.y - 1)] = Self::BOTTOM.to_char();
        }
        for y in top_left.y + 1..bottom_right.y - 1 {
            frame[(top_left.x, y)] = Self::LEFT.to_char();
            frame[(bottom_right.x - 1, y)] = Self::RIGHT.to_char();
        }
        frame[(top_left.x, top_left.y)] = Self::TOP_LEFT.to_char();
        frame[(bottom_right.x - 1, top_left.y)] = Self::TOP_RIGHT.to_char();
        frame[(top_left.x, bottom_right.y - 1)] = Self::BOTTOM_LEFT.to_char();
        frame[(bottom_right.x - 1, bottom_right.y - 1)] = Self::BOTTOM_RIGHT.to_char();
        for x in (top_left.x + 1)..(bottom_right.x - 1) {
            for y in (top_left.y + 1)..(bottom_right.y - 1) {
                frame[(x, y)] = Self::MIDDLE.to_char();
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
