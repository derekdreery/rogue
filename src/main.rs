use mint::Point2;
use std::{thread, time::Duration};
use tiler::{App, Context, Frame, TileSet};

#[derive(Debug, TileSet)]
pub enum Tiles {
    #[tileset(char = '.', fg_color = "green", default)]
    Grass,
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
    #[tileset(char = '.', fg_color = "white")]
    Floor,
}

#[derive(Default)]
struct State;

impl App for State {
    const NAME: &'static str = "rogue";
    const AUTHOR: &'static str = "dodj";
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;
    type TileSet = Tiles;

    fn draw(&self, frame: &mut Frame<Self::TileSet>, ctx: Context) {
        let room = Room::new(0, 0, 5, 5);
        room.draw(frame);
        let room = Room::new(20, 15, 28, 18);
        //room.draw(frame);
        frame[(5, 5)] = Tiles::Character;
    }
}

/// The indexes of the parameters are the walls.
pub struct Room {
    top_left: mint::Point2<usize>,
    bottom_right: mint::Point2<usize>,
}

impl Room {
    fn new(left: usize, top: usize, right: usize, bottom: usize) -> Self {
        Room {
            top_left: Point2 { x: left, y: top },
            bottom_right: Point2 {
                x: right,
                y: bottom,
            },
        }
    }

    fn draw(&self, frame: &mut Frame<Tiles>) {
        let Room {
            top_left,
            bottom_right,
        } = self;

        for col in top_left.x+1..bottom_right.x-1 {
            frame[(top_left.y, col)] = Tiles::WallEW;
            frame[(bottom_right.y - 1, col)] = Tiles::WallEW;
        }
        for row in top_left.y+1..bottom_right.y-1 {
            frame[(row, top_left.x)] = Tiles::WallNS;
            frame[(row, bottom_right.x - 1)] = Tiles::WallNS;
        }
        frame[(top_left.y, top_left.x)] = Tiles::WallNW;
        frame[(top_left.y, bottom_right.x-1)] = Tiles::WallNE;
        frame[(bottom_right.y-1, top_left.x)] = Tiles::WallSW;
        frame[(bottom_right.y-1, bottom_right.x-1)] = Tiles::WallSE;
        for col in (top_left.x + 1)..(bottom_right.x - 1) {
            for row in (top_left.y + 1)..(bottom_right.y - 1) {
                frame[(row, col)] = Tiles::Floor;
            }
        }
    }
}

pub fn main() {
    tiler::run(State);
}
