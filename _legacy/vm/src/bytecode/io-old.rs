trait DataIO: BytesIO {
    type Target: BytesIO;
    fn into_bytes(&self) -> Self::Target;
    fn from_bytes(t: Self::Target) -> Self;
}

trait BytesIO {
    fn read(b: &[u8]) -> Option<(&[u8], Self)>;
    fn write(t: &Self, b: &mut [u8]) -> Option<&mut [u8]>;
}

impl<T: DataIO> BytesIO for T {
    fn read(b: &[u8]) -> Option<(&[u8], Self)> {
        let (b, t) = <<T as DataIO>::Target as BytesIO>::read(b)?;
        Some((b, <T as DataIO>::from_bytes(t) ))
    }
    fn write(t: &Self, b: &mut [u8]) -> Option<&mut [u8]> {
        let t = <T as DataIO>::into_bytes(t);
        <<T as DataIO>::Target as BytesIO>::write(t)
    }
}

macro_rules! num_impl_bytes_io {
    ($n:ty) => {
        impl BytesIO for $n {
            use std::mem::size_of;
            fn read(b: &[u8]) -> Option<(&[u8], Self)> {
                use std::convert::TryInto;
                let s = size_of::<Self>();
                Some((b.get(s..)?, Self::from_be_bytes(b.get(0..s)?.try_into()?))
            }
            fn write(t: &Self, b: &mut [u8]) -> Option<&mut [u8]> {
                let s = size_of::<Self>();
                let t = b.get_mut(..s)?;
                t.copy_from_slice(&Self::to_be_bytes(*t));
                Some(b.get_mut(s..)?)
            }
        }
    };
    ($n:ty, $($nn:ty),+ $(,)?) => {
        num_impl_bytes_io!($n);
        num_impl_bytes_io!($( $nn ),+);
    };
}

num_impl_bytes_io!(i8, i16, i32, i64, u8, u16, u32, u64);

macro_rules! tuple_impl_bytes_io {
    (s1 $($t:ident),+) => {
        impl<$($t: BytesIO),+> BytesIO for ($($t),+ ,) {
            fn read(b: &[u8]) -> Option<(&[u8], Self)> {
                $(
                    let (b, $t) = $t::read(b)?;
                )+
                Some((b, ($($t),+ ,)))
            }
            fn write(t: &Self, b: &mut [u8]) -> Option<&mut [u8]> {
                let ($($t),+ ,) = t;
                $(
                    let b = $t::write($t, b)?;
                )+
                Some(b)
            }
        }
    };
    ($tip:ident, $($rest:ident),+) -> {
        tuple_impl_bytes_io!(s1 $tip, $($rest),+);
        tuple_impl_bytes_io!($($rest),+);
    };
}

tuple_impl_bytes_io!(T7, T6, T5, T4, T3, T2, T1, T0);
