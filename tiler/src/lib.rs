use mint::Point2;
use raw_window_handle::HasRawWindowHandle;
use std::ops::{Index, IndexMut};
use std::{
    f32,
    time::{Duration, Instant},
};
use wgpu_glyph::{GlyphBrushBuilder, Scale, Section};
pub use winit;
pub use winit::event::VirtualKeyCode as KeyCode;
use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::WindowBuilder,
};

pub use tiler_derive::TileSet;

pub trait TileSet {
    fn to_char(&self) -> Char;
}

const FULL_BLOCK: char = '█';
const FONT: &'static [u8] = include_bytes!("../source_code_pro.ttf");

pub trait App {
    const NAME: &'static str;
    const SIZE: Point2<usize>;

    /// Update state and draw to the supplied frame.
    fn update(&mut self, frame: &mut Frame);
    fn key_down_event(&mut self, mut ctx: Context<'_>, keycode: KeyCode) {
        match keycode {
            KeyCode::Escape => ctx.exit(),
            _ => (),
        };
    }
    //#[allow(unused_variables)]
    fn key_up_event(&mut self, keycode: KeyCode) {}
}

struct AppContainer<A>
where
    A: App,
{
    app: A,
    frame_buf: Frame,
}

pub struct Context<'a> {
    control_flow: &'a mut ControlFlow,
}

impl<'a> Context<'a> {
    pub fn exit(&mut self) {
        *self.control_flow = ControlFlow::Exit;
    }
}

pub fn run<A>(mut app: A) -> Result<(), Box<dyn std::error::Error + 'static>>
where
    A: App + 'static,
{
    let mut app_ctr = AppContainer {
        app,
        frame_buf: Frame::new(A::SIZE),
    };
    let instance = wgpu::Instance::new();
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
    });
    let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits { max_bind_groups: 1 },
    });
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        //.with_resizable(false)
        .with_class("floating".into(), "floating".into())
        .build(&event_loop)?;
    let surface = instance.create_surface(window.raw_window_handle());

    let render_format = wgpu::TextureFormat::Bgra8UnormSrgb;
    let mut size = window.inner_size().to_physical(window.hidpi_factor());

    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: render_format,
            width: size.width.round() as u32,
            height: size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        },
    );

    let mut glyph_brush =
        GlyphBrushBuilder::using_font_bytes(FONT).build(&mut device, render_format);

    //let mut last_resize_time: Option<Instant> = None;
    let mut tmp_str = String::from(" ");
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::EventsCleared => {
                // update state
                app_ctr.frame_buf.clear();
                app_ctr.app.update(&mut app_ctr.frame_buf);
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                log::trace!("draw");
                // draw
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                let frame = swap_chain.get_next_texture();
                {
                    let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                }
                let mut draw_char = |Point2 { x, y }: Point2<usize>, ch: char, color: [f32; 4]| {
                    tmp_str.clear();
                    tmp_str.push(ch);
                    glyph_brush.queue(Section {
                        text: &tmp_str,
                        screen_position: (
                            x as f32 * size.width as f32 / A::SIZE.x as f32,
                            y as f32 * size.height as f32 / A::SIZE.y as f32,
                        ),
                        color,
                        scale: Scale {
                            x: size.width as f32 / (A::SIZE.x as f32 / 2.0),
                            y: size.height as f32 / A::SIZE.y as f32,
                        },
                        bounds: (size.width as f32, size.height as f32),
                        ..Section::default()
                    });
                };
                let frame_size = app_ctr.frame_buf.size;
                for x in 0..frame_size.x {
                    for y in 0..frame_size.y {
                        let idx = Point2 { x, y };
                        let ch = app_ctr.frame_buf.get(idx);
                        // background
                        if ch.bg[3] > f32::EPSILON {
                            draw_char(idx, FULL_BLOCK, ch.bg);
                        }
                        // foreground
                        if ch.fg[3] > f32::EPSILON {
                            draw_char(idx, ch.ch, ch.fg);
                        }
                    }
                }
                if let Err(e) = glyph_brush.draw_queued(
                    &mut device,
                    &mut encoder,
                    &frame.view,
                    size.width.round() as u32,
                    size.height.round() as u32,
                ) {
                    println!("Error drawing queued glyphs: {}", e);
                    *control_flow = ControlFlow::Exit;
                }
                device.get_queue().submit(&[encoder.finish()]);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::DeviceEvent {
                event:
                    DeviceEvent::Key(KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    }),
                ..
            } => {
                if let Some(keycode) = virtual_keycode {
                    let mut ctx = Context { control_flow };
                    match state {
                        ElementState::Pressed => app_ctr.app.key_down_event(ctx, keycode),
                        _ => (),
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                // let's debounce this every 100ms.
                // TODO this isn't debouncing, need to schedule a resize
                /*
                if let Some(time) = last_resize_time {
                    if Instant::now().duration_since(time) < Duration::from_millis(100) {
                        return;
                    }
                }
                last_resize_time = Some(Instant::now());
                */
                size = new_size.to_physical(window.hidpi_factor());
                swap_chain = device.create_swap_chain(
                    &surface,
                    &wgpu::SwapChainDescriptor {
                        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                        format: render_format,
                        width: size.width.round() as u32,
                        height: size.height.round() as u32,
                        present_mode: wgpu::PresentMode::Vsync,
                    },
                );
                window.request_redraw();
            }
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}

#[derive(Debug, Clone)]
pub struct Frame {
    buf: Vec<Char>,
    size: Point2<usize>,
}

#[derive(Debug, Copy, Clone)]
pub struct Char {
    /// Get the character for this tile.
    pub ch: char,
    /// The foreground color of the character
    pub fg: [f32; 4],
    /// The background color of the character
    pub bg: [f32; 4],
}

impl Default for Char {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: [0.0; 4],
            bg: [0.0; 4],
        }
    }
}

