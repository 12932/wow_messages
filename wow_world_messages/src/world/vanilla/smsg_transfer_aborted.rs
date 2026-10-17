use std::io::{Read, Write};

use crate::vanilla::TransferAbortReason;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// mangoszero and cmangos-classic (`WorldSession::SendTransferAborted`) send the reason alone.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm#L14):
/// ```text
/// smsg SMSG_TRANSFER_ABORTED = 0x0040 {
///     TransferAbortReason reason;
/// }
/// ```
pub struct SMSG_TRANSFER_ABORTED {
    pub reason: TransferAbortReason,
}

impl crate::private::Sealed for SMSG_TRANSFER_ABORTED {}
impl SMSG_TRANSFER_ABORTED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // reason: TransferAbortReason
        let reason = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}

impl crate::Message for SMSG_TRANSFER_ABORTED {
    const OPCODE: u32 = 0x0040;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_TRANSFER_ABORTED"
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reason: TransferAbortReason
        w.write_all(&(self.reason.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(64, "SMSG_TRANSFER_ABORTED", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TRANSFER_ABORTED {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_TRANSFER_ABORTED;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_TRANSFER_ABORTED, expected: &SMSG_TRANSFER_ABORTED) {
        assert_eq!(t.reason, expected.reason);
    }

    const RAW0: [u8; 5] = [ 0x00, 0x03, 0x40, 0x00, 0x01, ];

    pub(crate) fn expected0() -> SMSG_TRANSFER_ABORTED {
        SMSG_TRANSFER_ABORTED {
            reason: TransferAbortReason::IsFull,
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm` line 21.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_transfer_aborted0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TRANSFER_ABORTED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TRANSFER_ABORTED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm` line 21.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_transfer_aborted0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TRANSFER_ABORTED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TRANSFER_ABORTED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/smsg/smsg_transfer_aborted.wowm` line 21.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_transfer_aborted0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TRANSFER_ABORTED(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TRANSFER_ABORTED, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(1 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

