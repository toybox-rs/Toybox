use toybox_core::collision::Rect;
use toybox_core::graphics::Color;

/// Detect a collision between a rectangle and a sprite, and delete pixels in the intersection.
pub fn destructive_collide(laser: &Rect, x: i32, y: i32, sprite: &mut Vec<Vec<Color>>) -> bool {
    let mut hit = false;
    for (yi, row) in sprite.iter_mut().enumerate() {
        for (xi, color) in row.iter_mut().enumerate() {
            if color.is_visible() {
                let px = (xi as i32) + x;
                let py = (yi as i32) + y;

                if laser.contains_xy(px, py) {
                    *color = Color::invisible();
                    hit = true;
                }
            }
        }
    }
    hit
}
