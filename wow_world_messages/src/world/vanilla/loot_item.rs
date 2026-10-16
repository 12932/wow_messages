use std::io::{Read, Write};

use crate::vanilla::LootSlotType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:87`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L87):
/// ```text
/// struct LootItem {
///     u8 index;
///     u32 item;
///     u32 amount;
///     u32 display_id;
///     u32 unknown;
///     u32 random_property_id;
///     LootSlotType ty;
/// }
/// ```
pub struct LootItem {
    pub index: u8,
    pub item: u32,
    pub amount: u32,
    pub display_id: u32,
    pub unknown: u32,
    pub random_property_id: u32,
    pub ty: LootSlotType,
}

impl LootItem {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // index: u8
        w.write_all(&self.index.to_le_bytes())?;

        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // random_property_id: u32
        w.write_all(&self.random_property_id.to_le_bytes())?;

        // ty: LootSlotType
        w.write_all(&(self.ty.as_int().to_le_bytes()))?;

        Ok(())
    }
}

impl LootItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // index: u8
        let index = crate::util::read_u8_le(&mut r)?;

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        // amount: u32
        let amount = crate::util::read_u32_le(&mut r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // random_property_id: u32
        let random_property_id = crate::util::read_u32_le(&mut r)?;

        // ty: LootSlotType
        let ty = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            index,
            item,
            amount,
            display_id,
            unknown,
            random_property_id,
            ty,
        })
    }

}

