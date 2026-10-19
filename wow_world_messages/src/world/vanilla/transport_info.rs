use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::Vector3d;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// The 1.12.1 client's own reader (0x47EBA0) reads a transport's whole GUID and the place and facing on it, with no timestamp, as mangos-classic's MovementInfo::Read and Write have it.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement.wowm#L3):
/// ```text
/// struct TransportInfo {
///     Guid guid;
///     Vector3d position;
///     f32 orientation;
/// }
/// ```
pub struct TransportInfo {
    pub guid: Guid,
    pub position: Vector3d,
    pub orientation: f32,
}

impl TransportInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // position: Vector3d
crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // orientation: f32
        w.write_all(&self.orientation.to_le_bytes())?;

        Ok(())
    }
}

impl TransportInfo {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // orientation: f32
        let orientation = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            position,
            orientation,
        })
    }

}

