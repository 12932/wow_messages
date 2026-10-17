use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::monster_move_type_vanilla_tbc_wrath::MonsterMoveType;
use wow_world_base::shared::spline_flag_vanilla_tbc::SplineFlag;
use wow_world_base::shared::vector3d_vanilla_tbc_wrath::Vector3d;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// A STOP ends after the move type (mangos-zero, cmangos-classic, cmangos-tbc and vmangos).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L11):
/// ```text
/// smsg SMSG_MONSTER_MOVE = 0x00DD {
///     PackedGuid guid;
///     Vector3d spline_point;
///     u32 spline_id;
///     MonsterMoveType move_type;
///     if (move_type == FACING_TARGET) {
///         Guid target;
///     }
///     else if (move_type == FACING_ANGLE) {
///         f32 angle;
///     }
///     else if (move_type == FACING_SPOT) {
///         Vector3d position;
///     }
///     if (move_type != STOP) {
///         SplineFlag spline_flags;
///         u32 duration;
///         MonsterMoveSplines splines;
///     }
/// }
/// ```
pub struct SMSG_MONSTER_MOVE {
    pub guid: Guid,
    pub spline_point: Vector3d,
    pub spline_id: u32,
    pub move_type: SMSG_MONSTER_MOVE_MonsterMoveType,
}

impl crate::private::Sealed for SMSG_MONSTER_MOVE {}
impl SMSG_MONSTER_MOVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(18..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        let mut move_type_if_target = Default::default();
        let mut move_type_if_angle = Default::default();
        let mut move_type_if_position = Default::default();
        let mut move_type_if_spline_flags = Default::default();
        let mut move_type_if_duration = Default::default();
        let mut move_type_if_splines = Default::default();

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // spline_point: Vector3d
        let spline_point = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // spline_id: u32
        let spline_id = crate::util::read_u32_le(&mut r)?;

        // move_type: MonsterMoveType
        let move_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        match move_type {
            MonsterMoveType::Normal => {
            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // position: Vector3d
                move_type_if_position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

            }
            MonsterMoveType::FacingTarget => {
                // target: Guid
                move_type_if_target = crate::util::read_guid(&mut r)?;

            }
            MonsterMoveType::FacingAngle => {
                // angle: f32
                move_type_if_angle = crate::util::read_f32_le(&mut r)?;

            }
        };

        match move_type {
            MonsterMoveType::Normal => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = crate::util::read_monster_move_spline(&mut r)?;

            }
            MonsterMoveType::Stop => {}
            MonsterMoveType::FacingSpot => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = crate::util::read_monster_move_spline(&mut r)?;

            }
            MonsterMoveType::FacingTarget => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = crate::util::read_monster_move_spline(&mut r)?;

            }
            MonsterMoveType::FacingAngle => {
                // spline_flags: SplineFlag
                move_type_if_spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

                // duration: u32
                move_type_if_duration = crate::util::read_u32_le(&mut r)?;

                // splines: MonsterMoveSplines
                move_type_if_splines = crate::util::read_monster_move_spline(&mut r)?;

            }
        };

        let move_type_if = match move_type {
            MonsterMoveType::Normal => {
                SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                    duration: move_type_if_duration,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                }
            }
            MonsterMoveType::Stop => {
                SMSG_MONSTER_MOVE_MonsterMoveType::Stop {
                }
            }
            MonsterMoveType::FacingSpot => {
                SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                    duration: move_type_if_duration,
                    position: move_type_if_position,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                }
            }
            MonsterMoveType::FacingTarget => {
                SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                    duration: move_type_if_duration,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                    target: move_type_if_target,
                }
            }
            MonsterMoveType::FacingAngle => {
                SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                    angle: move_type_if_angle,
                    duration: move_type_if_duration,
                    spline_flags: move_type_if_spline_flags,
                    splines: move_type_if_splines,
                }
            }
        };

        Ok(Self {
            guid,
            spline_point,
            spline_id,
            move_type: move_type_if,
        })
    }

}

impl crate::Message for SMSG_MONSTER_MOVE {
    const OPCODE: u32 = 0x00dd;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MONSTER_MOVE"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // spline_point: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.spline_point, &mut w)?;

        // spline_id: u32
        w.write_all(&self.spline_id.to_le_bytes())?;

        // move_type: MonsterMoveType
        w.write_all(&(self.move_type.as_int().to_le_bytes()))?;

