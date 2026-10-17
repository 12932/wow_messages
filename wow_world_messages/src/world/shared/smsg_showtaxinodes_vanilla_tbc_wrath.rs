use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm#L3):
/// ```text
/// smsg SMSG_SHOWTAXINODES = 0x01A9 {
///     u32 unknown1;
///     Guid guid;
///     u32 nearest_node;
///     u32[-] nodes;
/// }
/// ```
pub struct SMSG_SHOWTAXINODES {
    /// Set to 1 in mangoszero
    pub unknown1: u32,
    pub guid: Guid,
    pub nearest_node: u32,
    pub nodes: Vec<u32>,
}

impl crate::private::Sealed for SMSG_SHOWTAXINODES {}
impl SMSG_SHOWTAXINODES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=65551).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // nearest_node: u32
        let nearest_node = crate::util::read_u32_le(&mut r)?;

        // nodes: u32[-]
        let nodes = {
            let mut current_size = {
                4 // unknown1: u32
                + 8 // guid: Guid
                + 4 // nearest_node: u32
            };
            let mut nodes = Vec::with_capacity(r.len() / 4);
            while current_size < (body_size as usize) {
                nodes.push(crate::util::read_u32_le(&mut r)?);
                current_size += 4;
            }
            nodes
        };

        Ok(Self {
            unknown1,
            guid,
            nearest_node,
            nodes,
        })
    }

}

impl crate::Message for SMSG_SHOWTAXINODES {
    const OPCODE: u32 = 0x01a9;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SHOWTAXINODES"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // nearest_node: u32
        w.write_all(&self.nearest_node.to_le_bytes())?;

        // nodes: u32[-]
        for i in self.nodes.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(425, "SMSG_SHOWTAXINODES", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SHOWTAXINODES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SHOWTAXINODES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SHOWTAXINODES {}

impl SMSG_SHOWTAXINODES {
    pub(crate) fn size(&self) -> usize {
        4 // unknown1: u32
        + 8 // guid: Guid
        + 4 // nearest_node: u32
        + self.nodes.len() * core::mem::size_of::<u32>() // nodes: u32[-]
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SHOWTAXINODES;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_SHOWTAXINODES, expected: &SMSG_SHOWTAXINODES) {
        assert_eq!(t.unknown1, expected.unknown1);
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.nearest_node, expected.nearest_node);
        assert_eq!(t.nodes, expected.nodes);
    }

    const RAW0: [u8; 52] = [ 0x00, 0x32, 0xA9, 0x01, 0x01, 0x00, 0x00, 0x00, 0x10,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x30, 0xF1, 0x02, 0x00, 0x00, 0x00, 0x06,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_SHOWTAXINODES {
        SMSG_SHOWTAXINODES {
            unknown1: 0x1,
            guid: Guid::new(0xF130000001000010),
            nearest_node: 0x2,
            nodes: vec![ 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SHOWTAXINODES;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_SHOWTAXINODES, expected: &SMSG_SHOWTAXINODES) {
        assert_eq!(t.unknown1, expected.unknown1);
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.nearest_node, expected.nearest_node);
        assert_eq!(t.nodes, expected.nodes);
    }

    const RAW0: [u8; 52] = [ 0x00, 0x32, 0xA9, 0x01, 0x01, 0x00, 0x00, 0x00, 0x10,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x30, 0xF1, 0x02, 0x00, 0x00, 0x00, 0x06,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_SHOWTAXINODES {
        SMSG_SHOWTAXINODES {
            unknown1: 0x1,
            guid: Guid::new(0xF130000001000010),
            nearest_node: 0x2,
            nodes: vec![ 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_SHOWTAXINODES;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_SHOWTAXINODES, expected: &SMSG_SHOWTAXINODES) {
        assert_eq!(t.unknown1, expected.unknown1);
        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.nearest_node, expected.nearest_node);
        assert_eq!(t.nodes, expected.nodes);
    }

    const RAW0: [u8; 52] = [ 0x00, 0x32, 0xA9, 0x01, 0x01, 0x00, 0x00, 0x00, 0x10,
         0x00, 0x00, 0x01, 0x00, 0x00, 0x30, 0xF1, 0x02, 0x00, 0x00, 0x00, 0x06,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_SHOWTAXINODES {
        SMSG_SHOWTAXINODES {
            unknown1: 0x1,
            guid: Guid::new(0xF130000001000010),
            nearest_node: 0x2,
            nodes: vec![ 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_showtaxinodes.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_showtaxinodes0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_SHOWTAXINODES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_SHOWTAXINODES, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

