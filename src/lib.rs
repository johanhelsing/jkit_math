// from Game programming gems 4
pub fn smooth_damp(
    current: f32,
    target: f32,
    vel: &mut f32,
    smooth_time: f32,
    delta_time: f32,
) -> f32 {
    let omega = 2. / smooth_time;
    let x = omega * delta_time;
    let exp = 1. / (1. + x + 0.48 * x * x + 0.235 * x * x * x);
    let change = current - target;
    let temp = (*vel + omega * change) * delta_time;
    *vel = (*vel - omega * temp) * exp;
    target + (change + temp) * exp
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

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
}
