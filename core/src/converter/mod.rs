mod playback_device_context;
mod player;

pub use self::player::*;
use crate::{imports::*, parser::*};

#[cfg(feature = "svg")]
mod svg;

#[cfg(feature = "svg")]
pub use self::svg::*;

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum ConvertError {
    #[snafu(display("parse error: {source}"))]
    ParseError { source: ParseError },
    #[snafu(display("play error: {source}"))]
    PlayError { source: crate::converter::PlayError },
    #[snafu(display("WMF convert error: {source}"))]
    WMFConvertError { source: wmf_core::converter::ConvertError },
    #[snafu(display("I/O error: {cause}"))]
    IoError { cause: String },
}

impl From<ParseError> for ConvertError {
    fn from(source: ParseError) -> Self {
        Self::ParseError { source }
    }
}

impl From<crate::converter::PlayError> for ConvertError {
    fn from(source: crate::converter::PlayError) -> Self {
        Self::PlayError { source }
    }
}

pub struct EMFConverter<B, P, WP> {
    buffer: B,
    player: P,
    wmf_player: WP,
}

impl<B, P, WP> EMFConverter<B, P, WP> {
    pub fn new(buffer: B, player: P, wmf_player: WP) -> Self {
        Self { buffer, player, wmf_player }
    }
}

