use toybox_core::collision::Rect;
use toybox_core::graphics::Color;

pub fn collision_fallout(laser: &Rect, x: i32, y: i32) -> bool {
    laser.contains_xy(x, y)
        || laser.contains_xy(x + 1, y)
        || laser.contains_xy(x, y + 1)
        || laser.contains_xy(x + 1, y + 1)
}

/// Detect a collision between a rectangle and a sprite, and delete pixels in the intersection.
pub fn destructive_collide(laser: &Rect, x: i32, y: i32, sprite: &mut Vec<Vec<Color>>) -> bool {
    let mut hit = false;
    for (yi, row) in sprite.iter_mut().enumerate() {
        for (xi, color) in row.iter_mut().enumerate() {
            if color.is_visible() {
                let px = (xi as i32) + x;
                let py = (yi as i32) + y;

                if collision_fallout(laser, px, py) {
                    *color = Color::invisible();
                    hit = true;
                }
            }
        }
    }
    hit
}
