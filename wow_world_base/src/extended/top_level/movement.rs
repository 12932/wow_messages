/// Default forward running speed.
pub const DEFAULT_RUNNING_SPEED: f32 = 7.0;
/// Default backwards running speed.
pub const DEFAULT_RUNNING_BACKWARDS_SPEED: f32 = 4.5;
/// Default forward walking speed.
pub const DEFAULT_WALKING_SPEED: f32 = 2.5;
/// Default turning speed. Unknown unit, but is always [pi](std::f32::consts::PI).
pub const DEFAULT_TURN_SPEED: f32 = std::f32::consts::PI;

#[cfg(test)]
mod tests {
    use super::*;

    /// mangos-zero's and mangos-classic's `baseMoveSpeed` (`Unit.cpp`):
    /// walk, run and run back.
    #[test]
    fn defaults_are_mangos_base_move_speeds() {
        assert_eq!(DEFAULT_WALKING_SPEED, 2.5);
        assert_eq!(DEFAULT_RUNNING_SPEED, 7.0);
        assert_eq!(DEFAULT_RUNNING_BACKWARDS_SPEED, 4.5);
    }
}
