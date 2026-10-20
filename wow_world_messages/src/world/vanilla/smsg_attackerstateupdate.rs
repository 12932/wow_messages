use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    DamageInfo, HitInfo,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// mangos-classic's `Unit::SendAttackStateUpdate` writes the debug trailer (a u32, eight floats, five pairs of floats and a u32, all 0) under `HITINFO_UNK0`, and the 1.12 client's parse (0x625C60) reads it under the same flag.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:88`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L88):
/// ```text
/// smsg SMSG_ATTACKERSTATEUPDATE = 0x014A {
///     HitInfo hit_info;
///     PackedGuid attacker;
///     PackedGuid target;
///     u32 total_damage;
///     u8 amount_of_damages;
///     DamageInfo[amount_of_damages] damages;
///     u32 damage_state;
///     u32 unknown1;
///     u32 spell_id;
///     u32 blocked_amount;
///     if (hit_info & UNK1) {
///         u32 debug_int1;
///         f32 debug_float1;
///         f32 debug_float2;
///         f32 debug_float3;
///         f32 debug_float4;
///         f32 debug_float5;
///         f32 debug_float6;
///         f32 debug_float7;
///         f32 debug_float8;
///         f32 debug_float9;
///         f32 debug_float10;
///         f32 debug_float11;
///         f32 debug_float12;
///         f32 debug_float13;
///         f32 debug_float14;
///         f32 debug_float15;
///         f32 debug_float16;
///         f32 debug_float17;
///         f32 debug_float18;
///         u32 debug_int2;
///     }
/// }
/// ```
pub struct SMSG_ATTACKERSTATEUPDATE {
    pub hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo,
    pub attacker: Guid,
    pub target: Guid,
    pub total_damage: u32,
    pub damages: Vec<DamageInfo>,
    pub damage_state: u32,
    pub unknown1: u32,
    /// vmangos: spell id, seen with heroic strike and disarm as examples
    pub spell_id: u32,
    pub blocked_amount: u32,
}

impl crate::private::Sealed for SMSG_ATTACKERSTATEUPDATE {}
impl SMSG_ATTACKERSTATEUPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(27..=5243).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // hit_info: HitInfo
        let hit_info = HitInfo::new(crate::util::read_u32_le(&mut r)?);

        // attacker: PackedGuid
        let attacker = crate::util::read_packed_guid(&mut r)?;

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // total_damage: u32
        let total_damage = crate::util::read_u32_le(&mut r)?;

        // amount_of_damages: u8
        let amount_of_damages = crate::util::read_u8_le(&mut r)?;

        // damages: DamageInfo[amount_of_damages]
        let damages = {
            let mut damages = Vec::with_capacity(crate::util::capacity(amount_of_damages as usize, r.len(), 20));
            for _ in 0..amount_of_damages {
                damages.push(DamageInfo::read(&mut r)?);
            }
            damages
        };

        // damage_state: u32
        let damage_state = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // spell_id: u32
        let spell_id = crate::util::read_u32_le(&mut r)?;

        // blocked_amount: u32
        let blocked_amount = crate::util::read_u32_le(&mut r)?;

        let hit_info_unk1 = if hit_info.is_unk1() {
            // debug_int1: u32
            let debug_int1 = crate::util::read_u32_le(&mut r)?;

            // debug_float1: f32
            let debug_float1 = crate::util::read_f32_le(&mut r)?;

            // debug_float2: f32
            let debug_float2 = crate::util::read_f32_le(&mut r)?;

            // debug_float3: f32
            let debug_float3 = crate::util::read_f32_le(&mut r)?;

            // debug_float4: f32
            let debug_float4 = crate::util::read_f32_le(&mut r)?;

            // debug_float5: f32
            let debug_float5 = crate::util::read_f32_le(&mut r)?;

            // debug_float6: f32
            let debug_float6 = crate::util::read_f32_le(&mut r)?;

            // debug_float7: f32
            let debug_float7 = crate::util::read_f32_le(&mut r)?;

            // debug_float8: f32
            let debug_float8 = crate::util::read_f32_le(&mut r)?;

            // debug_float9: f32
            let debug_float9 = crate::util::read_f32_le(&mut r)?;

            // debug_float10: f32
            let debug_float10 = crate::util::read_f32_le(&mut r)?;

            // debug_float11: f32
            let debug_float11 = crate::util::read_f32_le(&mut r)?;

            // debug_float12: f32
            let debug_float12 = crate::util::read_f32_le(&mut r)?;

            // debug_float13: f32
            let debug_float13 = crate::util::read_f32_le(&mut r)?;

            // debug_float14: f32
            let debug_float14 = crate::util::read_f32_le(&mut r)?;

            // debug_float15: f32
            let debug_float15 = crate::util::read_f32_le(&mut r)?;

            // debug_float16: f32
            let debug_float16 = crate::util::read_f32_le(&mut r)?;

            // debug_float17: f32
            let debug_float17 = crate::util::read_f32_le(&mut r)?;

            // debug_float18: f32
            let debug_float18 = crate::util::read_f32_le(&mut r)?;

            // debug_int2: u32
            let debug_int2 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1 {
                debug_float1,
                debug_float10,
                debug_float11,
                debug_float12,
                debug_float13,
                debug_float14,
                debug_float15,
                debug_float16,
                debug_float17,
                debug_float18,
                debug_float2,
                debug_float3,
                debug_float4,
                debug_float5,
                debug_float6,
                debug_float7,
                debug_float8,
                debug_float9,
                debug_int1,
                debug_int2,
            })
        }
        else {
            None
        };

        let hit_info = SMSG_ATTACKERSTATEUPDATE_HitInfo {
            inner: hit_info.as_int(),
            unk1: hit_info_unk1,
        };

        Ok(Self {
            hit_info,
            attacker,
            target,
            total_damage,
            damages,
            damage_state,
            unknown1,
            spell_id,
            blocked_amount,
        })
    }

}