        match &self.move_type {
            SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                duration,
                spline_flags,
                splines,
            } => {
            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                duration,
                position,
                spline_flags,
                splines,
            } => {
                // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&position, &mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                duration,
                spline_flags,
                splines,
                target,
            } => {
                // target: Guid
                w.write_all(&target.guid().to_le_bytes())?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                angle,
                duration,
                spline_flags,
                splines,
            } => {
                // angle: f32
                w.write_all(&angle.to_le_bytes())?;

            }
            _ => {}
        }

        match &self.move_type {
            SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                duration,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int().to_le_bytes()))?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                crate::util::write_monster_move_spline(splines.as_slice(), &mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingSpot {
                duration,
                position,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int().to_le_bytes()))?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                crate::util::write_monster_move_spline(splines.as_slice(), &mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingTarget {
                duration,
                spline_flags,
                splines,
                target,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int().to_le_bytes()))?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                crate::util::write_monster_move_spline(splines.as_slice(), &mut w)?;

            }
            SMSG_MONSTER_MOVE_MonsterMoveType::FacingAngle {
                angle,
                duration,
                spline_flags,
                splines,
            } => {
                // spline_flags: SplineFlag
                w.write_all(&(spline_flags.as_int().to_le_bytes()))?;

                // duration: u32
                w.write_all(&duration.to_le_bytes())?;

                // splines: MonsterMoveSplines
                crate::util::write_monster_move_spline(splines.as_slice(), &mut w)?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(221, "SMSG_MONSTER_MOVE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MONSTER_MOVE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MONSTER_MOVE {}

impl SMSG_MONSTER_MOVE {
    pub(crate) fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 12 // spline_point: Vector3d
        + 4 // spline_id: u32
        + self.move_type.size() // move_type: SMSG_MONSTER_MOVE_MonsterMoveType
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SMSG_MONSTER_MOVE_MonsterMoveType {
    Normal {
        duration: u32,
        spline_flags: SplineFlag,
        splines: Vec<Vector3d>,
    },
    Stop,
    FacingSpot {
        duration: u32,
        position: Vector3d,
        spline_flags: SplineFlag,
        splines: Vec<Vector3d>,
    },
    FacingTarget {
        duration: u32,
        spline_flags: SplineFlag,
        splines: Vec<Vector3d>,
        target: Guid,
    },
    FacingAngle {
        angle: f32,
        duration: u32,
        spline_flags: SplineFlag,
        splines: Vec<Vector3d>,
    },
}

impl Default for SMSG_MONSTER_MOVE_MonsterMoveType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Stop
    }
}

impl SMSG_MONSTER_MOVE_MonsterMoveType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal { .. } => 0,
            Self::Stop => 1,
            Self::FacingSpot { .. } => 2,
            Self::FacingTarget { .. } => 3,
            Self::FacingAngle { .. } => 4,
        }
    }

}

impl std::fmt::Display for SMSG_MONSTER_MOVE_MonsterMoveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal{ .. } => f.write_str("Normal"),
            Self::Stop => f.write_str("Stop"),
            Self::FacingSpot{ .. } => f.write_str("FacingSpot"),
            Self::FacingTarget{ .. } => f.write_str("FacingTarget"),
            Self::FacingAngle{ .. } => f.write_str("FacingAngle"),
        }
    }
}

