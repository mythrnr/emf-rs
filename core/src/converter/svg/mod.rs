use crate::{converter::PlayError, parser::*};

pub struct SVGPlayer<W> {
    output: W,
}

impl<W> SVGPlayer<W> {
    pub fn new(output: W) -> Self {
        Self { output }
    }
}

impl<W> crate::converter::Player for SVGPlayer<W>
where
    W: std::io::Write,
{
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    )]
    fn generate(self) -> Result<(), PlayError> {
        tracing::info!("generate: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn alpha_blend(
        &mut self,
        _record: EMR_ALPHABLEND,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_ALPHABLEND: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn bit_blt(&mut self, _record: EMR_BITBLT) -> Result<(), PlayError> {
        tracing::info!("EMR_BITBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn mask_blt(&mut self, _record: EMR_MASKBLT) -> Result<(), PlayError> {
        tracing::info!("EMR_MASKBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn plg_blt(&mut self, _record: EMR_PLGBLT) -> Result<(), PlayError> {
        tracing::info!("EMR_PLGBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_dibits_to_device(
        &mut self,
        _record: EMR_SETDIBITSTODEVICE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETDIBITSTODEVICE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn stretch_blt(
        &mut self,
        _record: EMR_STRETCHBLT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_STRETCHBLT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn stretch_dibits(
        &mut self,
        _record: EMR_STRETCHDIBITS,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_STRETCHDIBITS: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn transparent_blt(
        &mut self,
        _record: EMR_TRANSPARENTBLT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_TRANSPARENTBLT: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Clipping Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn exclude_clip_rect(
        &mut self,
        _record: EMR_EXCLUDECLIPRECT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_EXCLUDECLIPRECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_select_clip_rgn(
        &mut self,
        _record: EMR_EXTSELECTCLIPRGN,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_EXTSELECTCLIPRGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn intersect_clip_rect(
        &mut self,
        _record: EMR_INTERSECTCLIPRECT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_INTERSECTCLIPRECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn offset_clip_rgn(
        &mut self,
        _record: EMR_OFFSETCLIPRGN,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_OFFSETCLIPRGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn select_clip_path(
        &mut self,
        _record: EMR_SELECTCLIPPATH,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SELECTCLIPPATH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_meta_rgn(
        &mut self,
        _record: EMR_SETMETARGN,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETMETARGN: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Comment Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn comment(&mut self, _record: EMR_COMMENT) -> Result<(), PlayError> {
        tracing::info!("EMR_COMMENT: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn eof(&mut self, _record: EMR_EOF) -> Result<(), PlayError> {
        tracing::info!("EMR_EOF: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn header(&mut self, _record: EMR_HEADER) -> Result<(), PlayError> {
        tracing::info!("EMR_HEADER: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn angle_arc(&mut self, _record: EMR_ANGLEARC) -> Result<(), PlayError> {
        tracing::info!("EMR_ANGLEARC: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn arc(&mut self, _record: EMR_ARC) -> Result<(), PlayError> {
        tracing::info!("EMR_ARC: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn arc_to(&mut self, _record: EMR_ARCTO) -> Result<(), PlayError> {
        tracing::info!("EMR_ARCTO: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn chord(&mut self, _record: EMR_CHORD) -> Result<(), PlayError> {
        tracing::info!("EMR_CHORD: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ellipse(&mut self, _record: EMR_ELLIPSE) -> Result<(), PlayError> {
        tracing::info!("EMR_ELLIPSE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_flood_fill(
        &mut self,
        _record: EMR_EXTFLOODFILL,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_EXTFLOODFILL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_text_out_a(
        &mut self,
        _record: EMR_EXTTEXTOUTA,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_EXTTEXTOUTA: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_text_out_w(
        &mut self,
        _record: EMR_EXTTEXTOUTW,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_EXTTEXTOUTW: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn fill_path(&mut self, _record: EMR_FILLPATH) -> Result<(), PlayError> {
        tracing::info!("EMR_FILLPATH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn fill_rgn(&mut self, _record: EMR_FILLRGN) -> Result<(), PlayError> {
        tracing::info!("EMR_FILLRGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn frame_rgn(&mut self, _record: EMR_FRAMERGN) -> Result<(), PlayError> {
        tracing::info!("EMR_FRAMERGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn gradient_fill(
        &mut self,
        _record: EMR_GRADIENTFILL,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_GRADIENTFILL: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn line_to(&mut self, _record: EMR_LINETO) -> Result<(), PlayError> {
        tracing::info!("EMR_LINETO: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn paint_rgn(&mut self, _record: EMR_PAINTRGN) -> Result<(), PlayError> {
        tracing::info!("EMR_PAINTRGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn pie(&mut self, _record: EMR_PIE) -> Result<(), PlayError> {
        tracing::info!("EMR_PIE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_bezier(
        &mut self,
        _record: EMR_POLYBEZIER,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYBEZIER: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_bezier_16(
        &mut self,
        _record: EMR_POLYBEZIER16,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYBEZIER16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_bezier_to(
        &mut self,
        _record: EMR_POLYBEZIERTO,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYBEZIERTO: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_bezier_to_16(
        &mut self,
        _record: EMR_POLYBEZIERTO16,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYBEZIERTO16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_draw(&mut self, _record: EMR_POLYDRAW) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYDRAW: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_draw_16(
        &mut self,
        _record: EMR_POLYDRAW16,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYDRAW16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_polygon(
        &mut self,
        _record: EMR_POLYPOLYGON,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYPOLYGON: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_polygon_16(
        &mut self,
        _record: EMR_POLYPOLYGON16,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYPOLYGON16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_polyline(
        &mut self,
        _record: EMR_POLYPOLYLINE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYPOLYLINE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_polyline_16(
        &mut self,
        _record: EMR_POLYPOLYLINE16,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYPOLYLINE16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_text_out_a(
        &mut self,
        _record: EMR_POLYTEXTOUTA,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYTEXTOUTA: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn poly_text_out_w(
        &mut self,
        _record: EMR_POLYTEXTOUTW,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYTEXTOUTW: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polygon(&mut self, _record: EMR_POLYGON) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYGON: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polygon_16(&mut self, _record: EMR_POLYGON16) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYGON16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polyline(&mut self, _record: EMR_POLYLINE) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYLINE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polyline_16(
        &mut self,
        _record: EMR_POLYLINE16,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYLINE16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polyline_to(
        &mut self,
        _record: EMR_POLYLINETO,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYLINETO: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn polyline_to_16(
        &mut self,
        _record: EMR_POLYLINETO16,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_POLYLINETO16: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn rectangle(&mut self, _record: EMR_RECTANGLE) -> Result<(), PlayError> {
        tracing::info!("EMR_RECTANGLE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn round_rect(&mut self, _record: EMR_ROUNDRECT) -> Result<(), PlayError> {
        tracing::info!("EMR_ROUNDRECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_pixel_v(&mut self, _record: EMR_SETPIXELV) -> Result<(), PlayError> {
        tracing::info!("EMR_SETPIXELV: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn small_text_out(
        &mut self,
        _record: EMR_SMALLTEXTOUT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SMALLTEXTOUT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn stroke_and_fill_path(
        &mut self,
        _record: EMR_STROKEANDFILLPATH,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_STROKEANDFILLPATH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn stroke_path(
        &mut self,
        _record: EMR_STROKEPATH,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_STROKEPATH: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn draw_escape(
        &mut self,
        _record: EMR_DRAWESCAPE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_DRAWESCAPE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_escape(&mut self, _record: EMR_EXTESCAPE) -> Result<(), PlayError> {
        tracing::info!("EMR_EXTESCAPE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn named_escape(
        &mut self,
        _record: EMR_NAMEDESCAPE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_NAMEDESCAPE: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Object Creation Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_brush_indirect(
        &mut self,
        _record: EMR_CREATEBRUSHINDIRECT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_CREATEBRUSHINDIRECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_color_space(
        &mut self,
        _record: EMR_CREATECOLORSPACE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_CREATECOLORSPACE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_color_space_w(
        &mut self,
        _record: EMR_CREATECOLORSPACEW,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_CREATECOLORSPACEW: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_dib_pattern_brush_pt(
        &mut self,
        _record: EMR_CREATEDIBPATTERNBRUSHPT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_CREATEDIBPATTERNBRUSHPT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_mono_brush(
        &mut self,
        _record: EMR_CREATEMONOBRUSH,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_CREATEMONOBRUSH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_palette(
        &mut self,
        _record: EMR_CREATEPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_CREATEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn create_pen(&mut self, record: EMR_CREATEPEN) -> Result<(), PlayError> {
        tracing::info!("EMR_CREATEPEN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_create_font_indirect_w(
        &mut self,
        _record: EMR_EXTCREATEFONTINDIRECTW,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_EXTCREATEFONTINDIRECTW: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn ext_create_pen(
        &mut self,
        _record: EMR_EXTCREATEPEN,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_EXTCREATEPEN: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Object Manipulation Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn color_correct_palette(
        &mut self,
        _record: EMR_COLORCORRECTPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_COLORCORRECTPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn delete_color_space(
        &mut self,
        _record: EMR_DELETECOLORSPACE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_DELETECOLORSPACE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn delete_object(
        &mut self,
        _record: EMR_DELETEOBJECT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_DELETEOBJECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn resize_palette(
        &mut self,
        _record: EMR_RESIZEPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_RESIZEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn select_object(
        &mut self,
        _record: EMR_SELECTOBJECT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SELECTOBJECT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn select_palette(
        &mut self,
        _record: EMR_SELECTPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SELECTPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_color_space(
        &mut self,
        _record: EMR_SETCOLORSPACE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETCOLORSPACE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_palette_entries(
        &mut self,
        _record: EMR_SETPALETTEENTRIES,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETPALETTEENTRIES: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle OpenGL Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn gls_bounded_record(
        &mut self,
        _record: EMR_GLSBOUNDEDRECORD,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_GLSBOUNDEDRECORD: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn gls_record(&mut self, _record: EMR_GLSRECORD) -> Result<(), PlayError> {
        tracing::info!("EMR_GLSRECORD: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Path Bracket Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn abort_path(&mut self, _record: EMR_ABORTPATH) -> Result<(), PlayError> {
        tracing::info!("EMR_ABORTPATH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn begin_path(&mut self, _record: EMR_BEGINPATH) -> Result<(), PlayError> {
        tracing::info!("EMR_BEGINPATH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn close_figure(
        &mut self,
        _record: EMR_CLOSEFIGURE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_CLOSEFIGURE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn end_path(&mut self, _record: EMR_ENDPATH) -> Result<(), PlayError> {
        tracing::info!("EMR_ENDPATH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn flatten_path(
        &mut self,
        _record: EMR_FLATTENPATH,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_FLATTENPATH: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn widen_path(&mut self, _record: EMR_WIDENPATH) -> Result<(), PlayError> {
        tracing::info!("EMR_WIDENPATH: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn color_match_to_target_w(
        &mut self,
        _record: EMR_COLORMATCHTOTARGETW,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_COLORMATCHTOTARGETW: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn force_ufi_mapping(
        &mut self,
        _record: EMR_FORCEUFIMAPPING,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_FORCEUFIMAPPING: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn invert_rgn(&mut self, _record: EMR_INVERTRGN) -> Result<(), PlayError> {
        tracing::info!("EMR_INVERTRGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn move_to_ex(&mut self, _record: EMR_MOVETOEX) -> Result<(), PlayError> {
        tracing::info!("EMR_MOVETOEX: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn pixel_format(
        &mut self,
        _record: EMR_PIXELFORMAT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_PIXELFORMAT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn realize_palette(
        &mut self,
        _record: EMR_REALIZEPALETTE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_REALIZEPALETTE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn restore_dc(&mut self, _record: EMR_RESTOREDC) -> Result<(), PlayError> {
        tracing::info!("EMR_RESTOREDC: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn save_dc(&mut self, _record: EMR_SAVEDC) -> Result<(), PlayError> {
        tracing::info!("EMR_SAVEDC: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn scale_viewport_ext_ex(
        &mut self,
        _record: EMR_SCALEVIEWPORTEXTEX,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SCALEVIEWPORTEXTEX: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn scale_window_ext_ex(
        &mut self,
        _record: EMR_SCALEWINDOWEXTEX,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SCALEWINDOWEXTEX: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_arc_direction(
        &mut self,
        _record: EMR_SETARCDIRECTION,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETARCDIRECTION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_bk_color(
        &mut self,
        _record: EMR_SETBKCOLOR,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETBKCOLOR: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_bk_mode(&mut self, _record: EMR_SETBKMODE) -> Result<(), PlayError> {
        tracing::info!("EMR_SETBKMODE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_brush_org_ex(
        &mut self,
        _record: EMR_SETBRUSHORGEX,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETBRUSHORGEX: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_color_adjustment(
        &mut self,
        _record: EMR_SETCOLORADJUSTMENT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETCOLORADJUSTMENT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_icm_mode(
        &mut self,
        _record: EMR_SETICMMODE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETICMMODE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_icm_profile_a(
        &mut self,
        _record: EMR_SETICMPROFILEA,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETICMPROFILEA: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_icm_profile_w(
        &mut self,
        _record: EMR_SETICMPROFILEW,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETICMPROFILEW: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_layout(&mut self, _record: EMR_SETLAYOUT) -> Result<(), PlayError> {
        tracing::info!("EMR_SETLAYOUT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_linked_ufis(
        &mut self,
        _record: EMR_SETLINKEDUFIS,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETLINKEDUFIS: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_map_mode(
        &mut self,
        _record: EMR_SETMAPMODE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETMAPMODE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_mapper_flags(
        &mut self,
        _record: EMR_SETMAPPERFLAGS,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETMAPPERFLAGS: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_miter_limit(
        &mut self,
        _record: EMR_SETMITERLIMIT,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETMITERLIMIT: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_polyfill_mode(
        &mut self,
        _record: EMR_SETPOLYFILLMODE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETPOLYFILLMODE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_rop2(&mut self, _record: EMR_SETROP2) -> Result<(), PlayError> {
        tracing::info!("EMR_SETROP2: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_stretch_blt_mode(
        &mut self,
        _record: EMR_SETSTRETCHBLTMODE,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETSTRETCHBLTMODE: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_text_align(
        &mut self,
        _record: EMR_SETTEXTALIGN,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETTEXTALIGN: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_text_color(
        &mut self,
        _record: EMR_SETTEXTCOLOR,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETTEXTCOLOR: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_text_justification(
        &mut self,
        _record: EMR_SETTEXTJUSTIFICATION,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETTEXTJUSTIFICATION: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_viewport_ext_ex(
        &mut self,
        _record: EMR_SETVIEWPORTEXTEX,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETVIEWPORTEXTEX: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_viewport_org_ex(
        &mut self,
        _record: EMR_SETVIEWPORTORGEX,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETVIEWPORTORGEX: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_window_ext_ex(
        &mut self,
        _record: EMR_SETWINDOWEXTEX,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETWINDOWEXTEX: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_window_org_ex(
        &mut self,
        _record: EMR_SETWINDOWORGEX,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETWINDOWORGEX: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Transform Record
    // .
    // .
    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn modify_world_transform(
        &mut self,
        _record: EMR_MODIFYWORLDTRANSFORM,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_MODIFYWORLDTRANSFORM: not implemented");
        Ok(())
    }

    #[tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    )]
    fn set_world_transform(
        &mut self,
        _record: EMR_SETWORLDTRANSFORM,
    ) -> Result<(), PlayError> {
        tracing::info!("EMR_SETWORLDTRANSFORM: not implemented");
        Ok(())
    }
}
