//! `bitflags` crate Serde shims
//!
//! To enable to `bitflags` shims, add it to the crate features list:
//!
//! ```toml
//! serde_shims = { version = "*", features = ["bitflags"] }
//! ```
//!
//! Full example:
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//! #[macro_use] // required for impl_serde_for_bitflags
//! extern crate serde_shims;
//!
//! #[macro_use]
//! extern crate bitflags;
//!
//! bitflags! {
//!     // Note that `impl_serde_for_bitflags` requires the flag type to
//!     // implement `Serialize` and `Deserialize`.
//!     //
//!     // All primitive integer types satisfy these requirements
//!     pub struct Permission: u32 {
//!         const SEND_MESSAGE = 0x00000001;
//!         const EDIT_MESSAGE = 0x00000002;
//!         const KICK_MEMBER  = 0x00000004;
//!         const BAN_MEMBER   = 0x00000008;
//!     }
//! }
//!
//! impl_serde_for_bitflags!(Permission);
//!
//! fn main() {
//!     let test = Permission::SEND_MESSAGE | Permission::EDIT_MESSAGE;
//!
//!     assert_eq!(serde_json::to_string(&test).unwrap(), "3");
//!
//!     assert_eq!(serde_json::from_str::<Permission>("3").unwrap(), test);
//!
//!     assert!(serde_json::from_str::<Permission>("51").is_err());
//! }
//! ```

/// Implements `Serialize` and `Deserialize` for a `bitflags!` generated structure.
///
/// Note that `impl_serde_for_bitflags` requires the flag type to
/// implement `Serialize` and `Deserialize`.
///
/// All primitive integer types satisfy these requirements.
///
/// See the [`bitflags`](./bitflags/index.html) module for a full example.
#[macro_export]
macro_rules! impl_serde_for_bitflags {
    ($name:ident) => {
        impl $crate::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                self.bits().serialize(serializer)
            }
        }

        impl<'de> $crate::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<$name, D::Error>
            where
                D: $crate::serde::Deserializer<'de>,
            {
                let value = <_ as $crate::serde::Deserialize<'de>>::deserialize(deserializer)?;

                $name::from_bits(value)
                    .ok_or_else(|| $crate::serde::de::Error::custom(format!("Invalid bits {:#X} for {}", value, stringify!($name))))
            }
        }
    };
}