impl<B, P, WP> EMFConverter<B, P, WP>
where
    B: crate::Read,
    P: crate::converter::Player,
    WP: wmf_core::converter::Player,
{
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn run(self) -> Result<Vec<u8>, ConvertError> {
        let Self { mut buffer, mut player, wmf_player } = self;

        let buffer = {
            let mut buf = vec![];

            // read to end
            loop {
                let mut b = vec![0; 8192];
                let read = buffer.read(&mut b).map_err(|err| {
                    ConvertError::IoError { cause: format!("{err:?}") }
                })?;

                buf.extend(&b[..read]);

                if read < 8192 {
                    break;
                }
            }

            buf
        };

        let mut b = &buffer[0..4];

        match RecordType::parse(&mut b) {
            Ok((RecordType::EMR_HEADER, _)) => {}
            Ok((record_type, _)) => {
                return Err(ConvertError::ParseError {
                    source: ParseError::UnexpectedPattern {
                        cause: format!(
                            "First 4 byte of file is expected {:010X}, but \
                             parsed value is {:010X}({record_type:?})",
                            RecordType::EMR_HEADER as u32,
                            record_type as u32,
                        ),
                    },
                })
            }
            Err(_) => {
                info!("This file may be WMF. Try to convert as WMF.");

                let wmf_converter = wmf_core::converter::WMFConverter::new(
                    buffer.as_slice(),
                    wmf_player,
                );

                return wmf_converter.run().map_err(|source| {
                    ConvertError::WMFConvertError { source }
                });
            }
        };

        let buf = &mut buffer.as_slice();
        let header = EMR_HEADER::parse(buf)?;
        let mut record_number = 0;

        debug!(%record_number, ?header);
        player = player.header(record_number, header)?;

        loop {
            record_number += 1;

            let ((record_type, record_type_bytes), (size, size_bytes)) = (
                RecordType::parse(buf).map_err(ParseError::from)?,
                read_u32_from_le_bytes(buf).map_err(ParseError::from)?,
            );

            let mut size = Size::from(size);
            size.consume(record_type_bytes + size_bytes);

            if size.byte_count() == 0 {
                debug!(%size, "skip parsing zero-sized record");

                continue;
            }

            match record_type {
                // bitmap record
                RecordType::EMR_ALPHABLEND => {
                    let record = EMR_ALPHABLEND::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.alpha_blend(record_number, record)?;
                }
                RecordType::EMR_BITBLT => {
                    let record = EMR_BITBLT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.bit_blt(record_number, record)?;
                }
                RecordType::EMR_MASKBLT => {
                    let record = EMR_MASKBLT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.mask_blt(record_number, record)?;
                }
                RecordType::EMR_PLGBLT => {
                    let record = EMR_PLGBLT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.plg_blt(record_number, record)?;
                }
                RecordType::EMR_SETDIBITSTODEVICE => {
                    let record =
                        EMR_SETDIBITSTODEVICE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_dibits_to_device(record_number, record)?;
                }
                RecordType::EMR_STRETCHBLT => {
                    let record = EMR_STRETCHBLT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.stretch_blt(record_number, record)?;
                }
                RecordType::EMR_STRETCHDIBITS => {
                    let record =
                        EMR_STRETCHDIBITS::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.stretch_dibits(record_number, record)?;
                }
                RecordType::EMR_TRANSPARENTBLT => {
                    let record =
                        EMR_TRANSPARENTBLT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.transparent_blt(record_number, record)?;
                }
                // clipping record
                RecordType::EMR_EXCLUDECLIPRECT => {
                    let record =
                        EMR_EXCLUDECLIPRECT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.exclude_clip_rect(record_number, record)?;
                }
                RecordType::EMR_EXTSELECTCLIPRGN => {
                    let record =
                        EMR_EXTSELECTCLIPRGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.ext_select_clip_rgn(record_number, record)?;
                }
                RecordType::EMR_INTERSECTCLIPRECT => {
                    let record =
                        EMR_INTERSECTCLIPRECT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.intersect_clip_rect(record_number, record)?;
                }
                RecordType::EMR_OFFSETCLIPRGN => {
                    let record =
                        EMR_OFFSETCLIPRGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.offset_clip_rgn(record_number, record)?;
                }
                RecordType::EMR_SELECTCLIPPATH => {
                    let record =
                        EMR_SELECTCLIPPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.select_clip_path(record_number, record)?;
                }
                RecordType::EMR_SETMETARGN => {
                    let record = EMR_SETMETARGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_meta_rgn(record_number, record)?;
                }
                // comment record
                RecordType::EMR_COMMENT => {
                    let record = EMR_COMMENT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.comment(record_number, record)?;
                }
                // control record
                RecordType::EMR_EOF => {
                    let record = EMR_EOF::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.eof(record_number, record)?;
                    break;
                }
                RecordType::EMR_HEADER => {
                    // NOOP
                }
                // drawing record
                RecordType::EMR_ANGLEARC => {
                    let record = EMR_ANGLEARC::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.angle_arc(record_number, record)?;
                }
                RecordType::EMR_ARC => {
                    let record = EMR_ARC::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.arc(record_number, record)?;
                }
                RecordType::EMR_ARCTO => {
                    let record = EMR_ARCTO::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.arc_to(record_number, record)?;
                }
                RecordType::EMR_CHORD => {
                    let record = EMR_CHORD::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.chord(record_number, record)?;
                }
                RecordType::EMR_ELLIPSE => {
                    let record = EMR_ELLIPSE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.ellipse(record_number, record)?;
                }
                RecordType::EMR_EXTFLOODFILL => {
                    let record =
                        EMR_EXTFLOODFILL::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.ext_flood_fill(record_number, record)?;
                }
                RecordType::EMR_EXTTEXTOUTA => {
                    let record =
                        EMR_EXTTEXTOUTA::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.ext_text_out_a(record_number, record)?;
                }
                RecordType::EMR_EXTTEXTOUTW => {
                    let record =
                        EMR_EXTTEXTOUTW::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.ext_text_out_w(record_number, record)?;
                }
                RecordType::EMR_FILLPATH => {
                    let record = EMR_FILLPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.fill_path(record_number, record)?;
                }
                RecordType::EMR_FILLRGN => {
                    let record = EMR_FILLRGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.fill_rgn(record_number, record)?;
                }
                RecordType::EMR_FRAMERGN => {
                    let record = EMR_FRAMERGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.frame_rgn(record_number, record)?;
                }
                RecordType::EMR_GRADIENTFILL => {
                    let record =
                        EMR_GRADIENTFILL::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.gradient_fill(record_number, record)?;
                }
                RecordType::EMR_LINETO => {
                    let record = EMR_LINETO::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.line_to(record_number, record)?;
                }
                RecordType::EMR_PAINTRGN => {
                    let record = EMR_PAINTRGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.paint_rgn(record_number, record)?;
                }
                RecordType::EMR_PIE => {
                    let record = EMR_PIE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.pie(record_number, record)?;
                }
                RecordType::EMR_POLYBEZIER => {
                    let record = EMR_POLYBEZIER::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_bezier(record_number, record)?;
                }
                RecordType::EMR_POLYBEZIER16 => {
                    let record =
                        EMR_POLYBEZIER16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_bezier_16(record_number, record)?;
                }
                RecordType::EMR_POLYBEZIERTO => {
                    let record =
                        EMR_POLYBEZIERTO::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_bezier_to(record_number, record)?;
                }
                RecordType::EMR_POLYBEZIERTO16 => {
                    let record =
                        EMR_POLYBEZIERTO16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_bezier_to_16(record_number, record)?;
                }
                RecordType::EMR_POLYDRAW => {
                    let record = EMR_POLYDRAW::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_draw(record_number, record)?;
                }
                RecordType::EMR_POLYDRAW16 => {
                    let record = EMR_POLYDRAW16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_draw_16(record_number, record)?;
                }
                RecordType::EMR_POLYPOLYGON => {
                    let record =
                        EMR_POLYPOLYGON::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_polygon(record_number, record)?;
                }
                RecordType::EMR_POLYPOLYGON16 => {
                    let record =
                        EMR_POLYPOLYGON16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_polygon_16(record_number, record)?;
                }
                RecordType::EMR_POLYPOLYLINE => {
                    let record =
                        EMR_POLYPOLYLINE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_polyline(record_number, record)?;
                }
                RecordType::EMR_POLYPOLYLINE16 => {
                    let record =
                        EMR_POLYPOLYLINE16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_polyline_16(record_number, record)?;
                }
                RecordType::EMR_POLYTEXTOUTA => {
                    let record =
                        EMR_POLYTEXTOUTA::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_text_out_a(record_number, record)?;
                }
                RecordType::EMR_POLYTEXTOUTW => {
                    let record =
                        EMR_POLYTEXTOUTW::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.poly_text_out_w(record_number, record)?;
                }
                RecordType::EMR_POLYGON => {
                    let record = EMR_POLYGON::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.polygon(record_number, record)?;
                }
                RecordType::EMR_POLYGON16 => {
                    let record = EMR_POLYGON16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.polygon_16(record_number, record)?;
                }
                RecordType::EMR_POLYLINE => {
                    let record = EMR_POLYLINE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.polyline(record_number, record)?;
                }
                RecordType::EMR_POLYLINE16 => {
                    let record = EMR_POLYLINE16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.polyline_16(record_number, record)?;
                }
                RecordType::EMR_POLYLINETO => {
                    let record = EMR_POLYLINETO::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.polyline_to(record_number, record)?;
                }
                RecordType::EMR_POLYLINETO16 => {
                    let record =
                        EMR_POLYLINETO16::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.polyline_to_16(record_number, record)?;
                }

                RecordType::EMR_RECTANGLE => {
                    let record = EMR_RECTANGLE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.rectangle(record_number, record)?;
                }
                RecordType::EMR_ROUNDRECT => {
                    let record = EMR_ROUNDRECT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.round_rect(record_number, record)?;
                }
                RecordType::EMR_SETPIXELV => {
                    let record = EMR_SETPIXELV::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_pixel_v(record_number, record)?;
                }
                RecordType::EMR_SMALLTEXTOUT => {
                    let record =
                        EMR_SMALLTEXTOUT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.small_text_out(record_number, record)?;
                }
                RecordType::EMR_STROKEANDFILLPATH => {
                    let record =
                        EMR_STROKEANDFILLPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.stroke_and_fill_path(record_number, record)?;
                }
                RecordType::EMR_STROKEPATH => {
                    let record = EMR_STROKEPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.stroke_path(record_number, record)?;
                }
                // escape record
                RecordType::EMR_DRAWESCAPE => {
                    let record = EMR_DRAWESCAPE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.draw_escape(record_number, record)?;
                }
                RecordType::EMR_EXTESCAPE => {
                    let record = EMR_EXTESCAPE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.ext_escape(record_number, record)?;
                }
                RecordType::EMR_NAMEDESCAPE => {
                    let record =
                        EMR_NAMEDESCAPE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.named_escape(record_number, record)?;
                }
                // object creation record
                RecordType::EMR_CREATEBRUSHINDIRECT => {
                    let record =
                        EMR_CREATEBRUSHINDIRECT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.create_brush_indirect(record_number, record)?;
                }
                RecordType::EMR_CREATECOLORSPACE => {
                    let record =
                        EMR_CREATECOLORSPACE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.create_color_space(record_number, record)?;
                }
                RecordType::EMR_CREATECOLORSPACEW => {
                    let record =
                        EMR_CREATECOLORSPACEW::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.create_color_space_w(record_number, record)?;
                }
                RecordType::EMR_CREATEDIBPATTERNBRUSHPT => {
                    let record = EMR_CREATEDIBPATTERNBRUSHPT::parse(
                        buf,
                        record_type,
                        size,
                    )?;

                    debug!(%record_number, ?record);
                    player = player
                        .create_dib_pattern_brush_pt(record_number, record)?;
                }
                RecordType::EMR_CREATEMONOBRUSH => {
                    let record =
                        EMR_CREATEMONOBRUSH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.create_mono_brush(record_number, record)?;
                }
                RecordType::EMR_CREATEPALETTE => {
                    let record =
                        EMR_CREATEPALETTE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.create_palette(record_number, record)?;
                }
                RecordType::EMR_CREATEPEN => {
                    let record = EMR_CREATEPEN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.create_pen(record_number, record)?;
                }
                RecordType::EMR_EXTCREATEFONTINDIRECTW => {
                    let record = EMR_EXTCREATEFONTINDIRECTW::parse(
                        buf,
                        record_type,
                        size,
                    )?;

                    debug!(%record_number, ?record);
                    player = player
                        .ext_create_font_indirect_w(record_number, record)?;
                }
                RecordType::EMR_EXTCREATEPEN => {
                    let record =
                        EMR_EXTCREATEPEN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.ext_create_pen(record_number, record)?;
                }
                // object manipulation record
                RecordType::EMR_COLORCORRECTPALETTE => {
                    let record =
                        EMR_COLORCORRECTPALETTE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.color_correct_palette(record_number, record)?;
                }
                RecordType::EMR_DELETECOLORSPACE => {
                    let record =
                        EMR_DELETECOLORSPACE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.delete_color_space(record_number, record)?;
                }
                RecordType::EMR_DELETEOBJECT => {
                    let record =
                        EMR_DELETEOBJECT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.delete_object(record_number, record)?;
                }
                RecordType::EMR_RESIZEPALETTE => {
                    let record =
                        EMR_RESIZEPALETTE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.resize_palette(record_number, record)?;
                }
                RecordType::EMR_SELECTOBJECT => {
                    let record =
                        EMR_SELECTOBJECT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.select_object(record_number, record)?;
                }
                RecordType::EMR_SELECTPALETTE => {
                    let record =
                        EMR_SELECTPALETTE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.select_palette(record_number, record)?;
                }
                RecordType::EMR_SETCOLORSPACE => {
                    let record =
                        EMR_SETCOLORSPACE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_color_space(record_number, record)?;
                }
                RecordType::EMR_SETPALETTEENTRIES => {
                    let record =
                        EMR_SETPALETTEENTRIES::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_palette_entries(record_number, record)?;
                }
                // opengl record
                RecordType::EMR_GLSBOUNDEDRECORD => {
                    let record =
                        EMR_GLSBOUNDEDRECORD::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.gls_bounded_record(record_number, record)?;
                }
                RecordType::EMR_GLSRECORD => {
                    let record = EMR_GLSRECORD::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.gls_record(record_number, record)?;
                }
                // path bracket record
                RecordType::EMR_ABORTPATH => {
                    let record = EMR_ABORTPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.abort_path(record_number, record)?;
                }
                RecordType::EMR_BEGINPATH => {
                    let record = EMR_BEGINPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.begin_path(record_number, record)?;
                }
                RecordType::EMR_CLOSEFIGURE => {
                    let record =
                        EMR_CLOSEFIGURE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.close_figure(record_number, record)?;
                }
                RecordType::EMR_ENDPATH => {
                    let record = EMR_ENDPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.end_path(record_number, record)?;
                }
                RecordType::EMR_FLATTENPATH => {
                    let record =
                        EMR_FLATTENPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.flatten_path(record_number, record)?;
                }
                RecordType::EMR_WIDENPATH => {
                    let record = EMR_WIDENPATH::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.widen_path(record_number, record)?;
                }
                // state record
                RecordType::EMR_COLORMATCHTOTARGETW => {
                    let record =
                        EMR_COLORMATCHTOTARGETW::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player
                        .color_match_to_target_w(record_number, record)?;
                }
                RecordType::EMR_FORCEUFIMAPPING => {
                    let record =
                        EMR_FORCEUFIMAPPING::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.force_ufi_mapping(record_number, record)?;
                }
                RecordType::EMR_INVERTRGN => {
                    let record = EMR_INVERTRGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.invert_rgn(record_number, record)?;
                }
                RecordType::EMR_MOVETOEX => {
                    let record = EMR_MOVETOEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.move_to_ex(record_number, record)?;
                }
                RecordType::EMR_PIXELFORMAT => {
                    let record =
                        EMR_PIXELFORMAT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.pixel_format(record_number, record)?;
                }
                RecordType::EMR_REALIZEPALETTE => {
                    let record =
                        EMR_REALIZEPALETTE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.realize_palette(record_number, record)?;
                }
                RecordType::EMR_RESTOREDC => {
                    let record = EMR_RESTOREDC::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.restore_dc(record_number, record)?;
                }
                RecordType::EMR_SAVEDC => {
                    let record = EMR_SAVEDC::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.save_dc(record_number, record)?;
                }
                RecordType::EMR_SCALEVIEWPORTEXTEX => {
                    let record =
                        EMR_SCALEVIEWPORTEXTEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.scale_viewport_ext_ex(record_number, record)?;
                }
                RecordType::EMR_SCALEWINDOWEXTEX => {
                    let record =
                        EMR_SCALEWINDOWEXTEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.scale_window_ext_ex(record_number, record)?;
                }
                RecordType::EMR_SETARCDIRECTION => {
                    let record =
                        EMR_SETARCDIRECTION::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_arc_direction(record_number, record)?;
                }
                RecordType::EMR_SETBKCOLOR => {
                    let record = EMR_SETBKCOLOR::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_bk_color(record_number, record)?;
                }
                RecordType::EMR_SETBKMODE => {
                    let record = EMR_SETBKMODE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_bk_mode(record_number, record)?;
                }
                RecordType::EMR_SETBRUSHORGEX => {
                    let record =
                        EMR_SETBRUSHORGEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_brush_org_ex(record_number, record)?;
                }
                RecordType::EMR_SETCOLORADJUSTMENT => {
                    let record =
                        EMR_SETCOLORADJUSTMENT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_color_adjustment(record_number, record)?;
                }
                RecordType::EMR_SETICMMODE => {
                    let record = EMR_SETICMMODE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_icm_mode(record_number, record)?;
                }
                RecordType::EMR_SETICMPROFILEA => {
                    let record =
                        EMR_SETICMPROFILEA::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_icm_profile_a(record_number, record)?;
                }
                RecordType::EMR_SETICMPROFILEW => {
                    let record =
                        EMR_SETICMPROFILEW::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_icm_profile_w(record_number, record)?;
                }
                RecordType::EMR_SETLAYOUT => {
                    let record = EMR_SETLAYOUT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_layout(record_number, record)?;
                }
                RecordType::EMR_SETLINKEDUFIS => {
                    let record =
                        EMR_SETLINKEDUFIS::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_linked_ufis(record_number, record)?;
                }
                RecordType::EMR_SETMAPMODE => {
                    let record = EMR_SETMAPMODE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_map_mode(record_number, record)?;
                }
                RecordType::EMR_SETMAPPERFLAGS => {
                    let record =
                        EMR_SETMAPPERFLAGS::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_mapper_flags(record_number, record)?;
                }
                RecordType::EMR_SETMITERLIMIT => {
                    let record =
                        EMR_SETMITERLIMIT::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_miter_limit(record_number, record)?;
                }
                RecordType::EMR_SETPOLYFILLMODE => {
                    let record =
                        EMR_SETPOLYFILLMODE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_polyfill_mode(record_number, record)?;
                }
                RecordType::EMR_SETROP2 => {
                    let record = EMR_SETROP2::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_rop2(record_number, record)?;
                }
                RecordType::EMR_SETSTRETCHBLTMODE => {
                    let record =
                        EMR_SETSTRETCHBLTMODE::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_stretch_blt_mode(record_number, record)?;
                }
                RecordType::EMR_SETTEXTALIGN => {
                    let record =
                        EMR_SETTEXTALIGN::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_text_align(record_number, record)?;
                }
                RecordType::EMR_SETTEXTCOLOR => {
                    let record =
                        EMR_SETTEXTCOLOR::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_text_color(record_number, record)?;
                }
                RecordType::EMR_SETTEXTJUSTIFICATION => {
                    let record = EMR_SETTEXTJUSTIFICATION::parse(
                        buf,
                        record_type,
                        size,
                    )?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_text_justification(record_number, record)?;
                }
                RecordType::EMR_SETVIEWPORTEXTEX => {
                    let record =
                        EMR_SETVIEWPORTEXTEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_viewport_ext_ex(record_number, record)?;
                }
                RecordType::EMR_SETVIEWPORTORGEX => {
                    let record =
                        EMR_SETVIEWPORTORGEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_viewport_org_ex(record_number, record)?;
                }
                RecordType::EMR_SETWINDOWEXTEX => {
                    let record =
                        EMR_SETWINDOWEXTEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_window_ext_ex(record_number, record)?;
                }
                RecordType::EMR_SETWINDOWORGEX => {
                    let record =
                        EMR_SETWINDOWORGEX::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player = player.set_window_org_ex(record_number, record)?;
                }
                // transform record
                RecordType::EMR_MODIFYWORLDTRANSFORM => {
                    let record = EMR_MODIFYWORLDTRANSFORM::parse(
                        buf,
                        record_type,
                        size,
                    )?;

                    debug!(%record_number, ?record);
                    player =
                        player.modify_world_transform(record_number, record)?;
                }
                RecordType::EMR_SETWORLDTRANSFORM => {
                    let record =
                        EMR_SETWORLDTRANSFORM::parse(buf, record_type, size)?;

                    debug!(%record_number, ?record);
                    player =
                        player.set_world_transform(record_number, record)?;
                }
            };
        }

        Ok(player.generate()?)
    }
}
