/// A set of flags in 1.12: a crushing blow is `AFFECTS_VICTIM | CRUSHING` (0x8002), a main hand's miss `AFFECTS_VICTIM | MISS` (0x12). The client's `SMSG_ATTACKERSTATEUPDATE` parse (0x625C60) branches on `UNK1` alone.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L1):
/// ```text
/// flag HitInfo : u32 {
///     NORMAL_SWING = 0x00000000;
///     UNK1 = 0x00000001;
///     AFFECTS_VICTIM = 0x00000002;
///     LEFT_SWING = 0x00000004;
///     EARLY_CRITICAL_HIT = 0x00000008;
///     MISS = 0x00000010;
///     ABSORB = 0x00000020;
///     RESIST = 0x00000040;
///     CRITICAL_HIT = 0x00000080;
///     UNK9 = 0x00000100;
///     BLOCK = 0x00000800;
///     UNK10 = 0x00002000;
///     GLANCING = 0x00004000;
///     CRUSHING = 0x00008000;
///     NO_ACTION = 0x00010000;
///     SWING_NO_HIT_SOUND = 0x00080000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct HitInfo {
    inner: u32,
}

#[cfg(feature = "print-testcase")]
impl HitInfo {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NORMAL_SWING").unwrap();
            first = false;
        }
        if self.is_unk1() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK1").unwrap();
            first = false;
        }
        if self.is_affects_victim() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "AFFECTS_VICTIM").unwrap();
            first = false;
        }
        if self.is_left_swing() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LEFT_SWING").unwrap();
            first = false;
        }
        if self.is_early_critical_hit() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "EARLY_CRITICAL_HIT").unwrap();
            first = false;
        }
        if self.is_miss() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "MISS").unwrap();
            first = false;
        }
        if self.is_absorb() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ABSORB").unwrap();
            first = false;
        }
        if self.is_resist() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "RESIST").unwrap();
            first = false;
        }
        if self.is_critical_hit() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CRITICAL_HIT").unwrap();
            first = false;
        }
        if self.is_unk9() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK9").unwrap();
            first = false;
        }
        if self.is_block() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "BLOCK").unwrap();
            first = false;
        }
        if self.is_unk10() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "UNK10").unwrap();
            first = false;
        }
        if self.is_glancing() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GLANCING").unwrap();
            first = false;
        }
        if self.is_crushing() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "CRUSHING").unwrap();
            first = false;
        }
        if self.is_no_action() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NO_ACTION").unwrap();
            first = false;
        }
        if self.is_swing_no_hit_sound() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SWING_NO_HIT_SOUND").unwrap();
            first = false;
        }
        s
    }

}

