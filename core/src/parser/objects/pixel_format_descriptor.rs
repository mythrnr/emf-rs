/// The PixelFormatDescriptor object specifies the pixel format of a drawing
/// surface.
///
/// The PixelFormatDescriptor object is used in EMR_HEADER records to specify
/// the pixel format of the output surface.
#[derive(Clone, Debug)]
pub struct PixelFormatDescriptor {
    /// nSize (2 bytes): An unsigned integer that specifies the size in bytes,
    /// of this data structure.
    pub n_size: u16,
    /// nVersion (2 bytes): An unsigned integer that MUST be set to 0x0001.
    pub n_version: u16,
    /// dwFlags (4 bytes): A set of bit flags that specify properties of the
    /// pixel buffer that is used for output to the drawing surface. These
    /// properties are not all mutually exclusive; combinations of flags are
    /// allowed, except where noted otherwise.
    pub dw_flags: DwFlags,
    /// iPixelType (1 byte): The type of pixel data.
    ///
    /// | Value | Meaning |
    /// |:-|:-|
    /// | `PFD_TYPE_RGBA` ( `0x00` ) | The pixel format is RGBA. |
    /// | `PFD_TYPE_COLORINDEX` ( `0x01` ) | Each pixel is an index in a color table. |
    pub i_pixel_type: u8,
    /// cColorBits (1 byte): The number of bits per pixel for RGBA pixel types,
    /// excluding the alpha bitplanes. For color table pixels, it is the size
    /// of each color table index.
    pub c_color_bits: u8,
    /// cRedBits (1 byte): Specifies the number of red bitplanes in each RGBA
    /// color buffer.
    pub c_red_bits: u8,
    /// cRedShift (1 byte): Specifies the shift count in bits for red bitplanes
    /// in each RGBA color buffer.
    pub c_red_shift: u8,
    /// cGreenBits (1 byte): Specifies the number of green bitplanes in each
    /// RGBA color buffer.
    pub c_green_bits: u8,
    /// cGreenShift (1 byte): Specifies the shift count for green bitplanes in
    /// each RGBA color buffer.
    pub c_green_shift: u8,
    /// cBlueBits (1 byte): Specifies the number of blue bitplanes in each RGBA
    /// color buffer.
    pub c_blue_bits: u8,
    /// cBlueShift (1 byte): Specifies the shift count for blue bitplanes in
    /// each RGBA color buffer.
    pub c_blue_shift: u8,
    /// cAlphaBits (1 byte): Specifies the number of alpha bitplanes in each
    /// RGBA color buffer.
    ///
    /// Windows does not support alpha bitplanes.
    pub c_alpha_bits: u8,
    /// cAlphaShift (1 byte): Specifies the shift count for alpha bitplanes in
    /// each RGBA color buffer.
    ///
    /// Windows does not support alpha bitplanes.
    pub c_alpha_shift: u8,
    /// cAccumBits (1 byte): Specifies the total number of bitplanes in the
    /// accumulation buffer.
    pub c_accum_bits: u8,
    /// cAccumRedBits (1 byte): Specifies the number of red bitplanes in the
    /// accumulation buffer.
    pub c_accum_red_bits: u8,
    /// cAccumGreenBits (1 byte): Specifies the number of green bitplanes in
    /// the accumulation buffer.
    pub c_accum_green_bits: u8,
    /// cAccumBlueBits (1 byte): Specifies the number of blue bitplanes in the
    /// accumulation buffer.
    pub c_accum_blue_bits: u8,
    /// cAccumAlphaBits (1 byte): Specifies the number of alpha bitplanes in
    /// the accumulation buffer.
    ///
    /// Windows does not support alpha bitplanes.
    pub c_accum_alpha_bits: u8,
    /// cDepthBits (1 byte): Specifies the depth of the depth (z-axis) buffer.
    pub c_depth_bits: u8,
    /// cStencilBits (1 byte): Specifies the depth of the stencil buffer.
    pub c_stencil_bits: u8,
    /// cAuxBuffers (1 byte): Specifies the number of auxiliary buffers.
    /// Auxiliary buffers are not supported.
    pub c_aux_buffers: u8,
    /// iLayerType (1 byte): This field MAY be ignored.
    pub i_layer_type: u8,
    /// bReserved (1 byte): Specifies the number of overlay and underlay
    /// planes. Bits 0 through 3 specify up to 15 overlay planes and bits 4
    /// through 7 specify up to 15 underlay planes.
    pub b_reserved: u8,
    /// dwLayerMask (4 bytes): This field MAY be ignored.
    pub dw_layer_mask: [u8; 4],
    /// dwVisibleMask (4 bytes): Specifies the transparent color or index of an
    /// underlay plane. When the pixel type is RGBA, dwVisibleMask is a
    /// transparent RGB color value. When the pixel type is color index, it is
    /// a transparent index value.
    pub dw_visible_mask: [u8; 4],
    /// dwDamageMask (4 bytes): This field SHOULD be ignored.
    pub dw_damage_mask: [u8; 4],
}

