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

pub trait Player: Sized {
    /// Call after converting to write output.
    fn generate(self) -> Result<Vec<u8>, PlayError>;

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    fn alpha_blend(self, record: EMR_ALPHABLEND) -> Result<Self, PlayError>;
    fn bit_blt(self, record: EMR_BITBLT) -> Result<Self, PlayError>;
    fn mask_blt(self, record: EMR_MASKBLT) -> Result<Self, PlayError>;
    fn plg_blt(self, record: EMR_PLGBLT) -> Result<Self, PlayError>;
    fn set_dibits_to_device(
        self,
        record: EMR_SETDIBITSTODEVICE,
    ) -> Result<Self, PlayError>;
    fn stretch_blt(self, record: EMR_STRETCHBLT) -> Result<Self, PlayError>;
    fn stretch_dibits(
        self,
        record: EMR_STRETCHDIBITS,
    ) -> Result<Self, PlayError>;
    fn transparent_blt(
        self,
        record: EMR_TRANSPARENTBLT,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Clipping Record
    // .
    // .
    fn exclude_clip_rect(
        self,
        record: EMR_EXCLUDECLIPRECT,
    ) -> Result<Self, PlayError>;
    fn ext_select_clip_rgn(
        self,
        record: EMR_EXTSELECTCLIPRGN,
    ) -> Result<Self, PlayError>;
    fn intersect_clip_rect(
        self,
        record: EMR_INTERSECTCLIPRECT,
    ) -> Result<Self, PlayError>;
    fn offset_clip_rgn(
        self,
        record: EMR_OFFSETCLIPRGN,
    ) -> Result<Self, PlayError>;
    fn select_clip_path(
        self,
        record: EMR_SELECTCLIPPATH,
    ) -> Result<Self, PlayError>;
    fn set_meta_rgn(self, record: EMR_SETMETARGN) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Comment Record
    // .
    // .
    fn comment(self, record: EMR_COMMENT) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    fn eof(self, record: EMR_EOF) -> Result<Self, PlayError>;
    fn header(self, record: EMR_HEADER) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .
    fn angle_arc(self, record: EMR_ANGLEARC) -> Result<Self, PlayError>;
    fn arc(self, record: EMR_ARC) -> Result<Self, PlayError>;
    fn arc_to(self, record: EMR_ARCTO) -> Result<Self, PlayError>;
    fn chord(self, record: EMR_CHORD) -> Result<Self, PlayError>;
    fn ellipse(self, record: EMR_ELLIPSE) -> Result<Self, PlayError>;
    fn ext_flood_fill(
        self,
        record: EMR_EXTFLOODFILL,
    ) -> Result<Self, PlayError>;
    fn ext_text_out_a(self, record: EMR_EXTTEXTOUTA)
        -> Result<Self, PlayError>;
    fn ext_text_out_w(self, record: EMR_EXTTEXTOUTW)
        -> Result<Self, PlayError>;
    fn fill_path(self, record: EMR_FILLPATH) -> Result<Self, PlayError>;
    fn fill_rgn(self, record: EMR_FILLRGN) -> Result<Self, PlayError>;
    fn frame_rgn(self, record: EMR_FRAMERGN) -> Result<Self, PlayError>;
    fn gradient_fill(self, record: EMR_GRADIENTFILL)
        -> Result<Self, PlayError>;
    fn line_to(self, record: EMR_LINETO) -> Result<Self, PlayError>;
    fn paint_rgn(self, record: EMR_PAINTRGN) -> Result<Self, PlayError>;
    fn pie(self, record: EMR_PIE) -> Result<Self, PlayError>;
    fn poly_bezier(self, record: EMR_POLYBEZIER) -> Result<Self, PlayError>;
    fn poly_bezier_16(
        self,
        record: EMR_POLYBEZIER16,
    ) -> Result<Self, PlayError>;
    fn poly_bezier_to(
        self,
        record: EMR_POLYBEZIERTO,
    ) -> Result<Self, PlayError>;
    fn poly_bezier_to_16(
        self,
        record: EMR_POLYBEZIERTO16,
    ) -> Result<Self, PlayError>;
    fn poly_draw(self, record: EMR_POLYDRAW) -> Result<Self, PlayError>;
    fn poly_draw_16(self, record: EMR_POLYDRAW16) -> Result<Self, PlayError>;
    fn poly_polygon(self, record: EMR_POLYPOLYGON) -> Result<Self, PlayError>;
    fn poly_polygon_16(
        self,
        record: EMR_POLYPOLYGON16,
    ) -> Result<Self, PlayError>;
    fn poly_polyline(self, record: EMR_POLYPOLYLINE)
        -> Result<Self, PlayError>;
    fn poly_polyline_16(
        self,
        record: EMR_POLYPOLYLINE16,
    ) -> Result<Self, PlayError>;
    fn poly_text_out_a(
        self,
        record: EMR_POLYTEXTOUTA,
    ) -> Result<Self, PlayError>;
    fn poly_text_out_w(
        self,
        record: EMR_POLYTEXTOUTW,
    ) -> Result<Self, PlayError>;
    fn polygon(self, record: EMR_POLYGON) -> Result<Self, PlayError>;
    fn polygon_16(self, record: EMR_POLYGON16) -> Result<Self, PlayError>;
    fn polyline(self, record: EMR_POLYLINE) -> Result<Self, PlayError>;
    fn polyline_16(self, record: EMR_POLYLINE16) -> Result<Self, PlayError>;
    fn polyline_to(self, record: EMR_POLYLINETO) -> Result<Self, PlayError>;
    fn polyline_to_16(
        self,
        record: EMR_POLYLINETO16,
    ) -> Result<Self, PlayError>;
    fn rectangle(self, record: EMR_RECTANGLE) -> Result<Self, PlayError>;
    fn round_rect(self, record: EMR_ROUNDRECT) -> Result<Self, PlayError>;
    fn set_pixel_v(self, record: EMR_SETPIXELV) -> Result<Self, PlayError>;
    fn small_text_out(
        self,
        record: EMR_SMALLTEXTOUT,
    ) -> Result<Self, PlayError>;
    fn stroke_and_fill_path(
        self,
        record: EMR_STROKEANDFILLPATH,
    ) -> Result<Self, PlayError>;
    fn stroke_path(self, record: EMR_STROKEPATH) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .
    fn draw_escape(self, record: EMR_DRAWESCAPE) -> Result<Self, PlayError>;
    fn ext_escape(self, record: EMR_EXTESCAPE) -> Result<Self, PlayError>;
    fn named_escape(self, record: EMR_NAMEDESCAPE) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Object Creation Record
    // .
    // .
    fn create_brush_indirect(
        self,
        record: EMR_CREATEBRUSHINDIRECT,
    ) -> Result<Self, PlayError>;
    fn create_color_space(
        self,
        record: EMR_CREATECOLORSPACE,
    ) -> Result<Self, PlayError>;
    fn create_color_space_w(
        self,
        record: EMR_CREATECOLORSPACEW,
    ) -> Result<Self, PlayError>;
    fn create_dib_pattern_brush_pt(
        self,
        record: EMR_CREATEDIBPATTERNBRUSHPT,
    ) -> Result<Self, PlayError>;
    fn create_mono_brush(
        self,
        record: EMR_CREATEMONOBRUSH,
    ) -> Result<Self, PlayError>;
    fn create_palette(
        self,
        record: EMR_CREATEPALETTE,
    ) -> Result<Self, PlayError>;
    fn create_pen(self, record: EMR_CREATEPEN) -> Result<Self, PlayError>;
    fn ext_create_font_indirect_w(
        self,
        record: EMR_EXTCREATEFONTINDIRECTW,
    ) -> Result<Self, PlayError>;
    fn ext_create_pen(
        self,
        record: EMR_EXTCREATEPEN,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Object Manipulation Record
    // .
    // .
    fn color_correct_palette(
        self,
        record: EMR_COLORCORRECTPALETTE,
    ) -> Result<Self, PlayError>;
    fn delete_color_space(
        self,
        record: EMR_DELETECOLORSPACE,
    ) -> Result<Self, PlayError>;
    fn delete_object(self, record: EMR_DELETEOBJECT)
        -> Result<Self, PlayError>;
    fn resize_palette(
        self,
        record: EMR_RESIZEPALETTE,
    ) -> Result<Self, PlayError>;
    fn select_object(self, record: EMR_SELECTOBJECT)
        -> Result<Self, PlayError>;
    fn select_palette(
        self,
        record: EMR_SELECTPALETTE,
    ) -> Result<Self, PlayError>;
    fn set_color_space(
        self,
        record: EMR_SETCOLORSPACE,
    ) -> Result<Self, PlayError>;
    fn set_palette_entries(
        self,
        record: EMR_SETPALETTEENTRIES,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle OpenGL Record
    // .
    // .
    fn gls_bounded_record(
        self,
        record: EMR_GLSBOUNDEDRECORD,
    ) -> Result<Self, PlayError>;
    fn gls_record(self, record: EMR_GLSRECORD) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Path Bracket Record
    // .
    // .
    fn abort_path(self, record: EMR_ABORTPATH) -> Result<Self, PlayError>;
    fn begin_path(self, record: EMR_BEGINPATH) -> Result<Self, PlayError>;
    fn close_figure(self, record: EMR_CLOSEFIGURE) -> Result<Self, PlayError>;
    fn end_path(self, record: EMR_ENDPATH) -> Result<Self, PlayError>;
    fn flatten_path(self, record: EMR_FLATTENPATH) -> Result<Self, PlayError>;
    fn widen_path(self, record: EMR_WIDENPATH) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    fn color_match_to_target_w(
        self,
        record: EMR_COLORMATCHTOTARGETW,
    ) -> Result<Self, PlayError>;
    fn force_ufi_mapping(
        self,
        record: EMR_FORCEUFIMAPPING,
    ) -> Result<Self, PlayError>;
    fn invert_rgn(self, record: EMR_INVERTRGN) -> Result<Self, PlayError>;
    fn move_to_ex(self, record: EMR_MOVETOEX) -> Result<Self, PlayError>;
    fn pixel_format(self, record: EMR_PIXELFORMAT) -> Result<Self, PlayError>;
    fn realize_palette(
        self,
        record: EMR_REALIZEPALETTE,
    ) -> Result<Self, PlayError>;
    fn restore_dc(self, record: EMR_RESTOREDC) -> Result<Self, PlayError>;
    fn save_dc(self, record: EMR_SAVEDC) -> Result<Self, PlayError>;
    fn scale_viewport_ext_ex(
        self,
        record: EMR_SCALEVIEWPORTEXTEX,
    ) -> Result<Self, PlayError>;
    fn scale_window_ext_ex(
        self,
        record: EMR_SCALEWINDOWEXTEX,
    ) -> Result<Self, PlayError>;
    fn set_arc_direction(
        self,
        record: EMR_SETARCDIRECTION,
    ) -> Result<Self, PlayError>;
    fn set_bk_color(self, record: EMR_SETBKCOLOR) -> Result<Self, PlayError>;
    fn set_bk_mode(self, record: EMR_SETBKMODE) -> Result<Self, PlayError>;
    fn set_brush_org_ex(
        self,
        record: EMR_SETBRUSHORGEX,
    ) -> Result<Self, PlayError>;
    fn set_color_adjustment(
        self,
        record: EMR_SETCOLORADJUSTMENT,
    ) -> Result<Self, PlayError>;
    fn set_icm_mode(self, record: EMR_SETICMMODE) -> Result<Self, PlayError>;
    fn set_icm_profile_a(
        self,
        record: EMR_SETICMPROFILEA,
    ) -> Result<Self, PlayError>;
    fn set_icm_profile_w(
        self,
        record: EMR_SETICMPROFILEW,
    ) -> Result<Self, PlayError>;
    fn set_layout(self, record: EMR_SETLAYOUT) -> Result<Self, PlayError>;
    fn set_linked_ufis(
        self,
        record: EMR_SETLINKEDUFIS,
    ) -> Result<Self, PlayError>;
    fn set_map_mode(self, record: EMR_SETMAPMODE) -> Result<Self, PlayError>;
    fn set_mapper_flags(
        self,
        record: EMR_SETMAPPERFLAGS,
    ) -> Result<Self, PlayError>;
    fn set_miter_limit(
        self,
        record: EMR_SETMITERLIMIT,
    ) -> Result<Self, PlayError>;
    fn set_polyfill_mode(
        self,
        record: EMR_SETPOLYFILLMODE,
    ) -> Result<Self, PlayError>;
    fn set_rop2(self, record: EMR_SETROP2) -> Result<Self, PlayError>;
    fn set_stretch_blt_mode(
        self,
        record: EMR_SETSTRETCHBLTMODE,
    ) -> Result<Self, PlayError>;
    fn set_text_align(
        self,
        record: EMR_SETTEXTALIGN,
    ) -> Result<Self, PlayError>;
    fn set_text_color(
        self,
        record: EMR_SETTEXTCOLOR,
    ) -> Result<Self, PlayError>;
    fn set_text_justification(
        self,
        record: EMR_SETTEXTJUSTIFICATION,
    ) -> Result<Self, PlayError>;
    fn set_viewport_ext_ex(
        self,
        record: EMR_SETVIEWPORTEXTEX,
    ) -> Result<Self, PlayError>;
    fn set_viewport_org_ex(
        self,
        record: EMR_SETVIEWPORTORGEX,
    ) -> Result<Self, PlayError>;
    fn set_window_ext_ex(
        self,
        record: EMR_SETWINDOWEXTEX,
    ) -> Result<Self, PlayError>;
    fn set_window_org_ex(
        self,
        record: EMR_SETWINDOWORGEX,
    ) -> Result<Self, PlayError>;

    // .
    // .
    // Functions to handle Transform Record
    // .
    // .
    fn modify_world_transform(
        self,
        record: EMR_MODIFYWORLDTRANSFORM,
    ) -> Result<Self, PlayError>;
    fn set_world_transform(
        self,
        record: EMR_SETWORLDTRANSFORM,
    ) -> Result<Self, PlayError>;
}
