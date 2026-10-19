/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogmiss.wowm#L1):
/// ```text
/// enum SpellLogMissLayout : u8 {
///     NORMAL = 0;
///     DEBUG = 1;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellLogMissLayout {
    Normal,
    /// Each target has two floats after its reason. The 1.12 client reads them for any value but 0.
    Debug,
}

impl SpellLogMissLayout {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Normal => 0x0,
            Self::Debug => 0x1,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
            Self::Normal,
            Self::Debug,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl SpellLogMissLayout {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Debug => "DEBUG",
        }
    }

}

const NAME: &str = "SpellLogMissLayout";

impl Default for SpellLogMissLayout {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for SpellLogMissLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Debug => f.write_str("Debug"),
        }
    }
}

impl TryFrom<u8> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Normal),
            1 => Ok(Self::Debug),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SpellLogMissLayout {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

