/// For now we only support RGB colors so we don't have to do alpha-blending in our software renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    a: u8,
}

impl Color {
    /// Create a color from (r, g, b) components.
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub fn invisible() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
    pub fn is_visible(self) -> bool {
        self.a > 0
    }
    pub fn black() -> Color {
        Color::rgb(0, 0, 0)
    }
    pub fn white() -> Color {
        Color::rgb(0xff, 0xff, 0xff)
    }
}

impl<'a> From<&'a (u8, u8, u8)> for Color {
    fn from(tuple: &'a (u8, u8, u8)) -> Color {
        Color::rgb(tuple.0, tuple.1, tuple.2)
    }
}

#[derive(Clone, Debug)]
pub struct SpriteData {
    pub x: i32,
    pub y: i32,
    scale: i32,
    data: Vec<Vec<Color>>,
}
impl SpriteData {
    pub fn new(data: Vec<Vec<Color>>, scale: i32) -> SpriteData {
        SpriteData {
            x: 0,
            y: 0,
            scale,
            data,
        }
    }
    pub fn width(&self) -> i32 {
        self.data[0].len() as i32
    }
    pub fn height(&self) -> i32 {
        self.data.len() as i32
    }
    pub fn scale(&self) -> i32 {
        self.scale
    }
    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn find_visible_color(&self) -> Option<Color> {
        for row in &self.data {
            for px in row {
                if px.is_visible() {
                    return Some(*px);
                }
            }
        }
        None
    }
    /// Make a full copy of this sprite with a new position.
    pub fn translate(&self, x: i32, y: i32) -> SpriteData {
        SpriteData {
            x,
            y,
            scale: self.scale,
            data: self.data.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Drawable {
    Rectangle {
        color: Color,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
    },
    Sprite(SpriteData),
}

impl Drawable {
    pub fn rect(color: Color, x: i32, y: i32, w: i32, h: i32) -> Drawable {
        Drawable::Rectangle { color, x, y, w, h }
    }
}

pub struct ImageBuffer {
    pub width: i32,
    pub height: i32,
    /// Pixels encoded as RGBA.
    pub data: Vec<u8>,
}
impl ImageBuffer {
    pub fn alloc(width: i32, height: i32) -> ImageBuffer {
        ImageBuffer {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }
    #[inline(always)]
    fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        debug_assert!(color.is_visible());
        let start = (y * self.width * 4) + (x * 4);
        if start < 0 {
            return;
        }
        let start = start as usize;
        if start >= self.data.len() {
            return;
        }
        self.data[start] = color.r;
        self.data[start + 1] = color.g;
        self.data[start + 2] = color.b;
        self.data[start + 3] = color.a;
    }
    #[inline(always)]
    fn set_pixel_alpha(&mut self, x: i32, y: i32, color: Color) {
        if color.is_visible() {
            self.set_pixel(x, y, color)
        }
    }
}

pub fn render_to_buffer(target: &mut ImageBuffer, commands: &[Drawable]) {
    for cmd in commands {
        match cmd {
            Drawable::Rectangle { color, x, y, w, h } => {
                for yi in *y..(y + h) {
                    for xi in *x..(x + w) {
                        target.set_pixel(xi, yi, *color)
                    }
                }
            }
            Drawable::Sprite(sprite) => {
                let w = sprite.width();
                let h = sprite.height();
                let (x, y) = sprite.position();
                let scale = sprite.scale();
                debug_assert!(scale > 0);
                for yi in 0..h {
                    for xi in 0..w {
                        let color = sprite.data[yi as usize][xi as usize];
                        for xt in 0..sprite.scale {
                            for yt in 0..sprite.scale {
                                target.set_pixel_alpha(xi + x + xt, yi + y + yt, color)
                            }
                        }
                    }
                }
            }
        }
    }
    // Done.
}
