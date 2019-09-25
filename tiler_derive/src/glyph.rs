use nom::IResult;
use rusttype::{point, Font as RTFont, FontCollection, Point, Scale};
use std::u8;

const FONT_WIDTH: usize = 10;
const FONT_HEIGHT: usize = 20;

pub struct Font {
    inner: RTFont<'static>,
    scale: Scale,
    offset: Point<f32>,
}

impl Font {
    pub fn new() -> Self {
        let font_data = include_bytes!("../font.ttf");
        let collection =
            FontCollection::from_bytes(&font_data[..]).expect("failed to parse otf font");
        let font = collection
            .into_font()
            .expect("expected single font, found multiple");
        let scale = Scale::uniform(20.0);
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        Font {
            inner: font,
            scale,
            offset,
        }
    }

    pub fn glyph(&self, ch: char, fg: Color, bg: Color) -> Glyph {
        let glyph = self
            .inner
            .glyph(ch)
            .scaled(self.scale)
            .positioned(self.offset);

        println!("{:#?}", ch);
        println!("{:#?}", glyph);
        let mut output = Glyph::blank(bg);
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < (FONT_WIDTH as i32) && y >= 0 && y < (FONT_HEIGHT as i32) {
                    let x = x as usize;
                    let y = y as usize;
                    // TODO this is wrong when we have different backgrounds
                    fn scale_u8(val: f32) -> u8 {
                        (val * (u8::MAX as f32)) as u8
                    }
                    output.set_pixel(x, y, Color::mono(scale_u8(v)));
                }
                //println!("{}, {} -> {}", x, y, v)
            });
        }
        assert!(output.is_valid());
        output
    }
}

pub struct Glyph {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl Glyph {
    fn is_valid(&self) -> bool {
        (self.width * self.height * 4) as usize == self.data.len()
    }
    fn blank(color: Color) -> Self {
        let mut data = Vec::with_capacity(FONT_WIDTH * FONT_HEIGHT * 4);
        for _ in 0..(FONT_WIDTH * FONT_HEIGHT) {
            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
            data.push(color.a);
        }
        Self {
            width: FONT_WIDTH,
            height: FONT_HEIGHT,
            data,
        }
    }

    pub fn pixel_start(&self, x: usize, y: usize) -> usize {
        y * (self.width as usize) + x
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let idx = self.pixel_start(x, y);
        self.data[idx * 4] = color.r;
        self.data[idx * 4 + 1] = color.g;
        self.data[idx * 4 + 2] = color.b;
        self.data[idx * 4 + 3] = color.a;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color::mono(u8::MAX);
    pub const BLACK: Color = Color::mono(0);

    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }

    #[inline]
    pub const fn mono(level: u8) -> Self {
        Self::rgb(level, level, level)
    }

