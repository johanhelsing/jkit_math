use std::ops::{Add, Mul, Sub};

// from Game programming gems 4
// TODO: switch to returning tuples when tuple assignment is stable
pub fn smooth_damp<T>(current: T, target: T, vel: &mut T, smooth_time: f32, delta_time: f32) -> T
where
    T: Sub<T, Output = T> + Add<T, Output = T> + Mul<f32, Output = T> + Add<T, Output = T> + Copy,
    f32: Mul<T, Output = T> + Mul<f32, Output = f32>,
{
    // The rust compiler is super-weird about this, * multiplication doesn't work when Vec2 is imported in the tests
    let fmul = <f32 as Mul<f32>>::mul;

    let omega = 2. / smooth_time;
    let x = fmul(omega, delta_time);
    let exp = 1. / (1. + x + fmul(fmul(0.48, x), x) + fmul(fmul(0.235, x), fmul(x, x)));
    let change = current - target;
    let temp: T = (*vel + (omega * change)) * delta_time;
    *vel = (*vel - omega * temp) * exp;
    target + (change + temp) * exp
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use bevy::math::Vec2;

    use crate::smooth_damp;

    #[test]
    fn smooth_damp_converges() {
        let mut current = 0.0;
        let target = 1.0;
        let delta_time = 0.1;
        let mut vel = 0.;
        let smooth_time = 1.0;

        for _ in 0..100 {
            current = smooth_damp(current, target, &mut vel, smooth_time, delta_time);
        }

        assert_relative_eq!(current, target);
    }

    #[test]
    fn smooth_damp_vec() {
        let mut current = Vec2::new(0., 1.);
        let target = Vec2::new(1., 0.);
        let delta_time = 0.1;
        let mut vel = Vec2::ZERO;
        let smooth_time = 1.0;

        for _ in 0..100 {
            current = smooth_damp(current, target, &mut vel, smooth_time, delta_time);
        }

        assert_relative_eq!(current.x, target.x);
        assert_relative_eq!(current.x, target.x);
    }
}
