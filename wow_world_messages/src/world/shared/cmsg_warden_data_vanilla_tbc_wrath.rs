use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm#L1):
/// ```text
/// cmsg CMSG_WARDEN_DATA = 0x02E7 {
///     u8[-] encrypted_data;
/// }
/// ```
pub struct CMSG_WARDEN_DATA {
    pub encrypted_data: Vec<u8>,
}

impl crate::private::Sealed for CMSG_WARDEN_DATA {}
impl CMSG_WARDEN_DATA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size > 65535 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // encrypted_data: u8[-]
        let encrypted_data = {
            let mut current_size = {
                0
            };
            let mut encrypted_data = Vec::with_capacity((body_size as usize).saturating_sub(current_size));
            while current_size < (body_size as usize) {
                encrypted_data.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            encrypted_data
        };

        Ok(Self {
            encrypted_data,
        })
    }

}

impl crate::Message for CMSG_WARDEN_DATA {
    const OPCODE: u32 = 0x02e7;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_WARDEN_DATA"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // encrypted_data: u8[-]
        for i in self.encrypted_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(743, "CMSG_WARDEN_DATA", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_WARDEN_DATA {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_WARDEN_DATA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_WARDEN_DATA {}

impl CMSG_WARDEN_DATA {
    pub(crate) fn size(&self) -> usize {
        self.encrypted_data.len() * core::mem::size_of::<u8>() // encrypted_data: u8[-]
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_WARDEN_DATA;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_WARDEN_DATA, expected: &CMSG_WARDEN_DATA) {
        assert_eq!(t.encrypted_data, expected.encrypted_data);
    }

    const RAW0: [u8; 11] = [ 0x00, 0x09, 0xE7, 0x02, 0x00, 0x00, 0x01, 0x02, 0x03,
         0x04, 0x05, ];

    pub(crate) fn expected0() -> CMSG_WARDEN_DATA {
        CMSG_WARDEN_DATA {
            encrypted_data: vec![ 0x01, 0x02, 0x03, 0x04, 0x05, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_WARDEN_DATA;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_WARDEN_DATA, expected: &CMSG_WARDEN_DATA) {
        assert_eq!(t.encrypted_data, expected.encrypted_data);
    }

    const RAW0: [u8; 11] = [ 0x00, 0x09, 0xE7, 0x02, 0x00, 0x00, 0x01, 0x02, 0x03,
         0x04, 0x05, ];

    pub(crate) fn expected0() -> CMSG_WARDEN_DATA {
        CMSG_WARDEN_DATA {
            encrypted_data: vec![ 0x01, 0x02, 0x03, 0x04, 0x05, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_WARDEN_DATA;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_WARDEN_DATA, expected: &CMSG_WARDEN_DATA) {
        assert_eq!(t.encrypted_data, expected.encrypted_data);
    }

    const RAW0: [u8; 11] = [ 0x00, 0x09, 0xE7, 0x02, 0x00, 0x00, 0x01, 0x02, 0x03,
         0x04, 0x05, ];

    pub(crate) fn expected0() -> CMSG_WARDEN_DATA {
        CMSG_WARDEN_DATA {
            encrypted_data: vec![ 0x01, 0x02, 0x03, 0x04, 0x05, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/warden/cmsg_warden_data.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_warden_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_WARDEN_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_WARDEN_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

