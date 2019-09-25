use mint::Point2;
use std::{thread, time::Duration};
use tiler::{App, Context, Frame, TileSet};

#[derive(Debug, TileSet)]
pub enum Tiles {
    #[tileset(char = ' ', default)]
    Empty,
    #[tileset(char = '☺')]
    Character,
    #[tileset(char = ' ', inverted)]
    Wall,
    #[tileset(char = '.', fg_color = "grey")]
    Floor,
}

#[derive(Default)]
struct State;

impl App for State {
    const NAME: &'static str = "rogue";
    const AUTHOR: &'static str = "dodj";
    const WIDTH: usize = 100;
    const HEIGHT: usize = 50;
    //const SIZE: Dims = Dims::new(80, 25);
    type TileSet = Tiles;

    fn draw(&self, frame: &mut Frame<Self::TileSet>, ctx: Context) {
        let room = Room::new(0, 0, 10, 10);
        room.draw(frame);
        let room = Room::new(20, 15, 28, 18);
        room.draw(frame);
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

        for col in top_left.x..bottom_right.x {
            frame[(top_left.y, col)] = Tiles::Wall;
            frame[(bottom_right.y - 1, col)] = Tiles::Wall;
        }
        for row in top_left.y..bottom_right.y {
            frame[(row, top_left.x)] = Tiles::Wall;
            frame[(row, bottom_right.x - 1)] = Tiles::Wall;
        }
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
