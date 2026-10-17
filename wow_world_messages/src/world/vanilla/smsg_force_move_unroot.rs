use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm#L1):
/// ```text
/// smsg SMSG_FORCE_MOVE_UNROOT = 0x00EA {
///     PackedGuid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_FORCE_MOVE_UNROOT {
    pub guid: Guid,
    pub counter: u32,
}

impl crate::private::Sealed for SMSG_FORCE_MOVE_UNROOT {}
impl SMSG_FORCE_MOVE_UNROOT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

}

impl crate::Message for SMSG_FORCE_MOVE_UNROOT {
    const OPCODE: u32 = 0x00ea;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_FORCE_MOVE_UNROOT"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(234, "SMSG_FORCE_MOVE_UNROOT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_FORCE_MOVE_UNROOT {}

impl SMSG_FORCE_MOVE_UNROOT {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // counter: u32
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_FORCE_MOVE_UNROOT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_FORCE_MOVE_UNROOT, expected: &SMSG_FORCE_MOVE_UNROOT) {
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.counter, expected.counter);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xEA, 0x00, 0x01, 0x04, 0x00, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> SMSG_FORCE_MOVE_UNROOT {
        SMSG_FORCE_MOVE_UNROOT {
            guid: Guid::new(0x4),
            counter: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_force_move_unroot0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_FORCE_MOVE_UNROOT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_FORCE_MOVE_UNROOT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_force_move_unroot0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_FORCE_MOVE_UNROOT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_FORCE_MOVE_UNROOT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_force_move_unroot0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_FORCE_MOVE_UNROOT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_FORCE_MOVE_UNROOT, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