impl PixelFormatDescriptor {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let ((n_size, n_size_bytes), (n_version, n_version_bytes)) = (
            crate::parser::read_u16_from_le_bytes(buf)?,
            crate::parser::read_u16_from_le_bytes(buf)?,
        );

        if n_version != 0x0001 {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: format!(
                    "n_version field in PixelFormatDescriptor must be \
                     `0x0001`, but parsed value is {n_version:#06X}"
                ),
            });
        }

        let (
            (dw_flags, dw_flags_bytes),
            (i_pixel_type, i_pixel_type_bytes),
            (c_color_bits, c_color_bits_bytes),
            (c_red_bits, c_red_bits_bytes),
            (c_red_shift, c_red_shift_bytes),
            (c_green_bits, c_green_bits_bytes),
            (c_green_shift, c_green_shift_bytes),
            (c_blue_bits, c_blue_bits_bytes),
            (c_blue_shift, c_blue_shift_bytes),
            (c_alpha_bits, c_alpha_bits_bytes),
            (c_alpha_shift, c_alpha_shift_bytes),
            (c_accum_bits, c_accum_bits_bytes),
            (c_accum_red_bits, c_accum_red_bits_bytes),
            (c_accum_green_bits, c_accum_green_bits_bytes),
            (c_accum_blue_bits, c_accum_blue_bits_bytes),
            (c_accum_alpha_bits, c_accum_alpha_bits_bytes),
            (c_depth_bits, c_depth_bits_bytes),
            (c_stencil_bits, c_stencil_bits_bytes),
            (c_aux_buffers, c_aux_buffers_bytes),
            (i_layer_type, i_layer_type_bytes),
            (b_reserved, b_reserved_bytes),
            (dw_layer_mask, dw_layer_mask_bytes),
            (dw_visible_mask, dw_visible_mask_bytes),
            (dw_damage_mask, dw_damage_mask_bytes),
        ) = (
            DwFlags::parse(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read_u8_from_le_bytes(buf)?,
            crate::parser::read::<_, 4>(buf)?,
            crate::parser::read::<_, 4>(buf)?,
            crate::parser::read::<_, 4>(buf)?,
        );

        Ok((
            Self {
                n_size,
                n_version,
                dw_flags,
                i_pixel_type,
                c_color_bits,
                c_red_bits,
                c_red_shift,
                c_green_bits,
                c_green_shift,
                c_blue_bits,
                c_blue_shift,
                c_alpha_bits,
                c_alpha_shift,
                c_accum_bits,
                c_accum_red_bits,
                c_accum_green_bits,
                c_accum_blue_bits,
                c_accum_alpha_bits,
                c_depth_bits,
                c_stencil_bits,
                c_aux_buffers,
                i_layer_type,
                b_reserved,
                dw_layer_mask,
                dw_visible_mask,
                dw_damage_mask,
            },
            n_size_bytes
                + n_version_bytes
                + dw_flags_bytes
                + i_pixel_type_bytes
                + c_color_bits_bytes
                + c_red_bits_bytes
                + c_red_shift_bytes
                + c_green_bits_bytes
                + c_green_shift_bytes
                + c_blue_bits_bytes
                + c_blue_shift_bytes
                + c_alpha_bits_bytes
                + c_alpha_shift_bytes
                + c_accum_bits_bytes
                + c_accum_red_bits_bytes
                + c_accum_green_bits_bytes
                + c_accum_blue_bits_bytes
                + c_accum_alpha_bits_bytes
                + c_depth_bits_bytes
                + c_stencil_bits_bytes
                + c_aux_buffers_bytes
                + i_layer_type_bytes
                + b_reserved_bytes
                + dw_layer_mask_bytes
                + dw_visible_mask_bytes
                + dw_damage_mask_bytes,
        ))
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug)]
pub struct DwFlags {
    /// The buffer uses RGBA pixels on a palette-managed device. A LogPalette
    /// object is required to achieve the best results for this pixel type.
    /// Colors in the palette SHOULD be specified according to the values of
    /// the cRedBits, cRedShift, cGreenBits, cGreenShift, cBlueBits, and
    /// cBlueShift fields.
    pub PFD_NEED_PALETTE: bool,
    /// The pixel format is natively supported by the operating system; this is
    /// known as the "generic" implementation. (Windows uses this flag to
    /// indicate that the pixel pixel format is supported by GDI.)
    ///
    /// If clear, the pixel format is supported by a device driver or hardware.
    pub PFD_GENERIC_FORMAT: bool,
    /// The pixel buffer supports OpenGL [OPENGL] drawing.
    pub PFD_SUPPORT_OPENGL: bool,
    /// This flag SHOULD be clear, but it MAY be set.
    ///
    /// Windows can use this flag to indicate that the pixel format specified
    /// by this structure is supported by GDI. See [MSDN-GDI+] for more
    /// information.
    ///
    /// Windows can also use this flag to specify single-buffering for the
    /// pixel buffer.
    ///
    /// The PFD_SUPPORT_GDI flag and PFD_DOUBLEBUFFER MUST NOT both be set.
    pub PFD_SUPPORT_GDI: bool,
    /// The pixel buffer can draw to a memory bitmap.
    pub PFD_DRAW_TO_BITMAP: bool,
    /// The pixel buffer can draw to a window or device surface.
    pub PFD_DRAW_TO_WINDOW: bool,
    /// The pixel buffer MAY be stereoscopic; that is, it MAY specify a color
    /// plane that is used to create the illusion of depth in an image.
    ///
    /// Windows implementations do not support this flag.
    pub PFD_STEREO: bool,
    /// The pixel buffer is double-buffered. This flag and PFD_SUPPORT_GDI MUST
    /// NOT both be set.
    pub PFD_DOUBLEBUFFER: bool,
    /// The pixel buffer supports compositing, which indicates that source
    /// pixels MAY overwrite or be combined with background pixels.
    ///
    /// Windows uses this with OpenGL drawing only.
    ///
    /// Windows NT 3.1, Windows NT 3.51, Windows NT 4.0, Windows 98, Windows
    /// 2000, Windows Millennium Edition, Windows XP, and Windows Server 2003
    /// do not support this flag.
    pub PFD_SUPPORT_COMPOSITION: bool,
    /// The pixel buffer supports Direct3D drawing, which accellerated
    /// rendering in three dimensions.
    pub PFD_DIRECT3D_ACCELERATED: bool,
    /// The pixel buffer supports DirectDraw drawing, which allows applications
    /// to have low-level control of the output drawing surface.
    pub PFD_SUPPORT_DIRECTDRAW: bool,
    /// The pixel format is supported by a device driver that accelerates the
    /// generic implementation. If this flag is clear and the
    /// PFD_GENERIC_FORMAT flag is set, the pixel format is supported by the
    /// generic implementation only.
    pub PFD_GENERIC_ACCELERATED: bool,
    /// A device can swap individual color planes with pixel formats that
    /// include double-buffered overlay or underlay color planes. Otherwise all
    /// color planes are swapped together as a group.
    pub PFD_SWAP_LAYER_BUFFERS: bool,
    /// The contents of the back buffer have been copied to the front buffer in
    /// a double-buffered color plane. The contents of the back buffer have not
    /// been affected.
    pub PFD_SWAP_COPY: bool,
    /// The contents of the back buffer have been exchanged with the contents
    /// of the front buffer in a double-buffered color plane.
    pub PFD_SWAP_EXCHANGE: bool,
    /// The output device supports one hardware palette in 256-color mode only.
    /// For such systems to use hardware acceleration, the hardware palette
    /// MUST be in a fixed order (for example, 3-3-2) when in RGBA mode, or
    /// MUST match the LogPalette object when in color table mode.
    pub PFD_NEED_SYSTEM_PALETTE: bool,
    /// The pixel buffer MAY be either monoscopic or stereoscopic.
    pub PFD_STEREO_DONTCARE: bool,
    /// The pixel buffer can be either single or double buffered.
    pub PFD_DOUBLEBUFFER_DONTCARE: bool,
    /// The pixel buffer is not required to include a color plane for depth
    /// effects.
    pub PFD_DEPTH_DONTCARE: bool,
}

