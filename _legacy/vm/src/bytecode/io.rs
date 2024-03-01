use std::convert::TryInto;
use std::mem::size_of;

pub enum BytesReadError<'a> {
    EndOfFile,
    InvalidValue(&'a [u8]),
}

pub trait BytesIO: Sized {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>>;
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]>;
}

pub trait DataIO: BytesIO {
    type Target: BytesIO;
    fn from_bytes(t: Self::Target) -> Option<Self>;
    fn into_bytes(&self) -> Self::Target;
}

// pub trait DataInput {
//     type Input: BytesIO;
//     fn from_bytes(t: Self::Input) -> Option<Self>;
// }

// impl<T: DataIO + Sized> DataInput for T {
//     type Input = <T as DataIO>::Target;
//     fn from_bytes(t: Self::Input) -> Option<Self> {
//         <T as DataIO>::from_bytes(t)
//     }
// }

// pub trait DataOutput {
//     type Output: BytesIO;
//     fn into_bytes(&self) -> &Self::Output;
// }

// impl<T: DataIO + Sized> DataOutput for T {
//     type Output = <T as DataIO>::Target;
//     fn into_bytes(&self) -> &Self::Output {
//         &<T as DataIO>::from_bytes(self)
//     }
// }

// impl<T: DataInput + DataOutput + Sized> BytesIO for T {
//     fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
//         let (b2, t) = <<T as DataInput>::Input as BytesIO>::read(b)?;
//         let t = match <T as DataInput>::from_bytes(t) {
//             Some(t) => t,
//             None => {
//                 return Err(BytesReadError::InvalidValue(b));
//             }
//         };
//         Ok((b2, t))
//     }
//     fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
//         let t = <T as DataOutput>::into_bytes(t);
//         <<T as DataOutput>::Output as BytesIO>::write(&t, b)
//     }
// }

impl<T: DataIO + Sized> BytesIO for T {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b2, t) = <<T as DataIO>::Target as BytesIO>::read(b)?;
        let t = match <T as DataIO>::from_bytes(t) {
            Some(t) => t,
            None => {
                return Err(BytesReadError::InvalidValue(b));
            }
        };
        Ok((b2, t))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        let t = <T as DataIO>::into_bytes(t);
        <<T as DataIO>::Target as BytesIO>::write(&t, b)
    }
}

impl<T: BytesIO> BytesIO for Vec<T> {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (mut b, len) = <u32 as BytesIO>::read(b)?;
        let mut acc = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let (b2, t) = <T as BytesIO>::read(b)?;
            acc.push(t);
            b = b2;
        }
        Ok((b, acc))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        let len = t.len();
        if len > u32::MAX as usize {
            panic!("error: vecs cannot have more than 4294967295 (2^32 - 1) elements");
        }
        let mut b = <u32 as BytesIO>::write(&(len as u32), b)?;
        for i in 0..len {
            b = <T as BytesIO>::write(unsafe { t.get_unchecked(i) }, b)?;
        }
        Some(b)
    }
}

macro_rules! num_impl_bytes_io {
    ($n:ty) => {
        impl BytesIO for $n {
            fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
                let s = size_of::<Self>();
                let b2 = b.get(s..).ok_or(BytesReadError::EndOfFile)?;
                let r = Self::from_be_bytes(b.get(0..s).ok_or(BytesReadError::EndOfFile)?.try_into().unwrap());
                Ok((b2, r))
            }
            fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
                let s = size_of::<Self>();
                let u = b.get_mut(..s)?;
                u.copy_from_slice(&Self::to_be_bytes(*t));
                Some(b.get_mut(s..)?)
            }
        }
    };
    ($n:ty, $($nn:ty),+ $(,)?) => {
        num_impl_bytes_io!($n);
        num_impl_bytes_io!($( $nn ),+);
    };
}

num_impl_bytes_io!(i8, i16, i32, i64, u8, u16, u32, u64, f64);

macro_rules! tuple_impl_bytes_io {
    (s1 $($t:ident),+) => {
        impl<$($t: BytesIO),+> BytesIO for ($($t),+ ,) {
            #![allow(non_snake_case)]
            fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
                $(
                    let (b, $t) = $t::read(b)?;
                )+
                Ok((b, ($($t),+ ,)))
            }
            fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
                let ($($t),+ ,) = t;
                $(
                    let b = $t::write($t, b)?;
                )+
                Some(b)
            }
        }
    };
    ($tip:ident, $($rest:ident),+) => {
        tuple_impl_bytes_io!(s1 $tip, $($rest),+);
        tuple_impl_bytes_io!($( $rest ),+);
    };
    ($tip:ident) => {};
}

tuple_impl_bytes_io!(T7, T6, T5, T4, T3, T2, T1, T0);
