use super::graphics::Color;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rect {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
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
