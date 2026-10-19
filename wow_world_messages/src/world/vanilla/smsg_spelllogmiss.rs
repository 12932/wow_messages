use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    SpellLogMiss, SpellLogMissDebug, SpellLogMissLayout,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// mangos-classic's `Unit::SendSpellMiss` writes the spell, the caster, a byte of 0, the count and each target's guid and reason (a byte). The 1.12 client's handler reads the same, and two floats more a target when the byte isn't 0.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:28`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L28):
/// ```text
/// smsg SMSG_SPELLLOGMISS = 0x024B {
///     u32 id;
///     Guid caster;
///     SpellLogMissLayout layout;
///     if (layout == NORMAL) {
///         u32 amount_of_targets;
///         SpellLogMiss[amount_of_targets] targets;
///     }
///     else if (layout == DEBUG) {
///         u32 amount_of_debug_targets;
///         SpellLogMissDebug[amount_of_debug_targets] debug_targets;
///     }
/// }
/// ```
pub struct SMSG_SPELLLOGMISS {
    pub id: u32,
    pub caster: Guid,
    /// cmangos/mangoszero: can be 0 or 1
    pub layout: SMSG_SPELLLOGMISS_SpellLogMissLayout,
}

impl crate::private::Sealed for SMSG_SPELLLOGMISS {}
impl SMSG_SPELLLOGMISS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(13..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // caster: Guid
        let caster = crate::util::read_guid(&mut r)?;

        // layout: SpellLogMissLayout
        let layout = crate::util::read_u8_le(&mut r)?.try_into()?;

        let layout_if = match layout {
            SpellLogMissLayout::Normal => {
                // amount_of_targets: u32
                let amount_of_targets = crate::util::read_u32_le(&mut r)?;

                // targets: SpellLogMiss[amount_of_targets]
                let targets = {
                    let mut targets = Vec::with_capacity(crate::util::capacity(amount_of_targets as usize, r.len(), 9));
                    for _ in 0..amount_of_targets {
                        targets.push(SpellLogMiss::read(&mut r)?);
                    }
                    targets
                };

                SMSG_SPELLLOGMISS_SpellLogMissLayout::Normal {
                    targets,
                }
            }
            SpellLogMissLayout::Debug => {
                // amount_of_debug_targets: u32
                let amount_of_debug_targets = crate::util::read_u32_le(&mut r)?;

                // debug_targets: SpellLogMissDebug[amount_of_debug_targets]
                let debug_targets = {
                    let mut debug_targets = Vec::with_capacity(crate::util::capacity(amount_of_debug_targets as usize, r.len(), 17));
                    for _ in 0..amount_of_debug_targets {
                        debug_targets.push(SpellLogMissDebug::read(&mut r)?);
                    }
                    debug_targets
                };

                SMSG_SPELLLOGMISS_SpellLogMissLayout::Debug {
                    debug_targets,
                }
            }
        };

        Ok(Self {
            id,
            caster,
            layout: layout_if,
        })
    }

}

impl crate::Message for SMSG_SPELLLOGMISS {
    const OPCODE: u32 = 0x024b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPELLLOGMISS"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // layout: SpellLogMissLayout
        w.write_all(&(self.layout.as_int().to_le_bytes()))?;

        match &self.layout {
            SMSG_SPELLLOGMISS_SpellLogMissLayout::Normal {
                targets,
            } => {
                // amount_of_targets: u32
                w.write_all(&(targets.len() as u32).to_le_bytes())?;

                // targets: SpellLogMiss[amount_of_targets]
                for i in targets.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
            SMSG_SPELLLOGMISS_SpellLogMissLayout::Debug {
                debug_targets,
            } => {
                // amount_of_debug_targets: u32
                w.write_all(&(debug_targets.len() as u32).to_le_bytes())?;

                // debug_targets: SpellLogMissDebug[amount_of_debug_targets]
                for i in debug_targets.iter() {
                    i.write_into_vec(&mut w)?;
                }

            }
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(587, "SMSG_SPELLLOGMISS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLLOGMISS {}

impl SMSG_SPELLLOGMISS {
    pub(crate) fn size(&self) -> usize {
        4 // id: u32
        + 8 // caster: Guid
        + self.layout.size() // layout: SMSG_SPELLLOGMISS_SpellLogMissLayout
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SMSG_SPELLLOGMISS_SpellLogMissLayout {
    Normal {
        targets: Vec<SpellLogMiss>,
    },
    Debug {
        debug_targets: Vec<SpellLogMissDebug>,
    },
}

impl Default for SMSG_SPELLLOGMISS_SpellLogMissLayout {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Normal {
            targets: Default::default(),
        }
    }
}

impl SMSG_SPELLLOGMISS_SpellLogMissLayout {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal { .. } => 0,
            Self::Debug { .. } => 1,
        }
    }

}

impl std::fmt::Display for SMSG_SPELLLOGMISS_SpellLogMissLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal{ .. } => f.write_str("Normal"),
            Self::Debug{ .. } => f.write_str("Debug"),
        }
    }
}

