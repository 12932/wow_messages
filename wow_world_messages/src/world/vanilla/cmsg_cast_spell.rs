use std::io::{Read, Write};

use crate::vanilla::SpellCastTargets;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm#L1):
/// ```text
/// cmsg CMSG_CAST_SPELL = 0x012E {
///     u32 spell;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_CAST_SPELL {
    pub spell: u32,
    pub targets: SpellCastTargets,
}

impl crate::private::Sealed for CMSG_CAST_SPELL {}
impl CMSG_CAST_SPELL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(6..=304).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(&mut r)?;

        Ok(Self {
            spell,
            targets,
        })
    }

}

impl crate::Message for CMSG_CAST_SPELL {
    const OPCODE: u32 = 0x012e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CAST_SPELL"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(302, "CMSG_CAST_SPELL", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_CAST_SPELL {}

impl CMSG_CAST_SPELL {
    pub(crate) fn size(&self) -> usize {
        4 // spell: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_CAST_SPELL;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_CAST_SPELL, expected: &CMSG_CAST_SPELL) {
        assert_eq!(t.spell, expected.spell);
        assert_eq!(t.targets, expected.targets);
    }

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0x2E, 0x01, 0x00, 0x00, 0x78, 0x50, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_CAST_SPELL {
        CMSG_CAST_SPELL {
            spell: 0x5078,
            targets: SpellCastTargets {
                target_flags: SpellCastTargets_SpellCastTargetFlags::empty()
                    ,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 27.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cast_spell0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 27.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cast_spell0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 27.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cast_spell0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 27] = [ 0x00, 0x19, 0x2E, 0x01, 0x00, 0x00, 0x78, 0x50, 0x00,
         0x00, 0x40, 0x02, 0x81, 0x2A, 0xF1, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00,
         0x00, 0x40, 0x00, 0x00, 0x40, 0x40, ];

    pub(crate) fn expected1() -> CMSG_CAST_SPELL {
        CMSG_CAST_SPELL {
            spell: 0x5078,
            targets: SpellCastTargets {
                target_flags: SpellCastTargets_SpellCastTargetFlags::empty()
                    .set_unit(SpellCastTargets_SpellCastTargetFlags_Unit::PvpCorpse {
                        pvp_corpse: Guid::new(0xF10000000000002A),
                    })
                    .set_dest_location(SpellCastTargets_SpellCastTargetFlags_DestLocation {
                        destination: Vector3d {
                            x: 1_f32,
                            y: 2_f32,
                            z: 3_f32,
                        },
                    })
                    ,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 45.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cast_spell1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 45.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cast_spell1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 45.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cast_spell1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 27] = [ 0x00, 0x19, 0x2E, 0x01, 0x00, 0x00, 0x78, 0x50, 0x00,
         0x00, 0x40, 0x80, 0x81, 0x2A, 0xF1, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00,
         0x00, 0x40, 0x00, 0x00, 0x40, 0x40, ];

    pub(crate) fn expected2() -> CMSG_CAST_SPELL {
        CMSG_CAST_SPELL {
            spell: 0x5078,
            targets: SpellCastTargets {
                target_flags: SpellCastTargets_SpellCastTargetFlags::empty()
                    .set_unit(SpellCastTargets_SpellCastTargetFlags_Unit::Corpse {
                        corpse: Guid::new(0xF10000000000002A),
                    })
                    .set_dest_location(SpellCastTargets_SpellCastTargetFlags_DestLocation {
                        destination: Vector3d {
                            x: 1_f32,
                            y: 2_f32,
                            z: 3_f32,
                        },
                    })
                    ,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 74.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cast_spell2() {
        let expected = expected2();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 74.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cast_spell2() {
        let expected = expected2();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 74.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cast_spell2() {
        let expected = expected2();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 21] = [ 0x00, 0x13, 0x2E, 0x01, 0x00, 0x00, 0x78, 0x50, 0x00,
         0x00, 0x12, 0x00, 0xCB, 0x23, 0x01, 0x26, 0x30, 0xF1, 0x81, 0x07, 0x40, ];

    pub(crate) fn expected3() -> CMSG_CAST_SPELL {
        CMSG_CAST_SPELL {
            spell: 0x5078,
            targets: SpellCastTargets {
                target_flags: SpellCastTargets_SpellCastTargetFlags::empty()
                    .set_unit(SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                        unit_target: Guid::new(0xF130000026000123),
                    })
                    .set_item(SpellCastTargets_SpellCastTargetFlags_Item::Item {
                        item: Guid::new(0x4000000000000007),
                    })
                    ,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 103.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cast_spell3() {
        let expected = expected3();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 103.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cast_spell3() {
        let expected = expected3();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 103.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cast_spell3() {
        let expected = expected3();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    const RAW4: [u8; 18] = [ 0x00, 0x10, 0x2E, 0x01, 0x00, 0x00, 0x78, 0x50, 0x00,
         0x00, 0x82, 0x40, 0xCB, 0x23, 0x01, 0x26, 0x30, 0xF1, ];

    pub(crate) fn expected4() -> CMSG_CAST_SPELL {
        CMSG_CAST_SPELL {
            spell: 0x5078,
            targets: SpellCastTargets {
                target_flags: SpellCastTargets_SpellCastTargetFlags::empty()
                    .set_unit(SpellCastTargets_SpellCastTargetFlags_Unit::Unit {
                        unit_target: Guid::new(0xF130000026000123),
                    })
                    .set_object_unk()
                    .set_unk1()
                    ,
            },
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 124.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_cast_spell4() {
        let expected = expected4();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW4)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 124.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_cast_spell4() {
        let expected = expected4();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_cast_spell.wowm` line 124.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_cast_spell4() {
        let expected = expected4();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW4)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_CAST_SPELL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_CAST_SPELL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW4.len());

        let mut dest = Vec::with_capacity(RAW4.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW4);
    }

}

