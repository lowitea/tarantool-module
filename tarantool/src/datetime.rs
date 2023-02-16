use serde::{Deserialize, Serialize};
use std::os::raw::c_char;
use std::{convert::TryInto, fmt::Display};
use time::{Duration, UtcOffset};

use crate::ffi::datetime as ffi;

type Inner = time::OffsetDateTime;

#[derive(Debug, Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Datetime {
    inner: Inner,
}

impl Datetime {
    #[inline(always)]
    pub fn from_inner(inner: Inner) -> Self {
        inner.into()
    }

    #[inline(always)]
    pub fn into_inner(self) -> Inner {
        self.into()
    }

    /// Convert an array of bytes in the  endian order into a `DateTime`.
    #[inline(always)]
    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        let sec_bytes: [u8; 8] = bytes[0..8].try_into().unwrap();
        let nsec_bytes: [u8; 4] = bytes[8..12].try_into().unwrap();
        let tzoffest_bytes: [u8; 2] = bytes[12..14].try_into().unwrap();

        let secs = i64::from_le_bytes(sec_bytes);
        let nsecs = u32::from_le_bytes(nsec_bytes);
        let tzoffset: i32 = i16::from_le_bytes(tzoffest_bytes).into();

        let dt = Inner::from_unix_timestamp(secs)
            .unwrap()
            .to_offset(UtcOffset::from_whole_seconds(tzoffset * 60).unwrap())
            + Duration::nanoseconds(nsecs as i64);

        dt.into()
    }

    /// Convert a slice of bytes in the little endian order into a `DateTime`. Return
    /// `None` if there's not enough bytes in the slice.
    #[inline(always)]
    pub fn try_from_slice(bytes: &[u8]) -> Option<Self> {
        std::convert::TryInto::try_into(bytes)
            .ok()
            .map(Self::from_bytes)
    }

    // /// Convert the tarantool native (little endian) datetime representation into a
    // /// `Datetime`.
    // #[inline(always)]
    // pub fn from_tt_datetime(mut tt: ffi::datetime) -> Self {
    //     unsafe {
    //         dbg!(&tt);
    //         tt.s = tt.s.swap_bytes();
    //         tt.n = tt.n.swap_bytes();
    //         tt.tz = tt.tz.swap_bytes();
    //         tt.tzi = tt.tzi.swap_bytes();
    //         dbg!(&tt);
    //         Self::from_bytes(std::mem::transmute(tt))
    //     }
    // }

    // /// Return an array of bytes in tarantool native (little endian) format
    // #[inline(always)]
    // pub fn to_tt_datetime(&self) -> ffi::datetime {
    //     unsafe {
    //         let mut tt: ffi::datetime = std::mem::transmute(self.as_bytes());
    //         dbg!(&tt);
    //         tt.s = tt.s.swap_bytes();
    //         tt.n = tt.n.swap_bytes();
    //         tt.tz = tt.tz.swap_bytes();
    //         tt.tzi = tt.tzi.swap_bytes();
    //         dbg!(&tt);
    //         tt
    //     }
    // }

    /// Return an array of bytes in the little endian order
    #[inline(always)]
    pub fn as_bytes(&self) -> [u8; 16] {
        let mut buf: Vec<u8> = vec![];

        buf.extend_from_slice(&self.inner.unix_timestamp().to_le_bytes());
        buf.extend_from_slice(&self.inner.nanosecond().to_le_bytes());
        buf.extend_from_slice(&self.inner.offset().whole_minutes().to_le_bytes());
        buf.resize(16, 0);

        buf.try_into().unwrap()
    }
}

impl From<Inner> for Datetime {
    #[inline(always)]
    fn from(inner: Inner) -> Self {
        Self { inner }
    }
}

impl From<Datetime> for Inner {
    #[inline(always)]
    fn from(dt: Datetime) -> Self {
        dt.inner
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

////////////////////////////////////////////////////////////////////////////////
/// Tuple
////////////////////////////////////////////////////////////////////////////////

impl serde::Serialize for Datetime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct _ExtStruct((c_char, serde_bytes::ByteBuf));

        let data = self.as_bytes();
        _ExtStruct((ffi::MP_DATETIME, serde_bytes::ByteBuf::from(&data as &[_])))
            .serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Datetime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct _ExtStruct((c_char, serde_bytes::ByteBuf));

        let _ExtStruct((kind, bytes)) = serde::Deserialize::deserialize(deserializer)?;

        if kind != ffi::MP_DATETIME {
            return Err(serde::de::Error::custom(format!(
                "Expected Datetime, found msgpack ext #{}",
                kind
            )));
        }

        let data = bytes.into_vec();
        Self::try_from_slice(&data).ok_or_else(|| {
            serde::de::Error::custom(format!(
                "Not enough bytes for Datetime: expected 16, got {}",
                data.len()
            ))
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Lua
////////////////////////////////////////////////////////////////////////////////

// static mut CTID_DATETIME: Option<u32> = None;

// fn ctid_datetime() -> u32 {
//     unsafe {
//         if CTID_DATETIME.is_none() {
//             let lua = crate::global_lua();
//             let ctid_datetime = tlua::ffi::luaL_ctypeid(
//                 tlua::AsLua::as_lua(&lua),
//                 crate::c_ptr!("struct tt_datetime"),
//             );
//             assert!(ctid_datetime != 0);
//             CTID_DATETIME = Some(ctid_datetime)
//         }
//         CTID_DATETIME.unwrap()
//     }
// }

// impl<L> tlua::LuaRead<L> for Datetime
// where
//     L: tlua::AsLua,
// {
//     fn lua_read_at_position(lua: L, index: std::num::NonZeroI32) -> Result<Self, L> {
//         let raw_lua = lua.as_lua();
//         let index = index.get();
//         unsafe {
//             if tlua::ffi::lua_type(raw_lua, index) != tlua::ffi::LUA_TCDATA {
//                 return Err(lua);
//             }
//             let mut ctypeid = std::mem::MaybeUninit::uninit();
//             let cdata = tlua::ffi::luaL_checkcdata(raw_lua, index, ctypeid.as_mut_ptr());
//             if ctypeid.assume_init() != ctid_datetime() {
//                 return Err(lua);
//             }
//             Ok(Self::from_tt_datetime(*cdata.cast()))
//         }
//     }
// }

// impl<L: tlua::AsLua> tlua::Push<L> for Datetime {
//     type Err = tlua::Void;

//     #[inline(always)]
//     fn push_to_lua(&self, lua: L) -> Result<tlua::PushGuard<L>, (Self::Err, L)> {
//         tlua::PushInto::push_into_lua(*self, lua)
//     }
// }

// impl<L: tlua::AsLua> tlua::PushOne<L> for Datetime {}

// impl<L: tlua::AsLua> tlua::PushInto<L> for Datetime {
//     type Err = tlua::Void;

//     fn push_into_lua(self, lua: L) -> Result<tlua::PushGuard<L>, (Self::Err, L)> {
//         unsafe {
//             let cdata = tlua::ffi::luaL_pushcdata(lua.as_lua(), ctid_datetime());
//             std::ptr::write(cdata as _, self.to_tt_datetime());
//             Ok(tlua::PushGuard::new(lua, 1))
//         }
//     }
// }

// impl<L: tlua::AsLua> tlua::PushOneInto<L> for Datetime {}
