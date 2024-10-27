mod node;

use wmf_core::parser::{PointL, SizeL};

use crate::{
    converter::{
        playback_device_context::{
            EmfObjectTable, GraphicsEnvironment, GraphicsObject,
            PlaybackDeviceContext, PlaybackStateColors, PlaybackStateDrawing,
            PlaybackStateRegions, PlaybackStateText, SelectedObject, Viewport,
            Window,
        },
        svg::node::{Data, Node},
        PlayError,
    },
    imports::*,
    parser::*,
};

pub struct SVGPlayer {
    context_stack: Vec<PlaybackDeviceContext>,
    context: PlaybackDeviceContext,
    definitions: Vec<Node>,
    elements: Vec<Node>,
    emf_object_table: EmfObjectTable,
    selected_emf_object: SelectedObject,
}

impl Default for SVGPlayer {
    fn default() -> Self {
        Self {
            context_stack: vec![],
            context: PlaybackDeviceContext::default(),
            definitions: vec![],
            elements: vec![],
            emf_object_table: EmfObjectTable::new(0),
            selected_emf_object: SelectedObject::default(),
        }
    }
}

impl SVGPlayer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl crate::converter::Player for SVGPlayer {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        let Self { definitions, elements, .. } = self;

        let mut document =
            Node::node("svg").set("xmlns", "http://www.w3.org/2000/svg");

        if !definitions.is_empty() {
            let mut defs = Node::node("defs");
            for v in definitions {
                defs = defs.add(v);
            }

            document = document.add(defs);
        }

        for v in elements {
            document = document.add(v);
        }

