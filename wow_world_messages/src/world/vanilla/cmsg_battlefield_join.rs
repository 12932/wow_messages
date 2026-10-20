use std::io::{Read, Write};

use crate::vanilla::Map;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// The 1.12.1 client (0x4AA00B) writes this as `CMSG_BATTLEMASTER_JOIN` without its battlemaster's guid: when it joins from a list shown with no battlemaster.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm#L3):
/// ```text
/// cmsg CMSG_BATTLEFIELD_JOIN = 0x023E {
///     Map map;
///     u32 instance_id;
///     Bool join_as_group;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_JOIN {
    pub map: Map,
    /// The instance picked from the battlefield list, 0 for the first available.
    pub instance_id: u32,
    pub join_as_group: bool,
}

impl crate::private::Sealed for CMSG_BATTLEFIELD_JOIN {}
impl CMSG_BATTLEFIELD_JOIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 9 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // instance_id: u32
        let instance_id = crate::util::read_u32_le(&mut r)?;

        // join_as_group: Bool
        let join_as_group = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            map,
            instance_id,
            join_as_group,
        })
    }

}

impl crate::Message for CMSG_BATTLEFIELD_JOIN {
    const OPCODE: u32 = 0x023e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_BATTLEFIELD_JOIN"
    }

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // instance_id: u32
        w.write_all(&self.instance_id.to_le_bytes())?;

        // join_as_group: Bool
        w.write_all(u8::from(self.join_as_group).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(574, "CMSG_BATTLEFIELD_JOIN", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_BATTLEFIELD_JOIN {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_BATTLEFIELD_JOIN;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_BATTLEFIELD_JOIN, expected: &CMSG_BATTLEFIELD_JOIN) {
        assert_eq!(t.map, expected.map);
        assert_eq!(t.instance_id, expected.instance_id);
        assert_eq!(t.join_as_group, expected.join_as_group);
    }

    const RAW0: [u8; 15] = [ 0x00, 0x0D, 0x3E, 0x02, 0x00, 0x00, 0x11, 0x02, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x01, ];

    pub(crate) fn expected0() -> CMSG_BATTLEFIELD_JOIN {
        CMSG_BATTLEFIELD_JOIN {
            map: Map::ArathiBasin,
            instance_id: 0x0,
            join_as_group: true, 
        }

    }

    // Generated from `wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_battlefield_join0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BATTLEFIELD_JOIN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(9 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_battlefield_join0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BATTLEFIELD_JOIN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(9 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_battlefield_join0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BATTLEFIELD_JOIN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(9 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 15] = [ 0x00, 0x0D, 0x3E, 0x02, 0x00, 0x00, 0xE9, 0x01, 0x00,
         0x00, 0x03, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected1() -> CMSG_BATTLEFIELD_JOIN {
        CMSG_BATTLEFIELD_JOIN {
            map: Map::WarsongGulch,
            instance_id: 0x3,
            join_as_group: false, 
        }

    }

    // Generated from `wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm` line 25.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_battlefield_join1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BATTLEFIELD_JOIN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(9 + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm` line 25.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_battlefield_join1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BATTLEFIELD_JOIN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(9 + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/battleground/cmsg_battlefield_join.wowm` line 25.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_battlefield_join1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_BATTLEFIELD_JOIN(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_BATTLEFIELD_JOIN, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(9 + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

