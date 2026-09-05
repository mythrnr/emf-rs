use crate::{imports::*, parser::*};

#[derive(Clone, Debug, snafu::prelude::Snafu)]
pub enum PlayError {
    #[snafu(display("failed to generate: {cause}"))]
    FailedGenerate { cause: String },
    #[snafu(display("invalid brush: {cause}"))]
    InvalidBrush { cause: String },
    #[snafu(display("invalid record: {cause}"))]
    InvalidRecord { cause: String },
    #[snafu(display("unexpected graphics object: {cause}"))]
    UnexpectedGraphicsObject { cause: String },
    #[snafu(display("unknown: {cause}"))]
    Unknown { cause: String },
}

impl PlayError {
    /// Constructs a [`PlayError::InvalidRecord`] from a plain message,
    /// sparing call sites the struct-literal ceremony.
    pub fn invalid_record(cause: impl Into<String>) -> Self {
        Self::InvalidRecord { cause: cause.into() }
    }
}

impl From<ParseError> for PlayError {
    fn from(source: ParseError) -> Self {
        Self::invalid_record(source.to_string())
    }
}

/// Generates a default no-op handler for each `(method, record type)`
/// pair. The record type doubles as an intra-doc link, resolved in the
/// scope of the invoking module so the same macro serves both the EMF
/// and EMF+ player traits.
macro_rules! default_record_handlers {
    ($($method:ident : $record:ty),+ $(,)?) => {
        $(
            #[doc = concat!(
                "Renders the [`", stringify!($record), "`] record."
            )]
            fn $method(
                self,
                _record_number: usize,
                _record: $record,
            ) -> Result<Self, PlayError> {
                info!(concat!(
                    stringify!($record),
                    ": skipped (not implemented)"
                ));
                Ok(self)
            }
        )+
    };
}

pub(crate) use default_record_handlers;

/// Processes the EMF records of a metafile.
///
/// Methods take `self` by value and return `Self`, so a player folds its
/// state through the record stream. Every record method defaults to a
/// no-op; a player implements [`generate`] and overrides only the
/// records it renders.
///
/// [`EMFConverter`](crate::converter::EMFConverter) drives the trait: it
/// parses the record stream and calls the method matching each record.
///
/// [`generate`]: Player::generate
pub trait Player: Sized {
    /// Call after converting to write output.
    fn generate(self) -> Result<Vec<u8>, PlayError>;

    // .
    // .
    // Functions to handle Bitmap Record (2.3.1)
    // .
    // .
    default_record_handlers! {
        alpha_blend: EMR_ALPHABLEND,
        bit_blt: EMR_BITBLT,
        mask_blt: EMR_MASKBLT,
        plg_blt: EMR_PLGBLT,
        set_dibits_to_device: EMR_SETDIBITSTODEVICE,
        stretch_blt: EMR_STRETCHBLT,
        stretch_dibits: EMR_STRETCHDIBITS,
        transparent_blt: EMR_TRANSPARENTBLT,
    }

    // .
    // .
    // Functions to handle Clipping Record (2.3.2)
    // .
    // .
    default_record_handlers! {
        exclude_clip_rect: EMR_EXCLUDECLIPRECT,
        ext_select_clip_rgn: EMR_EXTSELECTCLIPRGN,
        intersect_clip_rect: EMR_INTERSECTCLIPRECT,
        offset_clip_rgn: EMR_OFFSETCLIPRGN,
        select_clip_path: EMR_SELECTCLIPPATH,
        set_meta_rgn: EMR_SETMETARGN,
    }

    // .
    // .
    // Functions to handle Comment Record (2.3.3)
    // .
    // .
    default_record_handlers! {
        comment: EMR_COMMENT,
    }

    // .
    // .
    // Functions to handle Control Record (2.3.4)
    // .
    // .
    default_record_handlers! {
        eof: EMR_EOF,
        header: EMR_HEADER,
    }

    // .
    // .
    // Functions to handle Drawing Record (2.3.5)
    // .
    // .
    default_record_handlers! {
        angle_arc: EMR_ANGLEARC,
        arc: EMR_ARC,
        arc_to: EMR_ARCTO,
        chord: EMR_CHORD,
        ellipse: EMR_ELLIPSE,
        ext_flood_fill: EMR_EXTFLOODFILL,
        ext_text_out_a: EMR_EXTTEXTOUTA,
        ext_text_out_w: EMR_EXTTEXTOUTW,
        fill_path: EMR_FILLPATH,
        fill_rgn: EMR_FILLRGN,
        frame_rgn: EMR_FRAMERGN,
        gradient_fill: EMR_GRADIENTFILL,
        line_to: EMR_LINETO,
        paint_rgn: EMR_PAINTRGN,
        pie: EMR_PIE,
        poly_bezier: EMR_POLYBEZIER,
        poly_bezier_16: EMR_POLYBEZIER16,
        poly_bezier_to: EMR_POLYBEZIERTO,
        poly_bezier_to_16: EMR_POLYBEZIERTO16,
        poly_draw: EMR_POLYDRAW,
        poly_draw_16: EMR_POLYDRAW16,
        poly_polygon: EMR_POLYPOLYGON,
        poly_polygon_16: EMR_POLYPOLYGON16,
        poly_polyline: EMR_POLYPOLYLINE,
        poly_polyline_16: EMR_POLYPOLYLINE16,
        poly_text_out_a: EMR_POLYTEXTOUTA,
        poly_text_out_w: EMR_POLYTEXTOUTW,
        polygon: EMR_POLYGON,
        polygon_16: EMR_POLYGON16,
        polyline: EMR_POLYLINE,
        polyline_16: EMR_POLYLINE16,
        polyline_to: EMR_POLYLINETO,
        polyline_to_16: EMR_POLYLINETO16,
        rectangle: EMR_RECTANGLE,
        round_rect: EMR_ROUNDRECT,
        set_pixel_v: EMR_SETPIXELV,
        small_text_out: EMR_SMALLTEXTOUT,
        stroke_and_fill_path: EMR_STROKEANDFILLPATH,
        stroke_path: EMR_STROKEPATH,
    }

