use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::SpellMissInfo;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L17):
/// ```text
/// struct SpellLogMissDebug {
///     Guid target;
///     SpellMissInfo miss_info;
///     f32 hit_roll;
///     f32 hit_roll_needed;
/// }
/// ```
pub struct SpellLogMissDebug {
    pub target: Guid,
    pub miss_info: SpellMissInfo,
    /// The 1.12 client hands this and `hit_roll_needed` only to a log call that does nothing in its release build. Named as TC master's `SpellLogMissDebug` (a later client's packet) names them.
    pub hit_roll: f32,
    pub hit_roll_needed: f32,
}

impl SpellLogMissDebug {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // miss_info: SpellMissInfo
        w.write_all(&(self.miss_info.as_int().to_le_bytes()))?;

        // hit_roll: f32
        w.write_all(&self.hit_roll.to_le_bytes())?;

        // hit_roll_needed: f32
        w.write_all(&self.hit_roll_needed.to_le_bytes())?;

        Ok(())
    }
}

impl SpellLogMissDebug {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // miss_info: SpellMissInfo
        let miss_info = crate::util::read_u8_le(&mut r)?.try_into()?;

        // hit_roll: f32
        let hit_roll = crate::util::read_f32_le(&mut r)?;

        // hit_roll_needed: f32
        let hit_roll_needed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            target,
            miss_info,
            hit_roll,
            hit_roll_needed,
        })
    }

}