impl crate::Message for SMSG_ATTACKERSTATEUPDATE {
    const OPCODE: u32 = 0x014a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ATTACKERSTATEUPDATE"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // hit_info: HitInfo
        w.write_all(&(self.hit_info.as_int().to_le_bytes()))?;

        // attacker: PackedGuid
        crate::util::write_packed_guid(&self.attacker, &mut w)?;

        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // total_damage: u32
        w.write_all(&self.total_damage.to_le_bytes())?;

        // amount_of_damages: u8
        w.write_all(&(self.damages.len() as u8).to_le_bytes())?;

        // damages: DamageInfo[amount_of_damages]
        for i in self.damages.iter() {
            i.write_into_vec(&mut w)?;
        }

        // damage_state: u32
        w.write_all(&self.damage_state.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // spell_id: u32
        w.write_all(&self.spell_id.to_le_bytes())?;

        // blocked_amount: u32
        w.write_all(&self.blocked_amount.to_le_bytes())?;

        if let Some(if_statement) = &self.hit_info.unk1 {
            // debug_int1: u32
            w.write_all(&if_statement.debug_int1.to_le_bytes())?;

            // debug_float1: f32
            w.write_all(&if_statement.debug_float1.to_le_bytes())?;

            // debug_float2: f32
            w.write_all(&if_statement.debug_float2.to_le_bytes())?;

            // debug_float3: f32
            w.write_all(&if_statement.debug_float3.to_le_bytes())?;

            // debug_float4: f32
            w.write_all(&if_statement.debug_float4.to_le_bytes())?;

            // debug_float5: f32
            w.write_all(&if_statement.debug_float5.to_le_bytes())?;

            // debug_float6: f32
            w.write_all(&if_statement.debug_float6.to_le_bytes())?;

            // debug_float7: f32
            w.write_all(&if_statement.debug_float7.to_le_bytes())?;

            // debug_float8: f32
            w.write_all(&if_statement.debug_float8.to_le_bytes())?;

            // debug_float9: f32
            w.write_all(&if_statement.debug_float9.to_le_bytes())?;

            // debug_float10: f32
            w.write_all(&if_statement.debug_float10.to_le_bytes())?;

            // debug_float11: f32
            w.write_all(&if_statement.debug_float11.to_le_bytes())?;

            // debug_float12: f32
            w.write_all(&if_statement.debug_float12.to_le_bytes())?;

            // debug_float13: f32
            w.write_all(&if_statement.debug_float13.to_le_bytes())?;

            // debug_float14: f32
            w.write_all(&if_statement.debug_float14.to_le_bytes())?;

            // debug_float15: f32
            w.write_all(&if_statement.debug_float15.to_le_bytes())?;

            // debug_float16: f32
            w.write_all(&if_statement.debug_float16.to_le_bytes())?;

            // debug_float17: f32
            w.write_all(&if_statement.debug_float17.to_le_bytes())?;

            // debug_float18: f32
            w.write_all(&if_statement.debug_float18.to_le_bytes())?;

            // debug_int2: u32
            w.write_all(&if_statement.debug_int2.to_le_bytes())?;

        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(330, "SMSG_ATTACKERSTATEUPDATE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ATTACKERSTATEUPDATE {}

impl SMSG_ATTACKERSTATEUPDATE {
    pub(crate) fn size(&self) -> usize {
        self.hit_info.size() // hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo
        + crate::util::packed_guid_size(&self.attacker) // attacker: PackedGuid
        + crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 4 // total_damage: u32
        + 1 // amount_of_damages: u8
        + self.damages.len() * 20 // damages: DamageInfo[amount_of_damages]
        + 4 // damage_state: u32
        + 4 // unknown1: u32
        + 4 // spell_id: u32
        + 4 // blocked_amount: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo {
    inner: u32,
    unk1: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1>,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo {
    pub const fn new(inner: u32, unk1: Option<SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1>,) -> Self {
        Self {
            inner,
            unk1, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            unk1: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.unk1.is_none()
    }

    pub const fn new_unk1(unk1: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1) -> Self {
        Self {
            inner: HitInfo::UNK1,
            unk1: Some(unk1),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk1(mut self, unk1: SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1) -> Self {
        self.inner |= HitInfo::UNK1;
        self.unk1 = Some(unk1);
        self
    }

    pub const fn get_unk1(&self) -> Option<&SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1> {
        self.unk1.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk1(mut self) -> Self {
        self.inner &= !HitInfo::UNK1;
        self.unk1 = None;
        self
    }

    pub const fn new_affects_victim() -> Self {
        Self {
            inner: HitInfo::AFFECTS_VICTIM,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_affects_victim(mut self) -> Self {
        self.inner |= HitInfo::AFFECTS_VICTIM;
        self
    }

    pub const fn get_affects_victim(&self) -> bool {
        (self.inner & HitInfo::AFFECTS_VICTIM) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_affects_victim(mut self) -> Self {
        self.inner &= !HitInfo::AFFECTS_VICTIM;
        self
    }

    pub const fn new_left_swing() -> Self {
        Self {
            inner: HitInfo::LEFT_SWING,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_left_swing(mut self) -> Self {
        self.inner |= HitInfo::LEFT_SWING;
        self
    }

    pub const fn get_left_swing(&self) -> bool {
        (self.inner & HitInfo::LEFT_SWING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_left_swing(mut self) -> Self {
        self.inner &= !HitInfo::LEFT_SWING;
        self
    }

    pub const fn new_early_critical_hit() -> Self {
        Self {
            inner: HitInfo::EARLY_CRITICAL_HIT,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_early_critical_hit(mut self) -> Self {
        self.inner |= HitInfo::EARLY_CRITICAL_HIT;
        self
    }

    pub const fn get_early_critical_hit(&self) -> bool {
        (self.inner & HitInfo::EARLY_CRITICAL_HIT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_early_critical_hit(mut self) -> Self {
        self.inner &= !HitInfo::EARLY_CRITICAL_HIT;
        self
    }

    pub const fn new_miss() -> Self {
        Self {
            inner: HitInfo::MISS,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_miss(mut self) -> Self {
        self.inner |= HitInfo::MISS;
        self
    }

    pub const fn get_miss(&self) -> bool {
        (self.inner & HitInfo::MISS) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_miss(mut self) -> Self {
        self.inner &= !HitInfo::MISS;
        self
    }

    pub const fn new_absorb() -> Self {
        Self {
            inner: HitInfo::ABSORB,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_absorb(mut self) -> Self {
        self.inner |= HitInfo::ABSORB;
        self
    }

    pub const fn get_absorb(&self) -> bool {
        (self.inner & HitInfo::ABSORB) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_absorb(mut self) -> Self {
        self.inner &= !HitInfo::ABSORB;
        self
    }

    pub const fn new_resist() -> Self {
        Self {
            inner: HitInfo::RESIST,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_resist(mut self) -> Self {
        self.inner |= HitInfo::RESIST;
        self
    }

    pub const fn get_resist(&self) -> bool {
        (self.inner & HitInfo::RESIST) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_resist(mut self) -> Self {
        self.inner &= !HitInfo::RESIST;
        self
    }

    pub const fn new_critical_hit() -> Self {
        Self {
            inner: HitInfo::CRITICAL_HIT,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_critical_hit(mut self) -> Self {
        self.inner |= HitInfo::CRITICAL_HIT;
        self
    }

    pub const fn get_critical_hit(&self) -> bool {
        (self.inner & HitInfo::CRITICAL_HIT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_critical_hit(mut self) -> Self {
        self.inner &= !HitInfo::CRITICAL_HIT;
        self
    }

    pub const fn new_unk9() -> Self {
        Self {
            inner: HitInfo::UNK9,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk9(mut self) -> Self {
        self.inner |= HitInfo::UNK9;
        self
    }

    pub const fn get_unk9(&self) -> bool {
        (self.inner & HitInfo::UNK9) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk9(mut self) -> Self {
        self.inner &= !HitInfo::UNK9;
        self
    }

    pub const fn new_block() -> Self {
        Self {
            inner: HitInfo::BLOCK,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_block(mut self) -> Self {
        self.inner |= HitInfo::BLOCK;
        self
    }

    pub const fn get_block(&self) -> bool {
        (self.inner & HitInfo::BLOCK) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_block(mut self) -> Self {
        self.inner &= !HitInfo::BLOCK;
        self
    }

    pub const fn new_unk10() -> Self {
        Self {
            inner: HitInfo::UNK10,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unk10(mut self) -> Self {
        self.inner |= HitInfo::UNK10;
        self
    }

    pub const fn get_unk10(&self) -> bool {
        (self.inner & HitInfo::UNK10) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unk10(mut self) -> Self {
        self.inner &= !HitInfo::UNK10;
        self
    }

    pub const fn new_glancing() -> Self {
        Self {
            inner: HitInfo::GLANCING,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_glancing(mut self) -> Self {
        self.inner |= HitInfo::GLANCING;
        self
    }

    pub const fn get_glancing(&self) -> bool {
        (self.inner & HitInfo::GLANCING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_glancing(mut self) -> Self {
        self.inner &= !HitInfo::GLANCING;
        self
    }

    pub const fn new_crushing() -> Self {
        Self {
            inner: HitInfo::CRUSHING,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_crushing(mut self) -> Self {
        self.inner |= HitInfo::CRUSHING;
        self
    }

    pub const fn get_crushing(&self) -> bool {
        (self.inner & HitInfo::CRUSHING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_crushing(mut self) -> Self {
        self.inner &= !HitInfo::CRUSHING;
        self
    }

    pub const fn new_no_action() -> Self {
        Self {
            inner: HitInfo::NO_ACTION,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_action(mut self) -> Self {
        self.inner |= HitInfo::NO_ACTION;
        self
    }

    pub const fn get_no_action(&self) -> bool {
        (self.inner & HitInfo::NO_ACTION) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_action(mut self) -> Self {
        self.inner &= !HitInfo::NO_ACTION;
        self
    }

    pub const fn new_swing_no_hit_sound() -> Self {
        Self {
            inner: HitInfo::SWING_NO_HIT_SOUND,
            unk1: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_swing_no_hit_sound(mut self) -> Self {
        self.inner |= HitInfo::SWING_NO_HIT_SOUND;
        self
    }

    pub const fn get_swing_no_hit_sound(&self) -> bool {
        (self.inner & HitInfo::SWING_NO_HIT_SOUND) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_swing_no_hit_sound(mut self) -> Self {
        self.inner &= !HitInfo::SWING_NO_HIT_SOUND;
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_ATTACKERSTATEUPDATE_HitInfo {
    pub(crate) const fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.unk1 {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1 {
    pub debug_float1: f32,
    pub debug_float10: f32,
    pub debug_float11: f32,
    pub debug_float12: f32,
    pub debug_float13: f32,
    pub debug_float14: f32,
    pub debug_float15: f32,
    pub debug_float16: f32,
    pub debug_float17: f32,
    pub debug_float18: f32,
    pub debug_float2: f32,
    pub debug_float3: f32,
    pub debug_float4: f32,
    pub debug_float5: f32,
    pub debug_float6: f32,
    pub debug_float7: f32,
    pub debug_float8: f32,
    pub debug_float9: f32,
    pub debug_int1: u32,
    pub debug_int2: u32,
}

impl SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1 {
    pub(crate) const fn size(&self) -> usize {
        4 // debug_float1: f32
        + 4 // debug_float10: f32
        + 4 // debug_float11: f32
        + 4 // debug_float12: f32
        + 4 // debug_float13: f32
        + 4 // debug_float14: f32
        + 4 // debug_float15: f32
        + 4 // debug_float16: f32
        + 4 // debug_float17: f32
        + 4 // debug_float18: f32
        + 4 // debug_float2: f32
        + 4 // debug_float3: f32
        + 4 // debug_float4: f32
        + 4 // debug_float5: f32
        + 4 // debug_float6: f32
        + 4 // debug_float7: f32
        + 4 // debug_float8: f32
        + 4 // debug_float9: f32
        + 4 // debug_int1: u32
        + 4 // debug_int2: u32
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_ATTACKERSTATEUPDATE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_ATTACKERSTATEUPDATE, expected: &SMSG_ATTACKERSTATEUPDATE) {
        assert_eq!(t.hit_info, expected.hit_info);
        assert_eq!(t.attacker, expected.attacker);
        assert_eq!(t.target, expected.target);
        assert_eq!(t.total_damage, expected.total_damage);
        assert_eq!(t.damages, expected.damages);
        assert_eq!(t.damage_state, expected.damage_state);
        assert_eq!(t.unknown1, expected.unknown1);
        assert_eq!(t.spell_id, expected.spell_id);
        assert_eq!(t.blocked_amount, expected.blocked_amount);
    }

    const RAW0: [u8; 53] = [ 0x00, 0x33, 0x4A, 0x01, 0x80, 0x00, 0x00, 0x00, 0x01,
         0x17, 0x01, 0x64, 0x39, 0x05, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x80, 0xA6, 0x44, 0x34, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ATTACKERSTATEUPDATE {
        SMSG_ATTACKERSTATEUPDATE {
            hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo::empty()
                .set_critical_hit()
                ,
            attacker: Guid::new(0x17),
            target: Guid::new(0x64),
            total_damage: 0x539,
            damages: vec![
                DamageInfo {
                    spell_school_mask: 0x0,
                    damage_float: 1332_f32,
                    damage_uint: 0x534,
                    absorb: 0x0,
                    resist: 0x0,
                },
            ],
            damage_state: 0x0,
            unknown1: 0x0,
            spell_id: 0x0,
            blocked_amount: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 148.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackerstateupdate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 148.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackerstateupdate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 148.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackerstateupdate0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 53] = [ 0x00, 0x33, 0x4A, 0x01, 0x02, 0x80, 0x00, 0x00, 0x01,
         0x17, 0x01, 0x64, 0x78, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0xF0, 0x42, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected1() -> SMSG_ATTACKERSTATEUPDATE {
        SMSG_ATTACKERSTATEUPDATE {
            hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo::empty()
                .set_affects_victim()
                .set_crushing()
                ,
            attacker: Guid::new(0x17),
            target: Guid::new(0x64),
            total_damage: 0x78,
            damages: vec![
                DamageInfo {
                    spell_school_mask: 0x0,
                    damage_float: 120_f32,
                    damage_uint: 0x78,
                    absorb: 0x0,
                    resist: 0x0,
                },
            ],
            damage_state: 0x1,
            unknown1: 0x0,
            spell_id: 0x0,
            blocked_amount: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 194.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackerstateupdate1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 194.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackerstateupdate1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 194.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackerstateupdate1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    const RAW2: [u8; 53] = [ 0x00, 0x33, 0x4A, 0x01, 0x12, 0x00, 0x00, 0x00, 0x01,
         0x17, 0x01, 0x64, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected2() -> SMSG_ATTACKERSTATEUPDATE {
        SMSG_ATTACKERSTATEUPDATE {
            hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo::empty()
                .set_affects_victim()
                .set_miss()
                ,
            attacker: Guid::new(0x17),
            target: Guid::new(0x64),
            total_damage: 0x0,
            damages: vec![
                DamageInfo {
                    spell_school_mask: 0x0,
                    damage_float: 0_f32,
                    damage_uint: 0x0,
                    absorb: 0x0,
                    resist: 0x0,
                },
            ],
            damage_state: 0x0,
            unknown1: 0x0,
            spell_id: 0x0,
            blocked_amount: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 239.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackerstateupdate2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW2)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 239.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackerstateupdate2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 239.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackerstateupdate2() {
        let expected = expected2();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW2)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW2.len());

        let mut dest = Vec::with_capacity(RAW2.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW2);
    }

    const RAW3: [u8; 113] = [ 0x00, 0x6F, 0x4A, 0x01, 0x03, 0x00, 0x00, 0x00, 0x01,
         0x17, 0x01, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x3F, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x40, 0x09, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected3() -> SMSG_ATTACKERSTATEUPDATE {
        SMSG_ATTACKERSTATEUPDATE {
            hit_info: SMSG_ATTACKERSTATEUPDATE_HitInfo::empty()
                .set_unk1(SMSG_ATTACKERSTATEUPDATE_HitInfo_Unk1 {
                    debug_float1: 1_f32,
                    debug_float10: 0_f32,
                    debug_float11: 0_f32,
                    debug_float12: 0_f32,
                    debug_float13: 0_f32,
                    debug_float14: 0_f32,
                    debug_float15: 0_f32,
                    debug_float16: 0_f32,
                    debug_float17: 0_f32,
                    debug_float18: 2_f32,
                    debug_float2: 0_f32,
                    debug_float3: 0_f32,
                    debug_float4: 0_f32,
                    debug_float5: 0_f32,
                    debug_float6: 0_f32,
                    debug_float7: 0_f32,
                    debug_float8: 0_f32,
                    debug_float9: 0_f32,
                    debug_int1: 0x7,
                    debug_int2: 0x9,
                })
                .set_affects_victim()
                ,
            attacker: Guid::new(0x17),
            target: Guid::new(0x64),
            total_damage: 0x0,
            damages: vec![ ],
            damage_state: 0x1,
            unknown1: 0x0,
            spell_id: 0x0,
            blocked_amount: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 284.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_attackerstateupdate3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW3)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 284.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_attackerstateupdate3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

    // Generated from `wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm` line 284.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_attackerstateupdate3() {
        let expected = expected3();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW3)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ATTACKERSTATEUPDATE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ATTACKERSTATEUPDATE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW3.len());

        let mut dest = Vec::with_capacity(RAW3.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW3);
    }

}