impl DwFlags {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        let (v, bytes) = crate::parser::read_u32_from_le_bytes(buf)?;

        Ok((
            Self {
                // P
                PFD_NEED_PALETTE: v & (1 << 31) == (1 << 31),
                // F
                PFD_GENERIC_FORMAT: v & (1 << 30) == (1 << 30),
                // SO
                PFD_SUPPORT_OPENGL: v & (1 << 29) == (1 << 29),
                // G
                PFD_SUPPORT_GDI: v & (1 << 28) == (1 << 28),
                // M
                PFD_DRAW_TO_BITMAP: v & (1 << 27) == (1 << 27),
                // W
                PFD_DRAW_TO_WINDOW: v & (1 << 26) == (1 << 26),
                // S
                PFD_STEREO: v & (1 << 25) == (1 << 25),
                // D
                PFD_DOUBLEBUFFER: v & (1 << 24) == (1 << 24),
                // C
                PFD_SUPPORT_COMPOSITION: v & (1 << 23) == (1 << 23),
                // DA
                PFD_DIRECT3D_ACCELERATED: v & (1 << 22) == (1 << 22),
                // DS
                PFD_SUPPORT_DIRECTDRAW: v & (1 << 21) == (1 << 21),
                // A
                PFD_GENERIC_ACCELERATED: v & (1 << 20) == (1 << 20),
                // SL
                PFD_SWAP_LAYER_BUFFERS: v & (1 << 19) == (1 << 19),
                // SC
                PFD_SWAP_COPY: v & (1 << 18) == (1 << 18),
                // SE
                PFD_SWAP_EXCHANGE: v & (1 << 17) == (1 << 17),
                // SP
                PFD_NEED_SYSTEM_PALETTE: v & (1 << 16) == (1 << 16),
                // SD
                PFD_STEREO_DONTCARE: v & (1 << 3) == (1 << 3),
                // DD
                PFD_DOUBLEBUFFER_DONTCARE: v & (1 << 2) == (1 << 2),
                // DP
                PFD_DEPTH_DONTCARE: v & (1 << 1) == (1 << 1),
            },
            bytes,
        ))
    }
}