impl HitInfo {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NORMAL_SWING: u32 = 0x00;
    pub const UNK1: u32 = 0x01;
    pub const AFFECTS_VICTIM: u32 = 0x02;
    pub const LEFT_SWING: u32 = 0x04;
    pub const EARLY_CRITICAL_HIT: u32 = 0x08;
    pub const MISS: u32 = 0x10;
    pub const ABSORB: u32 = 0x20;
    pub const RESIST: u32 = 0x40;
    pub const CRITICAL_HIT: u32 = 0x80;
    pub const UNK9: u32 = 0x100;
    pub const BLOCK: u32 = 0x800;
    pub const UNK10: u32 = 0x2000;
    pub const GLANCING: u32 = 0x4000;
    pub const CRUSHING: u32 = 0x8000;
    pub const NO_ACTION: u32 = 0x10000;
    pub const SWING_NO_HIT_SOUND: u32 = 0x80000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NORMAL_SWING
                | Self::UNK1
                | Self::AFFECTS_VICTIM
                | Self::LEFT_SWING
                | Self::EARLY_CRITICAL_HIT
                | Self::MISS
                | Self::ABSORB
                | Self::RESIST
                | Self::CRITICAL_HIT
                | Self::UNK9
                | Self::BLOCK
                | Self::UNK10
                | Self::GLANCING
                | Self::CRUSHING
                | Self::NO_ACTION
                | Self::SWING_NO_HIT_SOUND
        }
    }

    pub const fn is_unk1(&self) -> bool {
        (self.inner & Self::UNK1) != 0
    }

    /// The 1.12 client reads a debug trailer after the blocked amount when this is set (mangos-classic's `HITINFO_UNK0`).
    pub const fn new_unk1() -> Self {
        Self { inner: Self::UNK1 }
    }

    pub fn set_unk1(&mut self) -> Self {
        self.inner |= Self::UNK1;
        *self
    }

    pub fn clear_unk1(&mut self) -> Self {
        self.inner &= !Self::UNK1;
        *self
    }

    pub const fn is_affects_victim(&self) -> bool {
        (self.inner & Self::AFFECTS_VICTIM) != 0
    }

    /// no being hit animation on victim without it
    pub const fn new_affects_victim() -> Self {
        Self { inner: Self::AFFECTS_VICTIM }
    }

    pub fn set_affects_victim(&mut self) -> Self {
        self.inner |= Self::AFFECTS_VICTIM;
        *self
    }

    pub fn clear_affects_victim(&mut self) -> Self {
        self.inner &= !Self::AFFECTS_VICTIM;
        *self
    }

    pub const fn is_left_swing(&self) -> bool {
        (self.inner & Self::LEFT_SWING) != 0
    }

    pub const fn new_left_swing() -> Self {
        Self { inner: Self::LEFT_SWING }
    }

    pub fn set_left_swing(&mut self) -> Self {
        self.inner |= Self::LEFT_SWING;
        *self
    }

    pub fn clear_left_swing(&mut self) -> Self {
        self.inner &= !Self::LEFT_SWING;
        *self
    }

    pub const fn is_early_critical_hit(&self) -> bool {
        (self.inner & Self::EARLY_CRITICAL_HIT) != 0
    }

    /// According to vmangos used as crit prior to 1.9.
    pub const fn new_early_critical_hit() -> Self {
        Self { inner: Self::EARLY_CRITICAL_HIT }
    }

    pub fn set_early_critical_hit(&mut self) -> Self {
        self.inner |= Self::EARLY_CRITICAL_HIT;
        *self
    }

    pub fn clear_early_critical_hit(&mut self) -> Self {
        self.inner &= !Self::EARLY_CRITICAL_HIT;
        *self
    }

    pub const fn is_miss(&self) -> bool {
        (self.inner & Self::MISS) != 0
    }

    pub const fn new_miss() -> Self {
        Self { inner: Self::MISS }
    }

    pub fn set_miss(&mut self) -> Self {
        self.inner |= Self::MISS;
        *self
    }

    pub fn clear_miss(&mut self) -> Self {
        self.inner &= !Self::MISS;
        *self
    }

    pub const fn is_absorb(&self) -> bool {
        (self.inner & Self::ABSORB) != 0
    }

    /// plays absorb sound
    pub const fn new_absorb() -> Self {
        Self { inner: Self::ABSORB }
    }

    pub fn set_absorb(&mut self) -> Self {
        self.inner |= Self::ABSORB;
        *self
    }

    pub fn clear_absorb(&mut self) -> Self {
        self.inner &= !Self::ABSORB;
        *self
    }

    pub const fn is_resist(&self) -> bool {
        (self.inner & Self::RESIST) != 0
    }

    /// resisted atleast some damage
    pub const fn new_resist() -> Self {
        Self { inner: Self::RESIST }
    }

    pub fn set_resist(&mut self) -> Self {
        self.inner |= Self::RESIST;
        *self
    }

    pub fn clear_resist(&mut self) -> Self {
        self.inner &= !Self::RESIST;
        *self
    }

    pub const fn is_critical_hit(&self) -> bool {
        (self.inner & Self::CRITICAL_HIT) != 0
    }

    pub const fn new_critical_hit() -> Self {
        Self { inner: Self::CRITICAL_HIT }
    }

    pub fn set_critical_hit(&mut self) -> Self {
        self.inner |= Self::CRITICAL_HIT;
        *self
    }

    pub fn clear_critical_hit(&mut self) -> Self {
        self.inner &= !Self::CRITICAL_HIT;
        *self
    }

    pub const fn is_unk9(&self) -> bool {
        (self.inner & Self::UNK9) != 0
    }

    /// wotlk?
    pub const fn new_unk9() -> Self {
        Self { inner: Self::UNK9 }
    }

    pub fn set_unk9(&mut self) -> Self {
        self.inner |= Self::UNK9;
        *self
    }

    pub fn clear_unk9(&mut self) -> Self {
        self.inner &= !Self::UNK9;
        *self
    }

    pub const fn is_block(&self) -> bool {
        (self.inner & Self::BLOCK) != 0
    }

    /// mangos-classic's `HITINFO_BLOCK`.
    pub const fn new_block() -> Self {
        Self { inner: Self::BLOCK }
    }

    pub fn set_block(&mut self) -> Self {
        self.inner |= Self::BLOCK;
        *self
    }

    pub fn clear_block(&mut self) -> Self {
        self.inner &= !Self::BLOCK;
        *self
    }

    pub const fn is_unk10(&self) -> bool {
        (self.inner & Self::UNK10) != 0
    }

    /// wotlk?
    pub const fn new_unk10() -> Self {
        Self { inner: Self::UNK10 }
    }

    pub fn set_unk10(&mut self) -> Self {
        self.inner |= Self::UNK10;
        *self
    }

    pub fn clear_unk10(&mut self) -> Self {
        self.inner &= !Self::UNK10;
        *self
    }

    pub const fn is_glancing(&self) -> bool {
        (self.inner & Self::GLANCING) != 0
    }

    pub const fn new_glancing() -> Self {
        Self { inner: Self::GLANCING }
    }

    pub fn set_glancing(&mut self) -> Self {
        self.inner |= Self::GLANCING;
        *self
    }

    pub fn clear_glancing(&mut self) -> Self {
        self.inner &= !Self::GLANCING;
        *self
    }

    pub const fn is_crushing(&self) -> bool {
        (self.inner & Self::CRUSHING) != 0
    }

    pub const fn new_crushing() -> Self {
        Self { inner: Self::CRUSHING }
    }

    pub fn set_crushing(&mut self) -> Self {
        self.inner |= Self::CRUSHING;
        *self
    }

    pub fn clear_crushing(&mut self) -> Self {
        self.inner &= !Self::CRUSHING;
        *self
    }

    pub const fn is_no_action(&self) -> bool {
        (self.inner & Self::NO_ACTION) != 0
    }

    pub const fn new_no_action() -> Self {
        Self { inner: Self::NO_ACTION }
    }

    pub fn set_no_action(&mut self) -> Self {
        self.inner |= Self::NO_ACTION;
        *self
    }

    pub fn clear_no_action(&mut self) -> Self {
        self.inner &= !Self::NO_ACTION;
        *self
    }

    pub const fn is_swing_no_hit_sound(&self) -> bool {
        (self.inner & Self::SWING_NO_HIT_SOUND) != 0
    }

    pub const fn new_swing_no_hit_sound() -> Self {
        Self { inner: Self::SWING_NO_HIT_SOUND }
    }

    pub fn set_swing_no_hit_sound(&mut self) -> Self {
        self.inner |= Self::SWING_NO_HIT_SOUND;
        *self
    }

    pub fn clear_swing_no_hit_sound(&mut self) -> Self {
        self.inner &= !Self::SWING_NO_HIT_SOUND;
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for HitInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for HitInfo {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for HitInfo {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for HitInfo {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for HitInfo {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for HitInfo {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for HitInfo {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u32> for HitInfo {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<u8> for HitInfo {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for HitInfo {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u64> for HitInfo {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for HitInfo {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i16> for HitInfo {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i32> for HitInfo {
    fn from(value: i32) -> Self {
        Self::new(u32::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i64> for HitInfo {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u32>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for HitInfo {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u32>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

