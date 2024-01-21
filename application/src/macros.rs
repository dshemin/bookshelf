#[macro_export]
macro_rules! domain_type {
    ( $tname:ident, $errname:ident, $type:ty, $($rules:expr),+ ) => {
        #[derive(
            std::fmt::Debug, garde::Validate, serde::Serialize, std::cmp::PartialEq, std::cmp::Eq,
        )]
        #[garde(transparent)]
        pub struct $tname(#[garde($($rules),+)] $type);

        impl $tname {
            pub fn new<T>(value: T) -> $errname
            where
                T: std::convert::Into<$type>,
            {
                let v = Self(value.into());
                v.validate(&()).map_err(|x| {
                    let v: Vec<_> = x.iter().collect();
                    garde::Error::new(v[0].1.message())
                })?;

                Ok(v)
            }

            pub(crate) fn new_valid<T>(value: T) -> Self
            where
                T: std::convert::Into<$type>,
            {
                Self(value.into())
            }
        }

        pub type $errname = std::result::Result<$tname, garde::Error>;

        impl std::convert::Into<$type> for $tname {
            fn into(self) -> $type {
                self.0
            }
        }

        impl std::convert::TryFrom<$type> for $tname {
            type Error = garde::Error;

            fn try_from(value: $type) -> std::result::Result<Self, Self::Error> {
                $tname::new(value)
            }
        }
    };
}