impl SMSG_SPELLLOGMISS_SpellLogMissLayout {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Normal {
                targets,
            } => {
                1
                + 4 // amount_of_targets: u32
                + targets.len() * 9 // targets: SpellLogMiss[amount_of_targets]
            }
            Self::Debug {
                debug_targets,
            } => {
                1
                + 4 // amount_of_debug_targets: u32
                + debug_targets.len() * 17 // debug_targets: SpellLogMissDebug[amount_of_debug_targets]
            }
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SPELLLOGMISS;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_SPELLLOGMISS, expected: &SMSG_SPELLLOGMISS) {
        assert_eq!(t.id, expected.id);
        assert_eq!(t.caster, expected.caster);
        assert_eq!(t.layout, expected.layout);
    }

    const RAW0: [u8; 30] = [ 0x00, 0x1C, 0x4B, 0x02, 0x85, 0x00, 0x00, 0x00, 0x04,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
         0x23, 0x01, 0x00, 0x26, 0x00, 0x00, 0x30, 0xF1, 0x02, ];

    pub(crate) fn expected0() -> SMSG_SPELLLOGMISS {
        SMSG_SPELLLOGMISS {
            id: 0x85,
            caster: Guid::new(0x4),
            layout: SMSG_SPELLLOGMISS_SpellLogMissLayout::Normal {
                targets: vec![
                    SpellLogMiss {
                        target: Guid::new(0xF130000026000123),
                        miss_info: SpellMissInfo::Resist,
                    },
                ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 46.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_spelllogmiss0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 46.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_spelllogmiss0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 46.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_spelllogmiss0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 48] = [ 0x00, 0x2E, 0x4B, 0x02, 0xA9, 0x05, 0x00, 0x00, 0x04,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00,
         0x23, 0x01, 0x00, 0x26, 0x00, 0x00, 0x30, 0xF1, 0x02, 0x24, 0x01, 0x00,
         0x26, 0x00, 0x00, 0x30, 0xF1, 0x06, 0xD2, 0x04, 0x00, 0x2E, 0x00, 0x00,
         0x30, 0xF1, 0x07, ];

    pub(crate) fn expected1() -> SMSG_SPELLLOGMISS {
        SMSG_SPELLLOGMISS {
            id: 0x5A9,
            caster: Guid::new(0x4),
            layout: SMSG_SPELLLOGMISS_SpellLogMissLayout::Normal {
                targets: vec![
                    SpellLogMiss {
                        target: Guid::new(0xF130000026000123),
                        miss_info: SpellMissInfo::Resist,
                    },
                    SpellLogMiss {
                        target: Guid::new(0xF130000026000124),
                        miss_info: SpellMissInfo::Evade,
                    },
                    SpellLogMiss {
                        target: Guid::new(0xF13000002E0004D2),
                        miss_info: SpellMissInfo::Immune,
                    },
                ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 73.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_spelllogmiss1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 73.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_spelllogmiss1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 73.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_spelllogmiss1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 55] = [ 0x00, 0x35, 0x4B, 0x02, 0xA9, 0x05, 0x00, 0x00, 0x04,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00,
         0x23, 0x01, 0x00, 0x26, 0x00, 0x00, 0x30, 0xF1, 0x02, 0x00, 0x00, 0xAF,
         0x42, 0x00, 0x00, 0xA6, 0x42, 0x24, 0x01, 0x00, 0x26, 0x00, 0x00, 0x30,
         0xF1, 0x01, 0x00, 0x00, 0x44, 0x41, 0x00, 0x00, 0xB0, 0x40, ];

    pub(crate) fn expected2() -> SMSG_SPELLLOGMISS {
        SMSG_SPELLLOGMISS {
            id: 0x5A9,
            caster: Guid::new(0x4),
            layout: SMSG_SPELLLOGMISS_SpellLogMissLayout::Debug {
                debug_targets: vec![
                    SpellLogMissDebug {
                        target: Guid::new(0xF130000026000123),
                        miss_info: SpellMissInfo::Resist,
                        hit_roll: 87.5_f32,
                        hit_roll_needed: 83_f32,
                    },
                    SpellLogMissDebug {
                        target: Guid::new(0xF130000026000124),
                        miss_info: SpellMissInfo::Miss,
                        hit_roll: 12.25_f32,
                        hit_roll_needed: 5.5_f32,
                    },
                ],
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 116.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_spelllogmiss2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 116.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_spelllogmiss2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm` line 116.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_spelllogmiss2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SPELLLOGMISS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SPELLLOGMISS, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

}