impl Frame {
    pub fn new(size: Point2<usize>) -> Self {
        let area = size.x * size.y;
        let mut buf = Vec::with_capacity(area);
        for _ in 0..area {
            buf.push(Default::default());
        }
        Self { buf, size }
    }

    pub fn clear(&mut self) {
        for el in self.buf.iter_mut() {
            *el = Default::default();
        }
    }

    pub fn from_fn(width: usize, height: usize, f: impl Fn(usize, usize) -> Char) -> Self {
        let mut buf = Vec::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                buf.push(f(width, height));
            }
        }
        Self {
            buf,
            size: Point2 {
                x: width,
                y: height,
            },
        }
    }

    /// Get an item from the grid by location.
    ///
    /// You can also use the implementation of `Index` like so: `frame[(1, 2)]`.
    #[inline]
    pub fn get(&self, Point2 { x, y }: Point2<usize>) -> &Char {
        &self.buf[self.idx(x, y)]
    }

    /// Get a mutable ref to and item from the grid by location.
    ///
    /// You can also use the implementation of `IndexMut` like so: `frame[(1, 2)] = 2`.
    #[inline]
    pub fn get_mut(&mut self, Point2 { x, y }: Point2<usize>) -> &mut Char {
        // to keep the borrowchecker happy
        let idx = self.idx(x, y);
        &mut self.buf[idx]
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        x * self.size.y + y
    }

    pub fn debug_print(&self) {
        println!("Frame:");
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                print!("{}", self.get(Point2 { x, y }).ch);
            }
            println!()
        }
    }
}

impl Index<(usize, usize)> for Frame {
    type Output = Char;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self[Point2 { x, y }]
    }
}

impl IndexMut<(usize, usize)> for Frame {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self[Point2 { x, y }]
    }
}

impl Index<Point2<usize>> for Frame {
    type Output = Char;
    fn index(&self, pos: Point2<usize>) -> &Self::Output {
        self.get(pos)
    }
}

impl IndexMut<Point2<usize>> for Frame {
    fn index_mut(&mut self, pos: Point2<usize>) -> &mut Self::Output {
        self.get_mut(pos)
    }
}