    // .
    // .
    // Functions to handle Escape Record (2.3.6)
    // .
    // .
    default_record_handlers! {
        draw_escape: EMR_DRAWESCAPE,
        ext_escape: EMR_EXTESCAPE,
        named_escape: EMR_NAMEDESCAPE,
    }

    // .
    // .
    // Functions to handle Object Creation Record (2.3.7)
    // .
    // .
    default_record_handlers! {
        create_brush_indirect: EMR_CREATEBRUSHINDIRECT,
        create_color_space: EMR_CREATECOLORSPACE,
        create_color_space_w: EMR_CREATECOLORSPACEW,
        create_dib_pattern_brush_pt: EMR_CREATEDIBPATTERNBRUSHPT,
        create_mono_brush: EMR_CREATEMONOBRUSH,
        create_palette: EMR_CREATEPALETTE,
        create_pen: EMR_CREATEPEN,
        ext_create_font_indirect_w: EMR_EXTCREATEFONTINDIRECTW,
        ext_create_pen: EMR_EXTCREATEPEN,
    }

    // .
    // .
    // Functions to handle Object Manipulation Record (2.3.8)
    // .
    // .
    default_record_handlers! {
        color_correct_palette: EMR_COLORCORRECTPALETTE,
        delete_color_space: EMR_DELETECOLORSPACE,
        delete_object: EMR_DELETEOBJECT,
        resize_palette: EMR_RESIZEPALETTE,
        select_object: EMR_SELECTOBJECT,
        select_palette: EMR_SELECTPALETTE,
        set_color_space: EMR_SETCOLORSPACE,
        set_palette_entries: EMR_SETPALETTEENTRIES,
    }

    // .
    // .
    // Functions to handle OpenGL Record (2.3.9)
    // .
    // .
    default_record_handlers! {
        gls_bounded_record: EMR_GLSBOUNDEDRECORD,
        gls_record: EMR_GLSRECORD,
    }

    // .
    // .
    // Functions to handle Path Bracket Record (2.3.10)
    // .
    // .
    default_record_handlers! {
        abort_path: EMR_ABORTPATH,
        begin_path: EMR_BEGINPATH,
        close_figure: EMR_CLOSEFIGURE,
        end_path: EMR_ENDPATH,
        flatten_path: EMR_FLATTENPATH,
        widen_path: EMR_WIDENPATH,
    }

    // .
    // .
    // Functions to handle State Record (2.3.11)
    // .
    // .
    default_record_handlers! {
        color_match_to_target_w: EMR_COLORMATCHTOTARGETW,
        force_ufi_mapping: EMR_FORCEUFIMAPPING,
        invert_rgn: EMR_INVERTRGN,
        move_to_ex: EMR_MOVETOEX,
        pixel_format: EMR_PIXELFORMAT,
        realize_palette: EMR_REALIZEPALETTE,
        restore_dc: EMR_RESTOREDC,
        save_dc: EMR_SAVEDC,
        scale_viewport_ext_ex: EMR_SCALEVIEWPORTEXTEX,
        scale_window_ext_ex: EMR_SCALEWINDOWEXTEX,
        set_arc_direction: EMR_SETARCDIRECTION,
        set_bk_color: EMR_SETBKCOLOR,
        set_bk_mode: EMR_SETBKMODE,
        set_brush_org_ex: EMR_SETBRUSHORGEX,
        set_color_adjustment: EMR_SETCOLORADJUSTMENT,
        set_icm_mode: EMR_SETICMMODE,
        set_icm_profile_a: EMR_SETICMPROFILEA,
        set_icm_profile_w: EMR_SETICMPROFILEW,
        set_layout: EMR_SETLAYOUT,
        set_linked_ufis: EMR_SETLINKEDUFIS,
        set_map_mode: EMR_SETMAPMODE,
        set_mapper_flags: EMR_SETMAPPERFLAGS,
        set_miter_limit: EMR_SETMITERLIMIT,
        set_polyfill_mode: EMR_SETPOLYFILLMODE,
        set_rop2: EMR_SETROP2,
        set_stretch_blt_mode: EMR_SETSTRETCHBLTMODE,
        set_text_align: EMR_SETTEXTALIGN,
        set_text_color: EMR_SETTEXTCOLOR,
        set_text_justification: EMR_SETTEXTJUSTIFICATION,
        set_viewport_ext_ex: EMR_SETVIEWPORTEXTEX,
        set_viewport_org_ex: EMR_SETVIEWPORTORGEX,
        set_window_ext_ex: EMR_SETWINDOWEXTEX,
        set_window_org_ex: EMR_SETWINDOWORGEX,
    }

    // .
    // .
    // Functions to handle Transform Record (2.3.12)
    // .
    // .
    default_record_handlers! {
        modify_world_transform: EMR_MODIFYWORLDTRANSFORM,
        set_world_transform: EMR_SETWORLDTRANSFORM,
    }
}
