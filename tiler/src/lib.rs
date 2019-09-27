pub use crate::frame::*;
pub use ggez::event::{KeyCode, KeyMods, quit};
use ggez::{
    graphics::{
        self,
        mint::{Point2, Vector2},
        BlendMode, DrawParam, FilterMode,
    },
    GameResult,
};
pub use tiler_derive::TileSet;

mod frame;

pub struct Context;

pub trait TileSet {
    /// Get the bitmaps representing this tileset.
    ///
    /// Currently, the bitmaps must be exactly 12*6 pixels.
    fn get_bmps() -> &'static [RgbaImage<'static>];

    /// Returns the character that represents the tile. Used for debugging.
    fn as_char(&self) -> char;

    /// Get the index in the images of this tile in the tileset.
    fn idx(&self) -> usize;
}

#[derive(Debug, Copy, Clone)]
pub struct RgbaImage<'a> {
    pub width: u16,
    pub height: u16,
    pub rgba: &'a [u8],
}

impl<'a> RgbaImage<'a> {
    pub const fn new(width: u16, height: u16, rgba: &'a [u8]) -> Self {
        RgbaImage {
            width,
            height,
            rgba,
        }
    }

    fn load_into_ggez(&self, context: &mut ggez::Context) -> GameResult<graphics::Image> {
        //println!("{:#?}", self);
        let mut image = graphics::Image::from_rgba8(context, self.width, self.height, self.rgba)?;
        //image.set_filter(FilterMode::Nearest);
        Ok(image)
    }
}

pub trait App {
    const NAME: &'static str;
    const AUTHOR: &'static str = "";
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    type TileSet: TileSet + Default;

    fn draw(&self, frame: &mut Grid<Self::TileSet>, ctx: Context);
    fn update(&mut self, ctx: Context);
    fn key_down_event(&mut self, ctx: Context, keycode: KeyCode, keymods: KeyMods, repeat: bool) {}
}

struct AppContainer<A> {
    app: A,
    images: Vec<graphics::Image>,
}

impl<A> ggez::event::EventHandler for AppContainer<A>
where
    A: App,
{
    fn update(&mut self, _: &mut ggez::Context) -> ggez::GameResult {
        self.app.update(Context);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut frame = Grid::new(A::WIDTH, A::HEIGHT);
        self.app.draw(&mut frame, Context);
        let dims = graphics::screen_coordinates(ctx);
        let stride_x = dims.w / A::WIDTH as f32;
        let stride_y = dims.h / A::HEIGHT as f32;
        let scale = Vector2 {
            x: stride_x / 20.0,
            y: stride_y / 40.0,
        };
        graphics::clear(ctx, graphics::BLACK);
        for x in 0..A::WIDTH {
            for y in 0..A::HEIGHT {
                let ch = &frame[(x, y)];
                let dest = Point2 {
                    x: x as f32 * stride_x,
                    y: y as f32 * stride_y,
                };
                //println!("{},{}: {:?}, {:?}", x,y, dest, scale);
                graphics::draw(
                    ctx,
                    &self.images[ch.idx()],
                    DrawParam::default().dest(dest).scale(scale),
                )?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        if keycode == KeyCode::Escape {
            quit(ctx);
        }
        self.app.key_down_event(Context, keycode, keymods, repeat)
    }
}

pub fn run<A>(mut app: A)
where
    A: App,
{
    let (mut ctx, mut event_loop) = ggez::ContextBuilder::new(<A as App>::NAME, <A as App>::AUTHOR)
        .build()
        .unwrap();
    //println!("{:?}", A::TileSet::get_bmps());
    let images = A::TileSet::get_bmps()
        .iter()
        .map(|image| image.load_into_ggez(&mut ctx).unwrap())
        .collect::<Vec<_>>();
    graphics::set_blend_mode(&mut ctx, BlendMode::Replace).unwrap();
    let mut app = AppContainer { app, images };
    println!("{}", graphics::renderer_info(&mut ctx).unwrap());
    ggez::event::run(&mut ctx, &mut event_loop, &mut app).unwrap();
}
