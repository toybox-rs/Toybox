/// For now we only support RGB colors so we don't have to do alpha-blending in our software renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
    a: u8,
}

impl Color {
    pub fn RGB(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }
    pub fn black() -> Color {
        Color::RGB(0, 0, 0)
    }
    pub fn white() -> Color {
        Color::RGB(0xff, 0xff, 0xff)
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
    fn set_pixel(&mut self, x: i32, y: i32, color: &Color) {
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
}

pub fn render_to_buffer(target: &mut ImageBuffer, commands: &[Drawable]) {
    for cmd in commands {
        match cmd {
            Drawable::Rectangle { color, x, y, w, h } => {
                for yi in *y..(y + h) {
                    for xi in *x..(x + w) {
                        target.set_pixel(xi, yi, color)
                    }
                }
            }
        }
    }
    // Done.
}
