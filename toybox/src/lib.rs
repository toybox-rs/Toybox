
#[derive(Debug,PartialEq,Eq)]
struct Vec2D {
    x: i32,
    y: i32,
}

impl Vec2D {
    fn new(x: i32, y: i32) -> Vec2D {
        Vec2D { x, y }
    }
    fn origin() -> Vec2D {
        Vec2D { x: 0, y: 0 }
    }
    fn translate(&self, by: Vec2D) -> Vec2D {
        Vec2D::new(self.x + by.x, self.y + by.y)
    }
}

enum Drawable {
    Rectangle(u32, u32, u32, u32),
    Circle(u32, u32, u32),
}

trait GameVisualOutput {
    fn game_size() -> Vec2D;
    fn display_list() -> Vec<Drawable>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ball = Vec2D::new(7,3);
        let velocity = Vec2D::new(4,4);
        let pos = ball.translate(velocity);
        assert_eq!(pos, Vec2D::new(11,7));
    }
}
