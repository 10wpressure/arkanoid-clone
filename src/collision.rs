use macroquad::prelude::*;

pub fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            match to_signum.y > 0f32 {
                true => vel.y = -vel.y.abs(),
                false => vel.y = vel.y.abs()
            }
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            match to_signum.x < 0f32 {
                true => vel.x = vel.x.abs(),
                false => vel.x = -vel.x.abs()
            }
        }
    }
    true
}