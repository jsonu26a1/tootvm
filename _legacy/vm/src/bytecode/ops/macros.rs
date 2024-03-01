macro_rules! new_op_empty {
    ($name:ident) => {
        pub struct $name;

        impl super::BytesIO for $name {
            fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), super::BytesReadError<'a>> {
                Ok((b, $name))
            }
            fn write<'a>(_: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
                Some(b)
            }
        }
    };
}

macro_rules! new_op {
    ($(#[$meta:meta])* $v:vis struct $name:ident { $($fv:vis $field:ident : $type:ty),+ $(,)? }) => {
        $(#[$meta])*
        $v struct $name {
            $($fv $field : $type),+
        }

        impl $name {
            pub fn new($($field : $type),+) -> $name {
                $name { $($field),+ }
            }
        }

        #[allow(unused_parens)]
        impl super::DataIO for $name {
            type Target = ($($type),+);
            fn from_bytes(t: Self::Target) -> Option<Self> {
                let ($($field),+) = t;
                Some($name { $($field),+ })
            }
            fn into_bytes(&self) -> Self::Target {
                ($(self.$field),+)
            }
        }
    };
}
