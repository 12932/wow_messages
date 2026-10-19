//! 1.12 flags: a flag's `clear_*` clears that flag and leaves the others.
//! It used to mask with the flag's bits reversed, so clearing one cleared
//! the rest.
#![cfg(feature = "vanilla")]

use wow_world_messages::vanilla::{
    MovementBlock_SplineFlag, MovementBlock_SplineFlag_FinalAngle, MovementBlock_UpdateFlag,
    MovementBlock_UpdateFlag_Living, MovementFlags, MovementInfo_MovementFlags,
    MovementInfo_MovementFlags_Jumping, SplineFlag, UpdateFlag, Vector3d,
};

#[test]
fn landing_keeps_running_forward() {
    let mut flags = MovementFlags::new(MovementFlags::FORWARD | MovementFlags::JUMPING);
    assert_eq!(flags.clear_jumping().as_int(), MovementFlags::FORWARD);
    assert!(flags.is_forward() && !flags.is_jumping());
}

#[test]
fn landing_keeps_running_forward_and_drops_the_jump() {
    let jump = MovementInfo_MovementFlags_Jumping {
        cos_angle: 1.0,
        sin_angle: 0.0,
        xy_speed: 7.0,
        z_speed: -7.955547,
    };
    let flags = MovementInfo_MovementFlags::new(
        MovementFlags::FORWARD | MovementFlags::JUMPING,
        Some(jump),
        None,
        None,
        None,
    );
    assert_eq!(
        flags.clear_jumping(),
        MovementInfo_MovementFlags::new(MovementFlags::FORWARD, None, None, None, None)
    );
}

/// A flag whose data is one of an else-if's (a spline's final angle,
/// target or point; a unit's living data or bare position) is set by any
/// of the else-if's flags, so clearing it clears whichever it was.
#[test]
fn clearing_an_else_if_clears_the_flag_it_was_set_by() {
    let run = MovementBlock_SplineFlag::new(SplineFlag::RUN_MODE, None);
    let target = MovementBlock_SplineFlag_FinalAngle::FinalTarget { target: 1 };
    assert_eq!(run.set_final_angle(target).clear_final_angle(), run);

    let me = MovementBlock_UpdateFlag::new(UpdateFlag::SELF, None, None, None, None, None);
    let position = MovementBlock_UpdateFlag_Living::HasPosition {
        orientation: 0.0,
        position: Vector3d {
            x: -8835.7,
            y: 620.485,
            z: 93.2388,
        },
    };
    assert_eq!(me.clone().set_living(position).clear_living(), me);
}