    /// Parse a color like you would in html
    pub fn parse(input: &str) -> Result<Self, String> {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
enum RawColor {
    Rgb(u8, u8, u8),
    Hsl(u16, f32, f32),
    // Red HTML Color Names
    /// rgb(205, 92, 92)
    IndianRed,
    /// rgb(240, 128, 128)
    LightCoral,
    /// rgb(250, 128, 114)
    Salmon,
    /// rgb(233, 150, 122)
    DarkSalmon,
    /// rgb(255, 160, 122)
    LightSalmon,
    /// rgb(220, 20, 60)
    Crimson,
    /// rgb(255, 0, 0)
    Red,
    /// rgb(178, 34, 34)
    FireBrick,
    /// rgb(139, 0, 0)
    DarkRed,
    // Pink HTML Color Names
    /// rgb(255, 192, 203)
    Pink,
    /// rgb(255, 182, 193)
    LightPink,
    /// rgb(255, 105, 180)
    HotPink,
    /// rgb(255, 20, 147)
    DeepPink,
    /// rgb(199, 21, 133)
    MediumVioletRed,
    /// rgb(219, 112, 147)
    PaleVioletRed,
    //Orange HTML Color Names
    // /// rgb(255, 160, 122) redefined
    // LightSalmon,
    /// rgb(255, 127, 80)
    Coral,
    /// rgb(255, 99, 71)
    Tomato,
    /// rgb(255, 69, 0)
    OrangeRed,
    /// rgb(255, 140, 0)
    DarkOrange,
    /// rgb(255, 165, 0)
    Orange,
    // Yellow HTML Color Names
    /// rgb(255, 215, 0)
    Gold,
    /// rgb(255, 255, 0)
    Yellow,
    /// rgb(255, 255, 224)
    LightYellow,
    /// rgb(255, 250, 205)
    LemonChiffon,
    /// rgb(250, 250, 210)
    LightGoldenrodYellow,
    /// rgb(255, 239, 213)
    PapayaWhip,
    /// rgb(255, 228, 181)
    Moccasin,
    /// rgb(255, 218, 185)
    PeachPuff,
    /// rgb(238, 232, 170)
    PaleGoldenrod,
    /// rgb(240, 230, 140)
    Khaki,
    /// rgb(189, 183, 107)
    DarkKhaki,
    // Purple HTML Color Names
    /// rgb(230, 230, 250)
    Lavender,
    /// rgb(216, 191, 216)
    Thistle,
    /// rgb(221, 160, 221)
    Plum,
    /// rgb(238, 130, 238)
    Violet,
    /// rgb(218, 112, 214)
    Orchid,
    /// rgb(255, 0, 255)
    Fuchsia,
    /// rgb(255, 0, 255)
    Magenta,
    /// rgb(186, 85, 211)
    MediumOrchid,
    /// rgb(147, 112, 219)
    MediumPurple,
    /// rgb(102, 51, 153)
    RebeccaPurple,
    /// rgb(138, 43, 226)
    BlueViolet,
    /// rgb(148, 0, 211)
    DarkViolet,
    /// rgb(153, 50, 204)
    DarkOrchid,
    /// rgb(139, 0, 139)
    DarkMagenta,
    /// rgb(128, 0, 128)
    Purple,
    /// rgb(75, 0, 130)
    Indigo,
    /// rgb(106, 90, 205)
    SlateBlue,
    /// rgb(72, 61, 139)
    DarkSlateBlue,
    /// rgb(123, 104, 238)
    MediumSlateBlue,
    // Green HTML Color Names
    /// rgb(173, 255, 47)
    GreenYellow,
    /// rgb(127, 255, 0)
    Chartreuse,
    /// rgb(124, 252, 0)
    LawnGreen,
    /// rgb(0, 255, 0)
    Lime,
    /// rgb(50, 205, 50)
    LimeGreen,
    /// rgb(152, 251, 152)
    PaleGreen,
    /// rgb(144, 238, 144)
    LightGreen,
    /// rgb(0, 250, 154)
    MediumSpringGreen,
    /// rgb(0, 255, 127)
    SpringGreen,
    /// rgb(60, 179, 113)
    MediumSeaGreen,
    /// rgb(46, 139, 87)
    SeaGreen,
    /// rgb(34, 139, 34)
    ForestGreen,
    /// rgb(0, 128, 0)
    Green,
    /// rgb(0, 100, 0)
    DarkGreen,
    /// rgb(154, 205, 50)
    YellowGreen,
    /// rgb(107, 142, 35)
    OliveDrab,
    /// rgb(128, 128, 0)
    Olive,
    /// rgb(85, 107, 47)
    DarkOliveGreen,
    /// rgb(102, 205, 170)
    MediumAquamarine,
    /// rgb(143, 188, 139)
    DarkSeaGreen,
    /// rgb(32, 178, 170)
    LightSeaGreen,
    /// rgb(0, 139, 139)
    DarkCyan,
    /// rgb(0, 128, 128)
    Teal,
    // Blue HTML Color Names
    /// rgb(0, 255, 255)
    Aqua,
    /// rgb(0, 255, 255)
    Cyan,
    /// rgb(224, 255, 255)
    LightCyan,
    /// rgb(175, 238, 238)
    PaleTurquoise,
    /// rgb(127, 255, 212)
    Aquamarine,
    /// rgb(64, 224, 208)
    Turquoise,
    /// rgb(72, 209, 204)
    MediumTurquoise,
    /// rgb(0, 206, 209)
    DarkTurquoise,
    /// rgb(95, 158, 160)
    CadetBlue,
    /// rgb(70, 130, 180)
    SteelBlue,
    /// rgb(176, 196, 222)
    LightSteelBlue,
    /// rgb(176, 224, 230)
    PowderBlue,
    /// rgb(173, 216, 230)
    LightBlue,
    /// rgb(135, 206, 235)
    SkyBlue,
    /// rgb(135, 206, 250)
    LightSkyBlue,
    /// rgb(0, 191, 255)
    DeepSkyBlue,
    /// rgb(30, 144, 255)
    DodgerBlue,
    /// rgb(100, 149, 237)
    CornflowerBlue,
    // /// rgb(123, 104, 238) duplicate
    //MediumSlateBlue,
    /// rgb(65, 105, 225)
    RoyalBlue,
    /// rgb(0, 0, 255)
    Blue,
    /// rgb(0, 0, 205)
    MediumBlue,
    /// rgb(0, 0, 139)
    DarkBlue,
    /// rgb(0, 0, 128)
    Navy,
    /// rgb(25, 25, 112)
    MidnightBlue,
    // Brown HTML Color Names
    /// rgb(255, 248, 220)
    Cornsilk,
    /// rgb(255, 235, 205)
    BlanchedAlmond,
    /// rgb(255, 228, 196)
    Bisque,
    /// rgb(255, 222, 173)
    NavajoWhite,
    /// rgb(245, 222, 179)
    Wheat,
    /// rgb(222, 184, 135)
    BurlyWood,
    /// rgb(210, 180, 140)
    Tan,
    /// rgb(188, 143, 143)
    RosyBrown,
    /// rgb(244, 164, 96)
    SandyBrown,
    /// rgb(218, 165, 32)
    Goldenrod,
    /// rgb(184, 134, 11)
    DarkGoldenrod,
    /// rgb(205, 133, 63)
    Peru,
    /// rgb(210, 105, 30)
    Chocolate,
    /// rgb(139, 69, 19)
    SaddleBrown,
    /// rgb(160, 82, 45)
    Sienna,
    /// rgb(165, 42, 42)
    Brown,
    /// rgb(128, 0, 0)
    Maroon,
    // White HTML Color Names
    /// rgb(255, 255, 255)
    White,
    /// rgb(255, 250, 250)
    Snow,
    /// rgb(240, 255, 240)
    HoneyDew,
    /// rgb(245, 255, 250)
    MintCream,
    /// rgb(240, 255, 255)
    Azure,
    /// rgb(240, 248, 255)
    AliceBlue,
    /// rgb(248, 248, 255)
    GhostWhite,
    /// rgb(245, 245, 245)
    WhiteSmoke,
    /// rgb(255, 245, 238)
    SeaShell,
    /// rgb(245, 245, 220)
    Beige,
    /// rgb(253, 245, 230)
    OldLace,
    /// rgb(255, 250, 240)
    FloralWhite,
    /// rgb(255, 255, 240)
    Ivory,
    /// rgb(250, 235, 215)
    AntiqueWhite,
    /// rgb(250, 240, 230)
    Linen,
    /// rgb(255, 240, 245)
    LavenderBlush,
    /// rgb(255, 228, 225)
    MistyRose,
    // Gray HTML Color Names
    /// rgb(220, 220, 220)
    Gainsboro,
    /// rgb(211, 211, 211)
    LightGray,
    /// rgb(192, 192, 192)
    Silver,
    /// rgb(169, 169, 169)
    DarkGray,
    /// rgb(128, 128, 128)
    Gray,
    /// rgb(105, 105, 105)
    DimGray,
    /// rgb(119, 136, 153)
    LightSlateGray,
    /// rgb(112, 128, 144)
    SlateGray,
    /// rgb(47, 79, 79)
    DarkSlateGray,
    /// rgb(0, 0, 0)
    Black,
}

impl RawColor {
    fn parse(input: &str) -> Option<Self> {
        if let Some(val) = Self::from_named(input) {
            return Some(val);
        }
        if let Ok((_, val)) = Self::parse_hsl(input) {
            return Some(val);
        }
        if let Ok((_, val)) = Self::parse_hex(input) {
            return Some(val);
        }
        if let Ok((_, val)) = Self::parse_rgb(input) {
            return Some(val);
        }
        None
    }

    #[inline]
    fn from_named(name: &str) -> Option<Self> {
        use RawColor::*;
        let name = name.to_ascii_lowercase();
        Some(match name.as_str() {
            "indianred" => IndianRed,
            "lightcoral" => LightCoral,
            "salmon" => Salmon,
            "darksalmon" => DarkSalmon,
            "lightsalmon" => LightSalmon,
            "crimson" => Crimson,
            "red" => Red,
            "firebrick" => FireBrick,
            "darkred" => DarkRed,
            "pink" => Pink,
            "lightpink" => LightPink,
            "hotpink" => HotPink,
            "deeppink" => DeepPink,
            "mediumvioletred" => MediumVioletRed,
            "palevioletred" => PaleVioletRed,
            "coral" => Coral,
            "tomato" => Tomato,
            "orangered" => OrangeRed,
            "darkorange" => DarkOrange,
            "orange" => Orange,
            "gold" => Gold,
            "yellow" => Yellow,
            "lightyellow" => LightYellow,
            "lemonchiffon" => LemonChiffon,
            "lightgoldenrodyellow" => LightGoldenrodYellow,
            "papayawhip" => PapayaWhip,
            "Moccasin" => Moccasin,
            "Peachpuff" => PeachPuff,
            "palegoldenrod" => PaleGoldenrod,
            "khaki" => Khaki,
            "darkkhaki" => DarkKhaki,
            "lavender" => Lavender,
            "thistle" => Thistle,
            "plum" => Plum,
            "violet" => Violet,
            "orchid" => Orchid,
            "fuchsia" => Fuchsia,
            "magenta" => Magenta,
            "mediumorchid" => MediumOrchid,
            "mediumpurple" => MediumPurple,
            "rebeccapurple" => RebeccaPurple,
            "blueviolet" => BlueViolet,
            "darkviolet" => DarkViolet,
            "darkorchid" => DarkOrchid,
            "darkmagenta" => DarkMagenta,
            "purple" => Purple,
            "indigo" => Indigo,
            "slateblue" => SlateBlue,
            "darkslateblue" => DarkSlateBlue,
            "mediumslateblue" => MediumSlateBlue,
            "greenyellow" => GreenYellow,
            "chartreuse" => Chartreuse,
            "lawngreen" => LawnGreen,
            "lime" => Lime,
            "limegreen" => LimeGreen,
            "palegreen" => PaleGreen,
            "lightgreen" => LightGreen,
            "mediumspringgreen" => MediumSpringGreen,
            "springgreen" => SpringGreen,
            "mediumseagreen" => MediumSeaGreen,
            "seagreen" => SeaGreen,
            "forestgreen" => ForestGreen,
            "green" => Green,
            "darkgreen" => DarkGreen,
            "yellowgreen" => YellowGreen,
            "olivedrab" => OliveDrab,
            "olive" => Olive,
            "darkolivegreen" => DarkOliveGreen,
            "mediumaquamarine" => MediumAquamarine,
            "darkseagreen" => DarkSeaGreen,
            "lightseagreen" => LightSeaGreen,
            "darkcyan" => DarkCyan,
            "teal" => Teal,
            "aqua" => Aqua,
            "cyan" => Cyan,
            "lightcyan" => LightCyan,
            "paleturquoise" => PaleTurquoise,
            "aquamarine" => Aquamarine,
            "turquoise" => Turquoise,
            "mediumturquoise" => MediumTurquoise,
            "darkturquoise" => DarkTurquoise,
            "cadetblue" => CadetBlue,
            "steelblue" => SteelBlue,
            "lightsteelblue" => LightSteelBlue,
            "powderblue" => PowderBlue,
            "lightblue" => LightBlue,
            "skyblue" => SkyBlue,
            "lightskyblue" => LightSkyBlue,
            "deepskyblue" => DeepSkyBlue,
            "dodgerblue" => DodgerBlue,
            "cornflowerblue" => CornflowerBlue,
            "royalblue" => RoyalBlue,
            "blue" => Blue,
            "mediumblue" => MediumBlue,
            "darkblue" => DarkBlue,
            "navy" => Navy,
            "midnightblue" => MidnightBlue,
            "cornsilk" => Cornsilk,
            "blanchedalmond" => BlanchedAlmond,
            "bisque" => Bisque,
            "navajowhite" => NavajoWhite,
            "wheat" => Wheat,
            "burlywood" => BurlyWood,
            "tan" => Tan,
            "rosybrown" => RosyBrown,
            "sandybrown" => SandyBrown,
            "goldenrod" => Goldenrod,
            "darkgoldenrod" => DarkGoldenrod,
            "peru" => Peru,
            "chocolate" => Chocolate,
            "saddlebrown" => SaddleBrown,
            "sienna" => Sienna,
            "brown" => Brown,
            "maroon" => Maroon,
            "white" => White,
            "snow" => Snow,
            "honeydew" => HoneyDew,
            "mintcream" => MintCream,
            "azure" => Azure,
            "aliceblue" => AliceBlue,
            "ghostwhite" => GhostWhite,
            "whitesmoke" => WhiteSmoke,
            "seashell" => SeaShell,
            "beige" => Beige,
            "oldlace" => OldLace,
            "floralwhite" => FloralWhite,
            "ivory" => Ivory,
            "antiquewhite" => AntiqueWhite,
            "linen" => Linen,
            "lavenderblush" => LavenderBlush,
            "mistyrose" => MistyRose,
            "gainsboro" => Gainsboro,
            "lightgray" => LightGray,
            "silver" => Silver,
            "darkgray" => DarkGray,
            "gray" => Gray,
            "dimgray" => DimGray,
            "lightslategray" => LightSlateGray,
            "slategray" => SlateGray,
            "darkslategray" => DarkSlateGray,
            "black" => Black,
            _ => return None,
        })
    }

    fn parse_hsl(input: &str) -> IResult<&str, Self> {
        unimplemented!()
    }

    fn parse_hex(input: &str) -> IResult<&str, Self> {
        use nom::{
            bytes::complete::{tag, take_while_m_n},
            combinator::map_res,
            sequence::tuple,
        };
        fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
            u8::from_str_radix(input, 8)
        }
        fn is_hex_digit(c: char) -> bool {
            c.is_digit(16)
        }
        fn hex_primary(input: &str) -> IResult<&str, u8> {
            map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
        }
        let (input, _) = tag("#")(input)?;
        let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
        Ok((input, RawColor::Rgb(red, green, blue)))
    }

    fn parse_rgb(input: &str) -> IResult<&str, Self> {
        unimplemented!()
    }
}

fn consume_space(input: &[u8]) -> IResult<&[u8], &[u8]> {
    use nom::{bytes::complete::take_while, character::is_space};
    take_while(is_space)(input)
}

#[cfg(test)]
mod tests {
    use super::RawColor;

    #[test]
    fn parse_hex() {
        assert_eq!(
            RawColor::parse_hex("#ffffff"),
            Ok(("", RawColor::Rgb(255, 255, 255)))
        );
    }
}
