//! Type definition of [`FixedString`].

/// Enumeration of errors about the [`FixedString`].
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FixedStringParseError {
    /// Failed to convert to `FixedString`
    #[error("Failed to convert '{0:?}' to `FixedString`")]
    ConvertFailure(String),
    /// Length of the array mismatch.
    #[error("Length of the array mismatch: {0}")]
    ArrayLenMismatch(String),
}

/// The fixed-capacity char string.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FixedString<const N: usize> {
    /// The inner char buffer.
    pub buf: [char; N],
}
impl<const N: usize> FixedString<N> {
    /// Create a new [`FixedString`] object.
    ///
    /// The `buf` field will automatically fill to `['\0'; N]`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// # fn main() {
    /// assert_eq!(FixedString::<3>::new().buf, ['\0', '\0', '\0']);
    /// # }
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self { buf: ['\0'; N] }
    }

    /// Get the length of the buffer.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// # fn main() {
    /// let fstr = FixedString::<10>::new();
    /// assert_eq!(fstr.len(), 10);
    /// # }
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        self.buf.len()
    }
    /// Returns if the `FixedString` is empty.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// assert!(FixedString::<0>::new().is_empty());
    /// assert!(!FixedString::<1>::new().is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    /// Write the buffer, auto pad trailing '\0', truncate if too long.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// # fn main() {
    /// let mut fstr = FixedString::<5>::new();
    /// fstr.write(&['h', 'e', 'l']);
    /// assert_eq!(fstr.buf, ['h', 'e', 'l', '\0', '\0']);
    ///
    /// fstr.write(&['h', 'e', 'l', 'l', 'o', '!']);
    /// assert_eq!(fstr.buf, ['h', 'e', 'l', 'l', 'o']);
    /// # }
    /// ```
    pub fn write(&mut self, data: &[char]) {
        self.buf = ['\0'; N];
        let take = data.len().min(N);
        self.buf[0..take].copy_from_slice(&data[0..take]);
    }

    /// Convert to UTF-8 char stream.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// # fn main() {
    /// let mut fstr = FixedString::<5>::new();
    /// fstr.write(&['h', 'e', 'l', 'l', 'o']);
    /// assert_eq!(fstr.to_utf8(), vec![104, 101, 108, 108, 111]);
    /// # }
    /// ```
    #[must_use]
    pub fn to_utf8(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let end = self.len();
        for c in &self.buf[0..end] {
            let mut tmp = [0u8; 4];
            let s = c.encode_utf8(&mut tmp);
            out.extend_from_slice(s.as_bytes());
        }
        out
    }
    /// Conervt to `[u32; N]`
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// # fn main() {
    /// let fstr = "abc".parse::<FixedString<3>>().unwrap();
    /// assert_eq!(fstr.to_u32_array(), [97, 98, 99]);
    /// # }
    /// ```
    #[must_use]
    pub fn to_u32_array(&self) -> Vec<u32> {
        self.buf.iter().map(|&c| c as u32).collect()
    }
    /// Returns the memory representation of this char array
    /// as a byte array in little-endian byte order.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// let value = "A".parse::<FixedString<1>>().unwrap();
    /// assert_eq!(value.to_u32_le_bytes(), vec![65, 0, 0, 0]);
    /// ```
    pub fn to_u32_le_bytes(&self) -> Vec<u8> {
        self.to_u32_array()
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect()
    }
    /// Creates a native endian fixed string from its
    /// representation as a byte array in little endian.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// let value = FixedString::<1>::from_le_bytes(&[65, 0, 0, 0]).unwrap();
    /// assert_eq!(value.trim_end_null(), "A");
    /// ```
    pub fn from_le_bytes(buf: &[u8]) -> Result<Self, FixedStringParseError> {
        if buf.len() != N * 4 {
            return Err(FixedStringParseError::ArrayLenMismatch(
                "FixedString buffer length must be N*4".into(),
            ));
        }
        let mut chars = ['\0'; N];
        for (i, chunk) in buf.as_chunks::<4>().0.iter().enumerate() {
            let chunk = *chunk;
            let raw = u32::from_le_bytes(chunk);
            let c = char::from_u32(raw).unwrap_or('\0');
            chars[i] = c;
        }
        Ok(Self { buf: chars })
    }
    /// Returns if the `buf` field is all null (`'\0'`).
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// assert!(FixedString::<2>::new().is_all_null());
    /// assert!(!FixedString::<2>::from_str_const("A").is_all_null());
    /// ```
    #[must_use]
    pub fn is_all_null(&self) -> bool {
        self.buf.iter().all(|ch| *ch == '\0')
    }
    /// Remove trailing null ('\0').
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// let value = FixedString::<3>::from_str_const("A");
    /// assert_eq!(value.trim_end_null(), "A");
    /// ```
    #[must_use]
    pub fn trim_end_null(&self) -> String {
        let slice = self.buf.as_slice();
        let end_pos = slice
            .iter()
            .rposition(|&c| c != '\0')
            .map(|i| i + 1)
            .unwrap_or(0);
        slice[0..end_pos].iter().collect::<String>()
    }
    /// Const version: convert &str to `FixedString`, truncate / pad '\0'.
    /// Works in const context.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::FixedString;
    /// const TITLE: FixedString<3> = FixedString::from_str_const("Title");
    /// assert_eq!(TITLE.trim_end_null(), "Tit");
    /// ```
    #[must_use]
    pub const fn from_str_const(s: &str) -> Self {
        let mut buf = ['\0'; N];
        let bytes = s.as_bytes();
        let mut i = 0; // char index into buf
        let mut p = 0; // byte index into bytes

        while i < N && p < bytes.len() {
            let b = bytes[p];
            if b & 0x80 == 0 {
                // 1-byte ASCII
                buf[i] = char::from_u32(b as u32).unwrap();
                i += 1;
                p += 1;
            } else if b & 0xE0 == 0xC0 {
                // 2-byte
                if p + 1 >= bytes.len() {
                    break;
                }
                let c = ((b & 0x1F) as u32) << 6 | (bytes[p + 1] & 0x3F) as u32;
                if let Some(ch) = char::from_u32(c) {
                    buf[i] = ch;
                    i += 1;
                }
                p += 2;
            } else if b & 0xF0 == 0xE0 {
                // 3-byte
                if p + 2 >= bytes.len() {
                    break;
                }
                let c = ((b & 0x0F) as u32) << 12
                    | ((bytes[p + 1] & 0x3F) as u32) << 6
                    | (bytes[p + 2] & 0x3F) as u32;
                if let Some(ch) = char::from_u32(c) {
                    buf[i] = ch;
                    i += 1;
                }
                p += 3;
            } else if b & 0xF8 == 0xF0 {
                // 4-byte
                if p + 3 >= bytes.len() {
                    break;
                }
                let c = ((b & 0x07) as u32) << 18
                    | ((bytes[p + 1] & 0x3F) as u32) << 12
                    | ((bytes[p + 2] & 0x3F) as u32) << 6
                    | (bytes[p + 3] & 0x3F) as u32;
                if let Some(ch) = char::from_u32(c) {
                    buf[i] = ch;
                    i += 1;
                }
                p += 4;
            } else {
                // invalid utf8 byte, skip
                p += 1;
            }
        }
        Self { buf }
    }
}
impl<const N: usize> Default for FixedString<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::fallible_impl_from)] // It will also automatically implement `TryFrom`
impl<const N: usize> From<&str> for FixedString<N> {
    /// Convert a [`str`] reference to a [`FixedString`].
    ///
    /// # Panics
    /// If the length of the [`str`] reference mismatches the generic argument `N`.
    fn from(value: &str) -> Self {
        let v = value.chars().collect::<Vec<_>>();
        Self {
            buf: v.try_into().unwrap(),
        }
    }
}

