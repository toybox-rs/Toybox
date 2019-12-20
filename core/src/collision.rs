use super::graphics::Color;
use std::cmp;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
    }
    pub fn intersects(&self, rhs: &Rect) -> bool {
        return rect_intersect(self, rhs);
    }
    pub fn x1(&self) -> i32 {
        self.x
    }
    pub fn y1(&self) -> i32 {
        self.y
    }
    pub fn x2(&self) -> i32 {
        self.x + self.w
    }
    pub fn y2(&self) -> i32 {
        self.y + self.h
    }
    pub fn center_x(&self) -> i32 {
        (self.x1() + self.x2()) / 2
    }
    pub fn center_y(&self) -> i32 {
        (self.y1() + self.y2()) / 2
    }
    pub fn contains_xy(&self, x: i32, y: i32) -> bool {
        x >= self.x1() && x <= self.x2() && y >= self.y1() && y <= self.y2()
    }

    /// Detect a collision between a rectangle and a sprite.
    pub fn collides_visible(&self, x: i32, y: i32, sprite: &Vec<Vec<Color>>) -> bool {
        for (yi, row) in sprite.iter().enumerate() {
            for (xi, color) in row.iter().enumerate() {
                if color.is_visible() {
                    let px = (xi as i32) + x;
                    let py = (yi as i32) + y;

                    if self.contains_xy(px, py) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Create a rectangle that surrounds a slice of rectangles.
    pub fn merge(rectangles: &[Rect]) -> Option<Rect> {
        if rectangles.len() == 0 {
            return None;
        }
        let mut x1 = rectangles[0].x1();
        let mut x2 = rectangles[0].x2();
        let mut y1 = rectangles[0].y1();
        let mut y2 = rectangles[0].y2();
        for rect in rectangles.iter() {
            x1 = cmp::min(rect.x1(), x1);
            x2 = cmp::max(rect.x2(), x2);
            y1 = cmp::min(rect.y1(), y1);
            y2 = cmp::max(rect.y2(), y2);
        }
        Some(Rect {
            x: x1,
            y: y1,
            w: x2 - x1,
            h: y2 - y1,
        })
    }
}

/// Core algorithm from [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection)
fn rect_intersect(r1: &Rect, r2: &Rect) -> bool {
    r1.x <= r2.x + r2.w && r1.x + r1.w >= r2.x && r1.y <= r2.y + r2.h && r1.y + r1.h >= r2.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rect_miss() {
        let r1 = Rect::new(0, 0, 1, 1);
        let r2 = Rect::new(3, 3, 1, 1);
        assert_eq!(false, r1.intersects(&r2));
    }
}
