/// The EmfPlusARGB object specifies a color as a combination of red,
/// green, blue, and alpha (MS-EMFPLUS 2.2.2.1).
///
/// The wire order is Blue, Green, Red, Alpha; note that this differs
/// from the WMF `ColorRef` order (Red, Green, Blue, Reserved).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EmfPlusARGB {
    /// Blue (1 byte): An unsigned integer that specifies the relative
    /// intensity of blue.
    pub blue: u8,
    /// Green (1 byte): An unsigned integer that specifies the relative
    /// intensity of green.
    pub green: u8,
    /// Red (1 byte): An unsigned integer that specifies the relative
    /// intensity of red.
    pub red: u8,
    /// Alpha (1 byte): An unsigned integer that specifies the
    /// transparency of the background, ranging from 0 for completely
    /// transparent to 0xFF for completely opaque.
    pub alpha: u8,
}

impl EmfPlusARGB {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::read_field;

        let mut consumed_bytes: usize = 0;
        let blue = read_field(buf, &mut consumed_bytes)?;
        let green = read_field(buf, &mut consumed_bytes)?;
        let red = read_field(buf, &mut consumed_bytes)?;
        let alpha = read_field(buf, &mut consumed_bytes)?;

        Ok((Self { blue, green, red, alpha }, consumed_bytes))
    }

    /// Reinterprets a 32-bit value in 0xAARRGGBB layout (the layout
    /// produced by reading the 4 wire bytes as a little-endian u32).
    /// Used where a record field is either an object table index or an
    /// ARGB color depending on a flag bit.
    pub fn from_u32(value: u32) -> Self {
        Self {
            blue: (value & 0xFF) as u8,
            green: ((value >> 8) & 0xFF) as u8,
            red: ((value >> 16) & 0xFF) as u8,
            alpha: ((value >> 24) & 0xFF) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_wire_order_blue_green_red_alpha() {
        let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04];
        let (v, c) = EmfPlusARGB::parse(&mut buf).unwrap();

        assert_eq!(c, 4);
        assert_eq!(v, EmfPlusARGB {
            blue: 0x01,
            green: 0x02,
            red: 0x03,
            alpha: 0x04
        },);
    }

    #[test]
    fn from_u32_matches_wire_parse() {
        let raw = u32::from_le_bytes([0x01, 0x02, 0x03, 0x04]);
        let mut buf: &[u8] = &[0x01, 0x02, 0x03, 0x04];
        let (parsed, _) = EmfPlusARGB::parse(&mut buf).unwrap();

        assert_eq!(EmfPlusARGB::from_u32(raw), parsed);
    }
}