impl<const N: usize> std::str::FromStr for FixedString<N> {
    /// Convert a [`str`] reference to a [`FixedString`].
    ///
    /// # Errors
    /// If the length of the [`str`] reference mismatches the generic argument `N`.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let v = value.chars().collect::<Vec<_>>();
        Ok(Self {
            buf: v
                .try_into()
                .map_err(|_| FixedStringParseError::ConvertFailure(value.into()))?,
        })
    }
    type Err = FixedStringParseError;
}

#[cfg(test)]
mod tests {
    use crate::midi::midi;

    use super::*;
    #[test]
    fn test_if_from_str_is_correct() {
        let s = String::from("hello");
        let fstr = s.parse::<FixedString<5>>().unwrap();
        assert_eq!(fstr.buf, ['h', 'e', 'l', 'l', 'o']);
    }
    #[test]
    fn test_if_from_str_fails_because_length_mismatched() {
        let s = String::from("hello");
        s.parse::<FixedString<7>>().unwrap_err();
    }
    #[test]
    fn test_if_fstr_construction_will_initialize_buffer_as_zeros() {
        assert_eq!(FixedString::<3>::new().buf, ['\0', '\0', '\0']);
    }
    #[test]
    fn test_if_buffer_write_success() {
        let mut fstr = FixedString::<3>::new();
        let data = ['a', 'b', 'c'];
        fstr.write(&data);
        assert_eq!(fstr.buf, data);
    }
    #[test]
    fn test_fstr_to_utf8_string() {
        let f = FixedString::<3> {
            buf: ['a', 'b', 'c'],
        };
        assert_eq!(f.trim_end_null(), "abc");
    }
    #[test]
    fn test_fstr_serialization_deserialization() {
        const NAME: &str = "John Doe";
        let midi = midi!(
            Name = "Foo",
            Copyright = "Copyright (c) 2026 Bar",
            Author = NAME,
            1.0,
        );
        assert_eq!(
            FixedString::<{ 8 * 4 }>::from_le_bytes(&midi.meta.author.unwrap().to_u32_le_bytes())
                .unwrap()
                .trim_end_null()
                .as_str(),
            NAME
        );
    }
}
