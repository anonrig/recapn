//! A fixed size blob of bytes contained in a Cap'n Proto message

use core::fmt;
use core::ops::{Deref, DerefMut};

use crate::internal::Sealed;
use crate::list::ElementSize;
use crate::{ty, Family, IntoFamily};

pub mod ptr {
    pub use crate::ptr::{BlobBuilder as Builder, BlobReader as Reader};
}

/// An error returned when trying to set a data field to a slice value if the value is too large.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromSliceError(pub(crate) ());

impl fmt::Display for TryFromSliceError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("attempted to create a data blob from too large a slice")
    }
}

impl core::error::Error for TryFromSliceError {}

#[derive(Clone, Copy)]
pub struct Data<T = Family>(T);

impl Sealed for Data {}
impl<T> IntoFamily for Data<T> {
    type Family = Data;
}
impl ty::ListValue for Data {
    const ELEMENT_SIZE: ElementSize = ElementSize::Pointer;
}

pub type Reader<'a> = Data<ptr::Reader<'a>>;

impl<'a> From<ptr::Reader<'a>> for Reader<'a> {
    #[inline]
    fn from(repr: ptr::Reader<'a>) -> Self {
        Self(repr)
    }
}

impl<'a> From<Reader<'a>> for ptr::Reader<'a> {
    #[inline]
    fn from(value: Reader<'a>) -> Self {
        value.0
    }
}

impl<'a> Reader<'a> {
    /// Creates an empty data reader.
    #[inline]
    pub const fn empty() -> Self {
        Self(ptr::Reader::empty())
    }

    /// Creates a data reader from a slice of bytes.
    ///
    /// # Panics
    ///
    /// If the slice is too large to be in a message, this function panics.
    #[inline]
    pub const fn from_slice(slice: &'a [u8]) -> Self {
        let Some(r) = ptr::Reader::new(slice) else {
            panic!("slice is too large to be contained within a cap'n proto message")
        };
        Self(r)
    }

    #[inline]
    pub const fn try_from_slice(slice: &'a [u8]) -> Result<Self, TryFromSliceError> {
        match ptr::Reader::new(slice) {
            Some(b) => Ok(Self(b)),
            None => Err(TryFromSliceError(())),
        }
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.0.len().get()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn as_slice(&self) -> &'a [u8] {
        self.0.as_slice()
    }
}

impl Deref for Reader<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl AsRef<[u8]> for Reader<'_> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> AsRef<ptr::Reader<'a>> for Reader<'a> {
    #[inline]
    fn as_ref(&self) -> &ptr::Reader<'a> {
        &self.0
    }
}

impl Default for Reader<'_> {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl PartialEq<Reader<'_>> for Reader<'_> {
    #[inline]
    fn eq(&self, other: &Reader<'_>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl PartialEq<[u8]> for Reader<'_> {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

impl PartialEq<Reader<'_>> for [u8] {
    #[inline]
    fn eq(&self, other: &Reader<'_>) -> bool {
        self == other.as_slice()
    }
}

pub type Builder<'a> = Data<ptr::Builder<'a>>;

impl<'a> From<ptr::Builder<'a>> for Builder<'a> {
    #[inline]
    fn from(repr: ptr::Builder<'a>) -> Self {
        Self(repr)
    }
}

impl<'a> From<Builder<'a>> for ptr::Builder<'a> {
    #[inline]
    fn from(value: Builder<'a>) -> Self {
        value.0
    }
}

impl<'a> Builder<'a> {
    #[inline]
    pub fn empty() -> Self {
        Data(ptr::Builder::empty())
    }

    #[inline]
    pub fn as_reader(&self) -> Reader<'_> {
        Data(unsafe { ptr::Reader::new_unchecked(self.0.data(), self.0.len()) })
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.0.len().get()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub const fn as_slice(&self) -> &[u8] {
        let data = self.0.data().as_ptr().cast_const();
        let len = self.len() as usize;
        unsafe { core::slice::from_raw_parts(data, len) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        let data = self.0.data().as_ptr();
        let len = self.len() as usize;
        unsafe { core::slice::from_raw_parts_mut(data, len) }
    }
}

impl<'a> AsRef<ptr::Builder<'a>> for Builder<'a> {
    fn as_ref(&self) -> &ptr::Builder<'a> {
        &self.0
    }
}

impl<'a> AsRef<[u8]> for Builder<'a> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl PartialEq<Builder<'_>> for Builder<'_> {
    fn eq(&self, other: &Builder<'_>) -> bool {
        **self == **other
    }
}

impl PartialEq<Reader<'_>> for Builder<'_> {
    fn eq(&self, other: &Reader<'_>) -> bool {
        **self == **other
    }
}

impl PartialEq<Builder<'_>> for Reader<'_> {
    fn eq(&self, other: &Builder<'_>) -> bool {
        **self == **other
    }
}

impl PartialEq<[u8]> for Builder<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        **self == *other
    }
}

impl PartialEq<Builder<'_>> for [u8] {
    fn eq(&self, other: &Builder<'_>) -> bool {
        *self == **other
    }
}

impl Deref for Builder<'_> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl DerefMut for Builder<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}
