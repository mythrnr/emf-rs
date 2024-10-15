use crate::parser::*;

#[derive(Clone, Debug, thiserror::Error)]
pub enum PlayError {
    #[error("failed to generate: {cause}")]
    FailedGenerate { cause: String },
    #[error("invalid brush: {cause}")]
    InvalidBrush { cause: String },
    #[error("invalid record: {cause}")]
    InvalidRecord { cause: String },
    #[error("unexpected graphics object: {cause}")]
    UnexpectedGraphicsObject { cause: String },
    #[error("unknown: {cause}")]
    Unknown { cause: String },
}

pub trait Player {
    /// Call after converting to write output.
    fn generate(self) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    fn alpha_blend(&mut self, record: EMR_ALPHABLEND) -> Result<(), PlayError>;
    fn bit_blt(&mut self, record: EMR_BITBLT) -> Result<(), PlayError>;
    fn mask_blt(&mut self, record: EMR_MASKBLT) -> Result<(), PlayError>;
    fn plg_blt(&mut self, record: EMR_PLGBLT) -> Result<(), PlayError>;
    fn set_dibits_to_device(
        &mut self,
        record: EMR_SETDIBITSTODEVICE,
    ) -> Result<(), PlayError>;
    fn stretch_blt(&mut self, record: EMR_STRETCHBLT) -> Result<(), PlayError>;
    fn stretch_dibits(
        &mut self,
        record: EMR_STRETCHDIBITS,
    ) -> Result<(), PlayError>;
    fn transparent_blt(
        &mut self,
        record: EMR_TRANSPARENTBLT,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Clipping Record
    // .
    // .
    fn exclude_clip_rect(
        &mut self,
        record: EMR_EXCLUDECLIPRECT,
    ) -> Result<(), PlayError>;
    fn ext_select_clip_rgn(
        &mut self,
        record: EMR_EXTSELECTCLIPRGN,
    ) -> Result<(), PlayError>;
    fn intersect_clip_rect(
        &mut self,
        record: EMR_INTERSECTCLIPRECT,
    ) -> Result<(), PlayError>;
    fn offset_clip_rgn(
        &mut self,
        record: EMR_OFFSETCLIPRGN,
    ) -> Result<(), PlayError>;
    fn select_clip_path(
        &mut self,
        record: EMR_SELECTCLIPPATH,
    ) -> Result<(), PlayError>;
    fn set_meta_rgn(&mut self, record: EMR_SETMETARGN)
        -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Comment Record
    // .
    // .
    fn comment(&mut self, record: EMR_COMMENT) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    fn eof(&mut self, record: EMR_EOF) -> Result<(), PlayError>;
    fn header(&mut self, header: EMR_HEADER) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .
    fn angle_arc(&mut self, record: EMR_ANGLEARC) -> Result<(), PlayError>;
    fn arc(&mut self, record: EMR_ARC) -> Result<(), PlayError>;
    fn arc_to(&mut self, record: EMR_ARCTO) -> Result<(), PlayError>;
    fn chord(&mut self, record: EMR_CHORD) -> Result<(), PlayError>;
    fn ellipse(&mut self, record: EMR_ELLIPSE) -> Result<(), PlayError>;
    fn ext_flood_fill(
        &mut self,
        record: EMR_EXTFLOODFILL,
    ) -> Result<(), PlayError>;
    fn ext_text_out_a(
        &mut self,
        record: EMR_EXTTEXTOUTA,
    ) -> Result<(), PlayError>;
    fn ext_text_out_w(
        &mut self,
        record: EMR_EXTTEXTOUTW,
    ) -> Result<(), PlayError>;
    fn fill_path(&mut self, record: EMR_FILLPATH) -> Result<(), PlayError>;
    fn fill_rgn(&mut self, record: EMR_FILLRGN) -> Result<(), PlayError>;
    fn frame_rgn(&mut self, record: EMR_FRAMERGN) -> Result<(), PlayError>;
    fn gradient_fill(
        &mut self,
        record: EMR_GRADIENTFILL,
    ) -> Result<(), PlayError>;
    fn line_to(&mut self, record: EMR_LINETO) -> Result<(), PlayError>;
    fn paint_rgn(&mut self, record: EMR_PAINTRGN) -> Result<(), PlayError>;
    fn pie(&mut self, record: EMR_PIE) -> Result<(), PlayError>;
    fn poly_bezier(&mut self, record: EMR_POLYBEZIER) -> Result<(), PlayError>;
    fn poly_bezier_16(
        &mut self,
        record: EMR_POLYBEZIER16,
    ) -> Result<(), PlayError>;
    fn poly_bezier_to(
        &mut self,
        record: EMR_POLYBEZIERTO,
    ) -> Result<(), PlayError>;
    fn poly_bezier_to_16(
        &mut self,
        record: EMR_POLYBEZIERTO16,
    ) -> Result<(), PlayError>;
    fn poly_draw(&mut self, record: EMR_POLYDRAW) -> Result<(), PlayError>;
    fn poly_draw_16(&mut self, record: EMR_POLYDRAW16)
        -> Result<(), PlayError>;
    fn poly_polygon(
        &mut self,
        record: EMR_POLYPOLYGON,
    ) -> Result<(), PlayError>;
    fn poly_polygon_16(
        &mut self,
        record: EMR_POLYPOLYGON16,
    ) -> Result<(), PlayError>;
    fn poly_polyline(
        &mut self,
        record: EMR_POLYPOLYLINE,
    ) -> Result<(), PlayError>;
    fn poly_polyline_16(
        &mut self,
        record: EMR_POLYPOLYLINE16,
    ) -> Result<(), PlayError>;
    fn poly_text_out_a(
        &mut self,
        record: EMR_POLYTEXTOUTA,
    ) -> Result<(), PlayError>;
    fn poly_text_out_w(
        &mut self,
        record: EMR_POLYTEXTOUTW,
    ) -> Result<(), PlayError>;
    fn polygon(&mut self, record: EMR_POLYGON) -> Result<(), PlayError>;
    fn polygon_16(&mut self, record: EMR_POLYGON16) -> Result<(), PlayError>;
    fn polyline(&mut self, record: EMR_POLYLINE) -> Result<(), PlayError>;
    fn polyline_16(&mut self, record: EMR_POLYLINE16) -> Result<(), PlayError>;
    fn polyline_to(&mut self, record: EMR_POLYLINETO) -> Result<(), PlayError>;
    fn polyline_to_16(
        &mut self,
        record: EMR_POLYLINETO16,
    ) -> Result<(), PlayError>;
    fn rectangle(&mut self, record: EMR_RECTANGLE) -> Result<(), PlayError>;
    fn round_rect(&mut self, record: EMR_ROUNDRECT) -> Result<(), PlayError>;
    fn set_pixel_v(&mut self, record: EMR_SETPIXELV) -> Result<(), PlayError>;
    fn small_text_out(
        &mut self,
        record: EMR_SMALLTEXTOUT,
    ) -> Result<(), PlayError>;
    fn stroke_and_fill_path(
        &mut self,
        record: EMR_STROKEANDFILLPATH,
    ) -> Result<(), PlayError>;
    fn stroke_path(&mut self, record: EMR_STROKEPATH) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .
    fn draw_escape(&mut self, record: EMR_DRAWESCAPE) -> Result<(), PlayError>;
    fn ext_escape(&mut self, record: EMR_EXTESCAPE) -> Result<(), PlayError>;
    fn named_escape(
        &mut self,
        record: EMR_NAMEDESCAPE,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Object Creation Record
    // .
    // .
    fn create_brush_indirect(
        &mut self,
        record: EMR_CREATEBRUSHINDIRECT,
    ) -> Result<(), PlayError>;
    fn create_color_space(
        &mut self,
        record: EMR_CREATECOLORSPACE,
    ) -> Result<(), PlayError>;
    fn create_color_space_w(
        &mut self,
        record: EMR_CREATECOLORSPACEW,
    ) -> Result<(), PlayError>;
    fn create_dib_pattern_brush_pt(
        &mut self,
        record: EMR_CREATEDIBPATTERNBRUSHPT,
    ) -> Result<(), PlayError>;
    fn create_mono_brush(
        &mut self,
        record: EMR_CREATEMONOBRUSH,
    ) -> Result<(), PlayError>;
    fn create_palette(
        &mut self,
        record: EMR_CREATEPALETTE,
    ) -> Result<(), PlayError>;
    fn create_pen(&mut self, record: EMR_CREATEPEN) -> Result<(), PlayError>;
    fn ext_create_font_indirect_w(
        &mut self,
        record: EMR_EXTCREATEFONTINDIRECTW,
    ) -> Result<(), PlayError>;
    fn ext_create_pen(
        &mut self,
        record: EMR_EXTCREATEPEN,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Object Manipulation Record
    // .
    // .
    fn color_correct_palette(
        &mut self,
        record: EMR_COLORCORRECTPALETTE,
    ) -> Result<(), PlayError>;
    fn delete_color_space(
        &mut self,
        record: EMR_DELETECOLORSPACE,
    ) -> Result<(), PlayError>;
    fn delete_object(
        &mut self,
        record: EMR_DELETEOBJECT,
    ) -> Result<(), PlayError>;
    fn resize_palette(
        &mut self,
        record: EMR_RESIZEPALETTE,
    ) -> Result<(), PlayError>;
    fn select_object(
        &mut self,
        record: EMR_SELECTOBJECT,
    ) -> Result<(), PlayError>;
    fn select_palette(
        &mut self,
        record: EMR_SELECTPALETTE,
    ) -> Result<(), PlayError>;
    fn set_color_space(
        &mut self,
        record: EMR_SETCOLORSPACE,
    ) -> Result<(), PlayError>;
    fn set_palette_entries(
        &mut self,
        record: EMR_SETPALETTEENTRIES,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle OpenGL Record
    // .
    // .
    fn gls_bounded_record(
        &mut self,
        record: EMR_GLSBOUNDEDRECORD,
    ) -> Result<(), PlayError>;
    fn gls_record(&mut self, record: EMR_GLSRECORD) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Path Bracket Record
    // .
    // .
    fn abort_path(&mut self, record: EMR_ABORTPATH) -> Result<(), PlayError>;
    fn begin_path(&mut self, record: EMR_BEGINPATH) -> Result<(), PlayError>;
    fn close_figure(
        &mut self,
        record: EMR_CLOSEFIGURE,
    ) -> Result<(), PlayError>;
    fn end_path(&mut self, record: EMR_ENDPATH) -> Result<(), PlayError>;
    fn flatten_path(
        &mut self,
        record: EMR_FLATTENPATH,
    ) -> Result<(), PlayError>;
    fn widen_path(&mut self, record: EMR_WIDENPATH) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    fn color_match_to_target_w(
        &mut self,
        record: EMR_COLORMATCHTOTARGETW,
    ) -> Result<(), PlayError>;
    fn force_ufi_mapping(
        &mut self,
        record: EMR_FORCEUFIMAPPING,
    ) -> Result<(), PlayError>;
    fn invert_rgn(&mut self, record: EMR_INVERTRGN) -> Result<(), PlayError>;
    fn move_to_ex(&mut self, record: EMR_MOVETOEX) -> Result<(), PlayError>;
    fn pixel_format(
        &mut self,
        record: EMR_PIXELFORMAT,
    ) -> Result<(), PlayError>;
    fn realize_palette(
        &mut self,
        record: EMR_REALIZEPALETTE,
    ) -> Result<(), PlayError>;
    fn restore_dc(&mut self, record: EMR_RESTOREDC) -> Result<(), PlayError>;
    fn save_dc(&mut self, record: EMR_SAVEDC) -> Result<(), PlayError>;
    fn scale_viewport_ext_ex(
        &mut self,
        record: EMR_SCALEVIEWPORTEXTEX,
    ) -> Result<(), PlayError>;
    fn scale_window_ext_ex(
        &mut self,
        record: EMR_SCALEWINDOWEXTEX,
    ) -> Result<(), PlayError>;
    fn set_arc_direction(
        &mut self,
        record: EMR_SETARCDIRECTION,
    ) -> Result<(), PlayError>;
    fn set_bk_color(&mut self, record: EMR_SETBKCOLOR)
        -> Result<(), PlayError>;
    fn set_bk_mode(&mut self, record: EMR_SETBKMODE) -> Result<(), PlayError>;
    fn set_brush_org_ex(
        &mut self,
        record: EMR_SETBRUSHORGEX,
    ) -> Result<(), PlayError>;
    fn set_color_adjustment(
        &mut self,
        record: EMR_SETCOLORADJUSTMENT,
    ) -> Result<(), PlayError>;
    fn set_icm_mode(&mut self, record: EMR_SETICMMODE)
        -> Result<(), PlayError>;
    fn set_icm_profile_a(
        &mut self,
        record: EMR_SETICMPROFILEA,
    ) -> Result<(), PlayError>;
    fn set_icm_profile_w(
        &mut self,
        record: EMR_SETICMPROFILEW,
    ) -> Result<(), PlayError>;
    fn set_layout(&mut self, record: EMR_SETLAYOUT) -> Result<(), PlayError>;
    fn set_linked_ufis(
        &mut self,
        record: EMR_SETLINKEDUFIS,
    ) -> Result<(), PlayError>;
    fn set_map_mode(&mut self, record: EMR_SETMAPMODE)
        -> Result<(), PlayError>;
    fn set_mapper_flags(
        &mut self,
        record: EMR_SETMAPPERFLAGS,
    ) -> Result<(), PlayError>;
    fn set_miter_limit(
        &mut self,
        record: EMR_SETMITERLIMIT,
    ) -> Result<(), PlayError>;
    fn set_polyfill_mode(
        &mut self,
        record: EMR_SETPOLYFILLMODE,
    ) -> Result<(), PlayError>;
    fn set_rop2(&mut self, record: EMR_SETROP2) -> Result<(), PlayError>;
    fn set_stretch_blt_mode(
        &mut self,
        record: EMR_SETSTRETCHBLTMODE,
    ) -> Result<(), PlayError>;
    fn set_text_align(
        &mut self,
        record: EMR_SETTEXTALIGN,
    ) -> Result<(), PlayError>;
    fn set_text_color(
        &mut self,
        record: EMR_SETTEXTCOLOR,
    ) -> Result<(), PlayError>;
    fn set_text_justification(
        &mut self,
        record: EMR_SETTEXTJUSTIFICATION,
    ) -> Result<(), PlayError>;
    fn set_viewport_ext_ex(
        &mut self,
        record: EMR_SETVIEWPORTEXTEX,
    ) -> Result<(), PlayError>;
    fn set_viewport_org_ex(
        &mut self,
        record: EMR_SETVIEWPORTORGEX,
    ) -> Result<(), PlayError>;
    fn set_window_ext_ex(
        &mut self,
        record: EMR_SETWINDOWEXTEX,
    ) -> Result<(), PlayError>;
    fn set_window_org_ex(
        &mut self,
        record: EMR_SETWINDOWORGEX,
    ) -> Result<(), PlayError>;

    // .
    // .
    // Functions to handle Transform Record
    // .
    // .
    fn modify_world_transform(
        &mut self,
        record: EMR_MODIFYWORLDTRANSFORM,
    ) -> Result<(), PlayError>;
    fn set_world_transform(
        &mut self,
        record: EMR_SETWORLDTRANSFORM,
    ) -> Result<(), PlayError>;
}