        Ok(document.to_string().into_bytes())
    }

    // .
    // .
    // Functions to handle Bitmap Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn alpha_blend(
        &mut self,
        _record: EMR_ALPHABLEND,
    ) -> Result<(), PlayError> {
        info!("EMR_ALPHABLEND: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn bit_blt(&mut self, _record: EMR_BITBLT) -> Result<(), PlayError> {
        info!("EMR_BITBLT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn mask_blt(&mut self, _record: EMR_MASKBLT) -> Result<(), PlayError> {
        info!("EMR_MASKBLT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn plg_blt(&mut self, _record: EMR_PLGBLT) -> Result<(), PlayError> {
        info!("EMR_PLGBLT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_dibits_to_device(
        &mut self,
        _record: EMR_SETDIBITSTODEVICE,
    ) -> Result<(), PlayError> {
        info!("EMR_SETDIBITSTODEVICE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stretch_blt(
        &mut self,
        _record: EMR_STRETCHBLT,
    ) -> Result<(), PlayError> {
        info!("EMR_STRETCHBLT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stretch_dibits(
        &mut self,
        _record: EMR_STRETCHDIBITS,
    ) -> Result<(), PlayError> {
        info!("EMR_STRETCHDIBITS: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn transparent_blt(
        &mut self,
        _record: EMR_TRANSPARENTBLT,
    ) -> Result<(), PlayError> {
        info!("EMR_TRANSPARENTBLT: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Clipping Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn exclude_clip_rect(
        &mut self,
        _record: EMR_EXCLUDECLIPRECT,
    ) -> Result<(), PlayError> {
        info!("EMR_EXCLUDECLIPRECT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_select_clip_rgn(
        &mut self,
        _record: EMR_EXTSELECTCLIPRGN,
    ) -> Result<(), PlayError> {
        info!("EMR_EXTSELECTCLIPRGN: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn intersect_clip_rect(
        &mut self,
        _record: EMR_INTERSECTCLIPRECT,
    ) -> Result<(), PlayError> {
        info!("EMR_INTERSECTCLIPRECT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn offset_clip_rgn(
        &mut self,
        _record: EMR_OFFSETCLIPRGN,
    ) -> Result<(), PlayError> {
        info!("EMR_OFFSETCLIPRGN: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_clip_path(
        &mut self,
        _record: EMR_SELECTCLIPPATH,
    ) -> Result<(), PlayError> {
        info!("EMR_SELECTCLIPPATH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_meta_rgn(
        &mut self,
        _record: EMR_SETMETARGN,
    ) -> Result<(), PlayError> {
        info!("EMR_SETMETARGN: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Comment Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn comment(&mut self, _record: EMR_COMMENT) -> Result<(), PlayError> {
        Ok(())
    }

    // .
    // .
    // Functions to handle Control Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn eof(&mut self, _record: EMR_EOF) -> Result<(), PlayError> {
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn header(&mut self, record: EMR_HEADER) -> Result<(), PlayError> {
        self.emf_object_table =
            EmfObjectTable::new(record.emf_header.handles as usize);

        let (extent, origin) = (
            SizeL {
                cx: (record.emf_header.bounds.right
                    - record.emf_header.bounds.left) as u32,
                cy: (record.emf_header.bounds.bottom
                    - record.emf_header.bounds.top) as u32,
            },
            PointL {
                x: record.emf_header.bounds.left,
                y: record.emf_header.bounds.top,
            },
        );

        self.context.graphics_environment = GraphicsEnvironment {
            regions: PlaybackStateRegions {
                clipping: None,
                meta_clipping: None,
                viewport: Viewport {
                    extent: extent.clone(),
                    origin: origin.clone(),
                },
                window: Window { extent, origin },
            },
            color: PlaybackStateColors::default(),
            text: PlaybackStateText::default(),
            drawing: PlaybackStateDrawing::default(),
        };

        Ok(())
    }

    // .
    // .
    // Functions to handle Drawing Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn angle_arc(&mut self, _record: EMR_ANGLEARC) -> Result<(), PlayError> {
        info!("EMR_ANGLEARC: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn arc(&mut self, _record: EMR_ARC) -> Result<(), PlayError> {
        info!("EMR_ARC: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn arc_to(&mut self, _record: EMR_ARCTO) -> Result<(), PlayError> {
        info!("EMR_ARCTO: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn chord(&mut self, _record: EMR_CHORD) -> Result<(), PlayError> {
        info!("EMR_CHORD: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ellipse(&mut self, _record: EMR_ELLIPSE) -> Result<(), PlayError> {
        info!("EMR_ELLIPSE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_flood_fill(
        &mut self,
        _record: EMR_EXTFLOODFILL,
    ) -> Result<(), PlayError> {
        info!("EMR_EXTFLOODFILL: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_text_out_a(
        &mut self,
        _record: EMR_EXTTEXTOUTA,
    ) -> Result<(), PlayError> {
        info!("EMR_EXTTEXTOUTA: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_text_out_w(
        &mut self,
        _record: EMR_EXTTEXTOUTW,
    ) -> Result<(), PlayError> {
        info!("EMR_EXTTEXTOUTW: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn fill_path(&mut self, _record: EMR_FILLPATH) -> Result<(), PlayError> {
        info!("EMR_FILLPATH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn fill_rgn(&mut self, _record: EMR_FILLRGN) -> Result<(), PlayError> {
        info!("EMR_FILLRGN: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn frame_rgn(&mut self, _record: EMR_FRAMERGN) -> Result<(), PlayError> {
        info!("EMR_FRAMERGN: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn gradient_fill(
        &mut self,
        _record: EMR_GRADIENTFILL,
    ) -> Result<(), PlayError> {
        info!("EMR_GRADIENTFILL: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn line_to(&mut self, _record: EMR_LINETO) -> Result<(), PlayError> {
        info!("EMR_LINETO: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn paint_rgn(&mut self, _record: EMR_PAINTRGN) -> Result<(), PlayError> {
        info!("EMR_PAINTRGN: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn pie(&mut self, _record: EMR_PIE) -> Result<(), PlayError> {
        info!("EMR_PIE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier(
        &mut self,
        _record: EMR_POLYBEZIER,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYBEZIER: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_16(
        &mut self,
        _record: EMR_POLYBEZIER16,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYBEZIER16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_to(
        &mut self,
        _record: EMR_POLYBEZIERTO,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYBEZIERTO: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_to_16(
        &mut self,
        _record: EMR_POLYBEZIERTO16,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYBEZIERTO16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_draw(&mut self, _record: EMR_POLYDRAW) -> Result<(), PlayError> {
        info!("EMR_POLYDRAW: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_draw_16(
        &mut self,
        _record: EMR_POLYDRAW16,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYDRAW16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polygon(
        &mut self,
        _record: EMR_POLYPOLYGON,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYPOLYGON: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polygon_16(
        &mut self,
        _record: EMR_POLYPOLYGON16,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYPOLYGON16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polyline(
        &mut self,
        _record: EMR_POLYPOLYLINE,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYPOLYLINE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polyline_16(
        &mut self,
        _record: EMR_POLYPOLYLINE16,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYPOLYLINE16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_text_out_a(
        &mut self,
        _record: EMR_POLYTEXTOUTA,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYTEXTOUTA: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_text_out_w(
        &mut self,
        _record: EMR_POLYTEXTOUTW,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYTEXTOUTW: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polygon(&mut self, _record: EMR_POLYGON) -> Result<(), PlayError> {
        info!("EMR_POLYGON: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polygon_16(&mut self, _record: EMR_POLYGON16) -> Result<(), PlayError> {
        info!("EMR_POLYGON16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline(&mut self, _record: EMR_POLYLINE) -> Result<(), PlayError> {
        info!("EMR_POLYLINE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_16(
        &mut self,
        _record: EMR_POLYLINE16,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYLINE16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_to(
        &mut self,
        _record: EMR_POLYLINETO,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYLINETO: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_to_16(
        &mut self,
        _record: EMR_POLYLINETO16,
    ) -> Result<(), PlayError> {
        info!("EMR_POLYLINETO16: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn rectangle(&mut self, _record: EMR_RECTANGLE) -> Result<(), PlayError> {
        info!("EMR_RECTANGLE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn round_rect(&mut self, _record: EMR_ROUNDRECT) -> Result<(), PlayError> {
        info!("EMR_ROUNDRECT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_pixel_v(&mut self, _record: EMR_SETPIXELV) -> Result<(), PlayError> {
        info!("EMR_SETPIXELV: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn small_text_out(
        &mut self,
        _record: EMR_SMALLTEXTOUT,
    ) -> Result<(), PlayError> {
        info!("EMR_SMALLTEXTOUT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stroke_and_fill_path(
        &mut self,
        _record: EMR_STROKEANDFILLPATH,
    ) -> Result<(), PlayError> {
        info!("EMR_STROKEANDFILLPATH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stroke_path(
        &mut self,
        _record: EMR_STROKEPATH,
    ) -> Result<(), PlayError> {
        info!("EMR_STROKEPATH: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Escape Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn draw_escape(
        &mut self,
        _record: EMR_DRAWESCAPE,
    ) -> Result<(), PlayError> {
        info!("EMR_DRAWESCAPE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_escape(&mut self, _record: EMR_EXTESCAPE) -> Result<(), PlayError> {
        info!("EMR_EXTESCAPE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn named_escape(
        &mut self,
        _record: EMR_NAMEDESCAPE,
    ) -> Result<(), PlayError> {
        info!("EMR_NAMEDESCAPE: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Object Creation Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_brush_indirect(
        &mut self,
        record: EMR_CREATEBRUSHINDIRECT,
    ) -> Result<(), PlayError> {
        self.emf_object_table.set(
            record.ih_brush as usize,
            GraphicsObject::LogBrushEx(record.log_brush),
        );

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_color_space(
        &mut self,
        _record: EMR_CREATECOLORSPACE,
    ) -> Result<(), PlayError> {
        info!("EMR_CREATECOLORSPACE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_color_space_w(
        &mut self,
        _record: EMR_CREATECOLORSPACEW,
    ) -> Result<(), PlayError> {
        info!("EMR_CREATECOLORSPACEW: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_dib_pattern_brush_pt(
        &mut self,
        _record: EMR_CREATEDIBPATTERNBRUSHPT,
    ) -> Result<(), PlayError> {
        info!("EMR_CREATEDIBPATTERNBRUSHPT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_mono_brush(
        &mut self,
        _record: EMR_CREATEMONOBRUSH,
    ) -> Result<(), PlayError> {
        info!("EMR_CREATEMONOBRUSH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_palette(
        &mut self,
        _record: EMR_CREATEPALETTE,
    ) -> Result<(), PlayError> {
        info!("EMR_CREATEPALETTE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_pen(&mut self, record: EMR_CREATEPEN) -> Result<(), PlayError> {
        self.emf_object_table.set(
            record.ih_pen as usize,
            GraphicsObject::LogPenEx(record.log_pen.into()),
        );

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_create_font_indirect_w(
        &mut self,
        record: EMR_EXTCREATEFONTINDIRECTW,
    ) -> Result<(), PlayError> {
        let font = match record.elw {
            crate::parser::ELW::LogFontExDv(v) => {
                v.get(0).expect("should be set").clone()
            }
            crate::parser::ELW::LogFontPanose(v) => v.into(),
        };

        self.emf_object_table
            .set(record.ih_fonts as usize, GraphicsObject::LogFontExDv(font));

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_create_pen(
        &mut self,
        record: EMR_EXTCREATEPEN,
    ) -> Result<(), PlayError> {
        self.emf_object_table
            .set(record.ih_pen as usize, GraphicsObject::LogPenEx(record.elp));

        Ok(())
    }

    // .
    // .
    // Functions to handle Object Manipulation Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn color_correct_palette(
        &mut self,
        _record: EMR_COLORCORRECTPALETTE,
    ) -> Result<(), PlayError> {
        info!("EMR_COLORCORRECTPALETTE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn delete_color_space(
        &mut self,
        _record: EMR_DELETECOLORSPACE,
    ) -> Result<(), PlayError> {
        info!("EMR_DELETECOLORSPACE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn delete_object(
        &mut self,
        record: EMR_DELETEOBJECT,
    ) -> Result<(), PlayError> {
        self.emf_object_table.delete(record.in_object as usize);
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn resize_palette(
        &mut self,
        _record: EMR_RESIZEPALETTE,
    ) -> Result<(), PlayError> {
        info!("EMR_RESIZEPALETTE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_object(
        &mut self,
        record: EMR_SELECTOBJECT,
    ) -> Result<(), PlayError> {
        let emf_object = if let Some(stock_object) =
            StockObject::from_repr(record.in_object)
        {
            GraphicsObject::from(&self.selected_emf_object, stock_object)
        } else {
            self.emf_object_table.get(record.in_object as usize).clone()
        };

        match emf_object {
            GraphicsObject::DeviceIndependentBitmap(v) => {
                self.selected_emf_object.dib = v.into();
            }
            GraphicsObject::LogBrushEx(v) => {
                self.selected_emf_object.brush = v.into();
            }
            GraphicsObject::LogColorSpace(v) => {
                self.selected_emf_object.color_space = v.into();
            }
            GraphicsObject::LogColorSpaceW(v) => {
                self.selected_emf_object.color_space_w = v.into();
            }
            GraphicsObject::LogFont(v) => {
                self.selected_emf_object.font = v.into();
            }
            GraphicsObject::LogFontExDv(v) => {
                self.selected_emf_object.font_ex_dv = v.into();
            }
            GraphicsObject::LogPalette(v) => {
                self.selected_emf_object.palette = v.into();
            }
            GraphicsObject::LogPenEx(v) => {
                self.selected_emf_object.pen = v.into();
            }
            _ => {
                return Err(PlayError::UnexpectedGraphicsObject {
                    cause: format!(
                        "unexpected graphics object is selected: \
                         {emf_object:?}"
                    ),
                });
            }
        }

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_palette(
        &mut self,
        _record: EMR_SELECTPALETTE,
    ) -> Result<(), PlayError> {
        info!("EMR_SELECTPALETTE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_color_space(
        &mut self,
        _record: EMR_SETCOLORSPACE,
    ) -> Result<(), PlayError> {
        info!("EMR_SETCOLORSPACE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_palette_entries(
        &mut self,
        _record: EMR_SETPALETTEENTRIES,
    ) -> Result<(), PlayError> {
        info!("EMR_SETPALETTEENTRIES: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle OpenGL Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn gls_bounded_record(
        &mut self,
        _record: EMR_GLSBOUNDEDRECORD,
    ) -> Result<(), PlayError> {
        info!("EMR_GLSBOUNDEDRECORD: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn gls_record(&mut self, _record: EMR_GLSRECORD) -> Result<(), PlayError> {
        info!("EMR_GLSRECORD: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle Path Bracket Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn abort_path(&mut self, _record: EMR_ABORTPATH) -> Result<(), PlayError> {
        info!("EMR_ABORTPATH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn begin_path(&mut self, _record: EMR_BEGINPATH) -> Result<(), PlayError> {
        info!("EMR_BEGINPATH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn close_figure(
        &mut self,
        _record: EMR_CLOSEFIGURE,
    ) -> Result<(), PlayError> {
        info!("EMR_CLOSEFIGURE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn end_path(&mut self, _record: EMR_ENDPATH) -> Result<(), PlayError> {
        info!("EMR_ENDPATH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn flatten_path(
        &mut self,
        _record: EMR_FLATTENPATH,
    ) -> Result<(), PlayError> {
        info!("EMR_FLATTENPATH: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn widen_path(&mut self, _record: EMR_WIDENPATH) -> Result<(), PlayError> {
        info!("EMR_WIDENPATH: not implemented");
        Ok(())
    }

    // .
    // .
    // Functions to handle State Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn color_match_to_target_w(
        &mut self,
        _record: EMR_COLORMATCHTOTARGETW,
    ) -> Result<(), PlayError> {
        info!("EMR_COLORMATCHTOTARGETW: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn force_ufi_mapping(
        &mut self,
        _record: EMR_FORCEUFIMAPPING,
    ) -> Result<(), PlayError> {
        info!("EMR_FORCEUFIMAPPING: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn invert_rgn(&mut self, _record: EMR_INVERTRGN) -> Result<(), PlayError> {
        info!("EMR_INVERTRGN: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn move_to_ex(&mut self, _record: EMR_MOVETOEX) -> Result<(), PlayError> {
        info!("EMR_MOVETOEX: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn pixel_format(
        &mut self,
        _record: EMR_PIXELFORMAT,
    ) -> Result<(), PlayError> {
        info!("EMR_PIXELFORMAT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn realize_palette(
        &mut self,
        _record: EMR_REALIZEPALETTE,
    ) -> Result<(), PlayError> {
        info!("EMR_REALIZEPALETTE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn restore_dc(&mut self, record: EMR_RESTOREDC) -> Result<(), PlayError> {
        let mut current = record.saved_dc;

        while current < 0 {
            let Some(context) = self.context_stack.pop() else {
                return Err(PlayError::InvalidRecord {
                    cause: "device context to restore is not saved.".to_owned(),
                });
            };

            self.context = context;
            current += 1;
        }

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn save_dc(&mut self, _record: EMR_SAVEDC) -> Result<(), PlayError> {
        self.context_stack.push(self.context.clone());

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn scale_viewport_ext_ex(
        &mut self,
        _record: EMR_SCALEVIEWPORTEXTEX,
    ) -> Result<(), PlayError> {
        info!("EMR_SCALEVIEWPORTEXTEX: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn scale_window_ext_ex(
        &mut self,
        _record: EMR_SCALEWINDOWEXTEX,
    ) -> Result<(), PlayError> {
        info!("EMR_SCALEWINDOWEXTEX: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_arc_direction(
        &mut self,
        _record: EMR_SETARCDIRECTION,
    ) -> Result<(), PlayError> {
        info!("EMR_SETARCDIRECTION: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_color(
        &mut self,
        _record: EMR_SETBKCOLOR,
    ) -> Result<(), PlayError> {
        info!("EMR_SETBKCOLOR: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_mode(&mut self, _record: EMR_SETBKMODE) -> Result<(), PlayError> {
        info!("EMR_SETBKMODE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_brush_org_ex(
        &mut self,
        _record: EMR_SETBRUSHORGEX,
    ) -> Result<(), PlayError> {
        info!("EMR_SETBRUSHORGEX: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_color_adjustment(
        &mut self,
        _record: EMR_SETCOLORADJUSTMENT,
    ) -> Result<(), PlayError> {
        info!("EMR_SETCOLORADJUSTMENT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_icm_mode(
        &mut self,
        _record: EMR_SETICMMODE,
    ) -> Result<(), PlayError> {
        info!("EMR_SETICMMODE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_icm_profile_a(
        &mut self,
        _record: EMR_SETICMPROFILEA,
    ) -> Result<(), PlayError> {
        info!("EMR_SETICMPROFILEA: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_icm_profile_w(
        &mut self,
        _record: EMR_SETICMPROFILEW,
    ) -> Result<(), PlayError> {
        info!("EMR_SETICMPROFILEW: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_layout(&mut self, _record: EMR_SETLAYOUT) -> Result<(), PlayError> {
        info!("EMR_SETLAYOUT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_linked_ufis(
        &mut self,
        _record: EMR_SETLINKEDUFIS,
    ) -> Result<(), PlayError> {
        info!("EMR_SETLINKEDUFIS: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_map_mode(
        &mut self,
        _record: EMR_SETMAPMODE,
    ) -> Result<(), PlayError> {
        info!("EMR_SETMAPMODE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_mapper_flags(
        &mut self,
        _record: EMR_SETMAPPERFLAGS,
    ) -> Result<(), PlayError> {
        info!("EMR_SETMAPPERFLAGS: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_miter_limit(
        &mut self,
        _record: EMR_SETMITERLIMIT,
    ) -> Result<(), PlayError> {
        info!("EMR_SETMITERLIMIT: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_polyfill_mode(
        &mut self,
        _record: EMR_SETPOLYFILLMODE,
    ) -> Result<(), PlayError> {
        info!("EMR_SETPOLYFILLMODE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_rop2(&mut self, _record: EMR_SETROP2) -> Result<(), PlayError> {
        info!("EMR_SETROP2: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_stretch_blt_mode(
        &mut self,
        _record: EMR_SETSTRETCHBLTMODE,
    ) -> Result<(), PlayError> {
        info!("EMR_SETSTRETCHBLTMODE: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_align(
        &mut self,
        record: EMR_SETTEXTALIGN,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.text.text_alignment =
            record.text_alignment_mode;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_color(
        &mut self,
        record: EMR_SETTEXTCOLOR,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.text_color = record.color;
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_justification(
        &mut self,
        _record: EMR_SETTEXTJUSTIFICATION,
    ) -> Result<(), PlayError> {
        info!("EMR_SETTEXTJUSTIFICATION: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_viewport_ext_ex(
        &mut self,
        record: EMR_SETVIEWPORTEXTEX,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.regions.viewport.extent =
            record.extent;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_viewport_org_ex(
        &mut self,
        record: EMR_SETVIEWPORTORGEX,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.regions.viewport.origin =
            record.origin;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_window_ext_ex(
        &mut self,
        record: EMR_SETWINDOWEXTEX,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.regions.window.extent = record.extent;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_window_org_ex(
        &mut self,
        record: EMR_SETWINDOWORGEX,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.regions.window.origin = record.origin;

        Ok(())
    }

    // .
    // .
    // Functions to handle Transform Record
    // .
    // .
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn modify_world_transform(
        &mut self,
        _record: EMR_MODIFYWORLDTRANSFORM,
    ) -> Result<(), PlayError> {
        info!("EMR_MODIFYWORLDTRANSFORM: not implemented");
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_world_transform(
        &mut self,
        _record: EMR_SETWORLDTRANSFORM,
    ) -> Result<(), PlayError> {
        info!("EMR_SETWORLDTRANSFORM: not implemented");
        Ok(())
    }
}
