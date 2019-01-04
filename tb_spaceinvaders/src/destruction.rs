use toybox_core::collision::Rect;
use toybox_core::graphics::Color;

pub fn collision_fallout(laser: &Rect, x: i32, y: i32) -> bool {
    laser.contains_xy(x, y)
}

/// Detect a collision between a rectangle and a sprite, and delete pixels in the intersection.
pub fn destructive_collide(laser: &Rect, x: i32, y: i32, sprite: &mut Vec<Vec<Color>>) -> bool {
    let mut hit = Vec::new();
    for (yi, row) in sprite.iter().enumerate() {
        for (xi, color) in row.iter().enumerate() {
            if color.is_visible() {
                let px = (xi as i32) + x;
                let py = (yi as i32) + y;

                if collision_fallout(laser, px, py) {
                    hit.push((xi, yi))
                }
            }
        }
    }

    for (x, y) in hit.iter() {
        // Align any laser hits to 2x2 grid.
        let px = ((x / 2) * 2) as usize;
        let py = ((y / 2) * 2) as usize;

        // Take 2x2 bites out of the shield:
        sprite[py][px] = Color::invisible();
        sprite[py + 1][px] = Color::invisible();
        sprite[py][px + 1] = Color::invisible();
        sprite[py + 1][px + 1] = Color::invisible();
    }

    !hit.is_empty()
}