impl SMSG_MONSTER_MOVE_MonsterMoveType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Normal {
                splines,
                ..
            } => {
                1
                + 4 // duration: u32
                + 4 // spline_flags: SplineFlag
                + crate::util::monster_move_spline_size(splines.as_slice()) // splines: MonsterMoveSplines
            }
            Self::FacingSpot {
                splines,
                ..
            } => {
                1
                + 4 // duration: u32
                + 12 // position: Vector3d
                + 4 // spline_flags: SplineFlag
                + crate::util::monster_move_spline_size(splines.as_slice()) // splines: MonsterMoveSplines
            }
            Self::FacingTarget {
                splines,
                ..
            } => {
                1
                + 4 // duration: u32
                + 4 // spline_flags: SplineFlag
                + crate::util::monster_move_spline_size(splines.as_slice()) // splines: MonsterMoveSplines
                + 8 // target: Guid
            }
            Self::FacingAngle {
                splines,
                ..
            } => {
                1
                + 4 // angle: f32
                + 4 // duration: u32
                + 4 // spline_flags: SplineFlag
                + crate::util::monster_move_spline_size(splines.as_slice()) // splines: MonsterMoveSplines
            }
            _ => 1,
        }
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_MONSTER_MOVE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_MONSTER_MOVE, expected: &SMSG_MONSTER_MOVE) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.spline_point, expected.spline_point);
        assert_eq!(t.spline_id, expected.spline_id);
        assert_eq!(t.move_type, expected.move_type);
    }

    const RAW0: [u8; 27] = [ 0x00, 0x19, 0xDD, 0x00, 0xD9, 0x1A, 0x2B, 0x01, 0x30,
         0xF1, 0x00, 0xAA, 0x0B, 0xC6, 0x00, 0x40, 0x03, 0xC3, 0x00, 0x00, 0xA7,
         0x42, 0x07, 0x00, 0x00, 0x00, 0x01, ];

    pub(crate) fn expected0() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0xF13000012B00001A),
            spline_point: Vector3d {
                x: -8938.5_f32,
                y: -131.25_f32,
                z: 83.5_f32,
            },
            spline_id: 0x7,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Stop,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 33.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 33.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 33.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 51] = [ 0x00, 0x31, 0xDD, 0x00, 0xD9, 0x1A, 0x2B, 0x01, 0x30,
         0xF1, 0x00, 0xAA, 0x0B, 0xC6, 0x00, 0x40, 0x03, 0xC3, 0x00, 0x00, 0xA7,
         0x42, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xE8, 0x03,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0xA7, 0x0B, 0xC6, 0x00, 0x00,
         0xEB, 0xC2, 0x00, 0x80, 0xA4, 0x42, ];

    pub(crate) fn expected1() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0xF13000012B00001A),
            spline_point: Vector3d {
                x: -8938.5_f32,
                y: -131.25_f32,
                z: 83.5_f32,
            },
            spline_id: 0x8,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                duration: 0x3E8,
                spline_flags: SplineFlag::empty()
                    .set_run_mode()
                    ,
                splines: vec![
                    Vector3d {
                        x: -8937.75,
                        y: -117.5,
                        z: 82.25,
                    },
                ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 55.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 55.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 55.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_MONSTER_MOVE;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_MONSTER_MOVE, expected: &SMSG_MONSTER_MOVE) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.spline_point, expected.spline_point);
        assert_eq!(t.spline_id, expected.spline_id);
        assert_eq!(t.move_type, expected.move_type);
    }

    const RAW0: [u8; 27] = [ 0x00, 0x19, 0xDD, 0x00, 0xD9, 0x1A, 0x2B, 0x01, 0x30,
         0xF1, 0x00, 0xAA, 0x0B, 0xC6, 0x00, 0x40, 0x03, 0xC3, 0x00, 0x00, 0xA7,
         0x42, 0x07, 0x00, 0x00, 0x00, 0x01, ];

    pub(crate) fn expected0() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0xF13000012B00001A),
            spline_point: Vector3d {
                x: -8938.5_f32,
                y: -131.25_f32,
                z: 83.5_f32,
            },
            spline_id: 0x7,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Stop,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 33.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 33.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 33.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 51] = [ 0x00, 0x31, 0xDD, 0x00, 0xD9, 0x1A, 0x2B, 0x01, 0x30,
         0xF1, 0x00, 0xAA, 0x0B, 0xC6, 0x00, 0x40, 0x03, 0xC3, 0x00, 0x00, 0xA7,
         0x42, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xE8, 0x03,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0xA7, 0x0B, 0xC6, 0x00, 0x00,
         0xEB, 0xC2, 0x00, 0x80, 0xA4, 0x42, ];

    pub(crate) fn expected1() -> SMSG_MONSTER_MOVE {
        SMSG_MONSTER_MOVE {
            guid: Guid::new(0xF13000012B00001A),
            spline_point: Vector3d {
                x: -8938.5_f32,
                y: -131.25_f32,
                z: 83.5_f32,
            },
            spline_id: 0x8,
            move_type: SMSG_MONSTER_MOVE_MonsterMoveType::Normal {
                duration: 0x3E8,
                spline_flags: SplineFlag::empty()
                    .set_run_mode()
                    ,
                splines: vec![
                    Vector3d {
                        x: -8937.75,
                        y: -117.5,
                        z: 82.25,
                    },
                ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 55.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 55.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm` line 55.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_monster_move1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MONSTER_MOVE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MONSTER_MOVE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

