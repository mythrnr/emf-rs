mod node;
mod util;

use wmf_core::parser::{PointL, SizeL};

use crate::{
    converter::{
        playback_device_context::{
            point_s_to_point_l, EmfObjectTable, GraphicsEnvironment,
            GraphicsObject, PlaybackDeviceContext, PlaybackStateColors,
            PlaybackStateDrawing, PlaybackStateRegions, PlaybackStateText,
            SelectedObject, Viewport, Window,
        },
        svg::{
            node::{Data, Node},
            util::{
                as_point_string_from_point_l, as_point_string_from_point_s,
                color_from_color_ref, polygon_fill_rule, text_align,
                url_string, Fill, Stroke,
            },
        },
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
    path: Data,
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
            path: Data::new(),
        }
    }
}

impl SVGPlayer {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn issue_id(&self) -> String {
        format!("defs{}", self.definitions.len())
    }
}

impl crate::converter::Player for SVGPlayer {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        let Self { context, definitions, elements, .. } = self;

        let window = context.graphics_environment.regions.window;
        let mut document =
            Node::new("svg").set("xmlns", "http://www.w3.org/2000/svg").set(
                "viewBox",
                format!(
                    "{} {} {} {}",
                    window.origin.x,
                    window.origin.y,
                    window.extent.cx,
                    window.extent.cy,
                ),
            );

        if !definitions.is_empty() {
            let mut defs = Node::new("defs");
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
        record: EMR_STRETCHDIBITS,
    ) -> Result<(), PlayError> {
        let mut buf = &record.bmi_src[..];
        let (dib_header_info, _) = wmf_core::parser::BitmapInfoHeader::parse(
            &mut buf,
        )
        .map_err(|err| PlayError::InvalidRecord { cause: err.to_string() })?;
        let point =
            self.context.xform.transform_point_l(wmf_core::parser::PointL {
                x: record.x_dest,
                y: record.y_dest,
            });
        let (width, height) =
            (dib_header_info.width(), dib_header_info.height());
        let bitmap: wmf_core::converter::Bitmap =
            wmf_core::parser::DeviceIndependentBitmap {
                dib_header_info,
                colors: wmf_core::parser::Colors::Null,
                bitmap_buffer: wmf_core::parser::BitmapBuffer {
                    undefined_space: vec![],
                    a_data: record.bits_src,
                },
            }
            .into();

        let image = Node::new("image")
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("width", width.to_string())
            .set("height", height.to_string())
            .set("href", bitmap.as_data_url());

        self.elements.push(image);

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
        info!("EMR_COMMENT: not implemented");
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

        let color = if let Some(record_buffer) = record.emf_header_record_buffer
        {
            let pixel_format = match record_buffer {
                EmfHeaderRecordBuffer::EmfMetafileHeader { .. } => None,
                EmfHeaderRecordBuffer::EmfMetafileHeaderExtension1 {
                    emf_pixel_format,
                    ..
                }
                | EmfHeaderRecordBuffer::EmfMetafileHeaderExtension2 {
                    emf_pixel_format,
                    ..
                } => emf_pixel_format,
            };

            PlaybackStateColors { pixel_format, ..Default::default() }
        } else {
            PlaybackStateColors::default()
        };

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
            color,
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
    fn ellipse(&mut self, record: EMR_ELLIPSE) -> Result<(), PlayError> {
        let r =
            self.context.xform.transform_point_l(wmf_core::parser::PointL {
                x: (record.bx.right - record.bx.left) / 2,
                y: (record.bx.bottom - record.bx.top) / 2,
            });

        if r.x == 0 || r.y == 0 {
            info!(
                %r.x, %r.y,
                "EMR_ELLIPSE is skipped because rx or ry is zero.",
            );

            return Ok(());
        }

        let pen = &self.selected_emf_object.pen;
        let brush = &self.selected_emf_object.brush;
        let stroke = Stroke::from(pen.clone());
        let fill = match Fill::from(&self.context, brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            &self.context.graphics_environment.drawing.polyfill_mode,
        );
        let c =
            self.context.xform.transform_point_l(wmf_core::parser::PointL {
                x: record.bx.left,
                y: record.bx.top,
            });

        let ellipse = Node::new("ellipse")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("cx", (c.x + r.x).to_string())
            .set("cy", (c.y + r.y).to_string())
            .set("rx", r.x.to_string())
            .set("ry", r.y.to_string());
        let ellipse = stroke.set_props(&self.context, ellipse);

        self.elements.push(ellipse);

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
        record: EMR_EXTTEXTOUTW,
    ) -> Result<(), PlayError> {
        let font = if let Some(ref font) = self.selected_emf_object.font_ex_dv {
            &font.log_font_ex.log_font
        } else if let Some(ref font) = self.selected_emf_object.font {
            font
        } else {
            return Err(PlayError::UnexpectedGraphicsObject {
                cause: format!("font is not selected"),
            });
        };
        let color = color_from_color_ref(
            &self.context.graphics_environment.drawing.text_color,
        );
        let text_align =
            text_align(self.context.graphics_environment.text.text_alignment);
        let point =
            self.context.xform.transform_point_l(record.w_emr_text.reference);

        let text = Node::new("text")
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("text-anchor", text_align)
            .set("fill", color)
            .add(Node::new_text(record.w_emr_text.string_buffer));
        let (text, styles) = font.set_props(text, &point);
        let text = text.set("style", styles.join(""));

        self.elements.push(text);

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn fill_path(&mut self, _record: EMR_FILLPATH) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.path_bracket = false;
        if self.path.is_empty() {
            return Ok(());
        }

        self.path = self.path.clone();

        let brush = &self.selected_emf_object.brush;
        let fill = match Fill::from(&self.context, brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            &self.context.graphics_environment.drawing.polyfill_mode,
        );

        let path = Node::new("path")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("d", self.path.to_string());

        self.elements.push(path);
        self.path = Data::new();

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
    fn line_to(&mut self, record: EMR_LINETO) -> Result<(), PlayError> {
        let point = self.context.xform.transform_point_l(record.point);

        self.path =
            self.path.clone().line_to(format!("{} {}", point.x, point.y));

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
    fn poly_bezier(&mut self, record: EMR_POLYBEZIER) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        // NOTE: ignore move to first point for SVG.
        // let Some(point) = record.a_points.first() else {
        //     return Err(PlayError::InvalidRecord {
        //         cause: "aPoints[0] is not defined".to_owned(),
        //     });
        // };
        // let point = self.context.xform.transform_point_l(point.clone());
        // let mut data =
        //     self.path.clone().move_to(format!("{} {}", point.x, point.y));

        let mut data = self.path.clone();
        let mut c = vec![];

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.xform.transform_point_l(point.clone());
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                data = data.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );

                // reset for next curve.
                c = vec![];
            }
        }

        self.path = data;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_16(
        &mut self,
        record: EMR_POLYBEZIER16,
    ) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        // NOTE: ignore move to first point for SVG.
        // let Some(point) = record.a_points.first() else {
        //     return Err(PlayError::InvalidRecord {
        //         cause: "aPoints[0] is not defined".to_owned(),
        //     });
        // };
        // let point = self.context.xform.transform_point_s(point.clone());
        // let mut data =
        //     self.path.clone().move_to(format!("{} {}", point.x, point.y));

        let mut data = self.path.clone();
        let mut c = vec![];

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(&point);

            let point = self.context.xform.transform_point_s(point.clone());
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                data = data.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                c = vec![];
            }
        }

        self.path = data;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_to(
        &mut self,
        record: EMR_POLYBEZIERTO,
    ) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        // NOTE: ignore move to first point for SVG.
        // let mut data = self.path.clone().move_to(format!(
        //     "{} {}",
        //     self.context.graphics_environment.drawing.current_position.x,
        //     self.context.graphics_environment.drawing.current_position.y
        // ));

        let mut data = self.path.clone();
        let mut c = vec![];

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.xform.transform_point_l(point.clone());
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                data = data.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                c = vec![];
            }
        }

        self.path = data;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_to_16(
        &mut self,
        record: EMR_POLYBEZIERTO16,
    ) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        // NOTE: ignore move to first point for SVG.
        // let point = self.context.xform.transform_point_l(
        //     self.context.graphics_environment.drawing.current_position.
        // clone(), );
        // let mut data =
        //     self.path.clone().move_to(format!("{} {}", point.x, point.y));

        let mut data = self.path.clone();
        let mut c = vec![];

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(&point);

            let point = self.context.xform.transform_point_s(point.clone());
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                data = data.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                c = vec![];
            }
        }

        self.path = data;

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
        record: EMR_POLYPOLYGON,
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
    fn polygon(&mut self, record: EMR_POLYGON) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polygon has no points");
            return Ok(());
        }

        let pen = &self.selected_emf_object.pen;
        let brush = &self.selected_emf_object.brush;
        let stroke = Stroke::from(pen.clone());
        let fill = match Fill::from(&self.context, brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            &self.context.graphics_environment.drawing.polyfill_mode,
        );

        let mut points = vec![];

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.xform.transform_point_l(point.clone());
            points.push(as_point_string_from_point_l(&point));
        }

        let polygon = Node::new("polygon")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("points", points.join(" "));
        let polygon = stroke.set_props(&self.context, polygon);

        self.elements.push(polygon);

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polygon_16(&mut self, record: EMR_POLYGON16) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polygon has no points");
            return Ok(());
        }

        let pen = &self.selected_emf_object.pen;
        let brush = &self.selected_emf_object.brush;
        let stroke = Stroke::from(pen.clone());
        let fill = match Fill::from(&self.context, brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            &self.context.graphics_environment.drawing.polyfill_mode,
        );

        let mut points = vec![];

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(&point);

            let point = self.context.xform.transform_point_s(point.clone());
            points.push(as_point_string_from_point_s(&point));
        }

        let polygon = Node::new("polygon")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("points", points.join(" "));
        let polygon = stroke.set_props(&self.context, polygon);

        self.elements.push(polygon);

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline(&mut self, record: EMR_POLYLINE) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        let Some(point) = record.a_points.first() else {
            return Err(PlayError::InvalidRecord {
                cause: "aPoints[0] is not defined".to_owned(),
            });
        };

        let point = self.context.xform.transform_point_l(point.clone());
        let mut data =
            self.path.clone().move_to(format!("{} {}", point.x, point.y));

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.xform.transform_point_l(point.clone());
            data = data.line_to(format!("{} {}", point.x, point.y));
        }

        self.path = data;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_16(&mut self, record: EMR_POLYLINE16) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        let Some(point) = record.a_points.first() else {
            return Err(PlayError::InvalidRecord {
                cause: "aPoints[0] is not defined".to_owned(),
            });
        };

        let mut data =
            self.path.clone().move_to(format!("{} {}", point.x, point.y));

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(point);

            let point = self.context.xform.transform_point_s(point.clone());
            data = data.line_to(format!("{} {}", point.x, point.y));
        }

        self.path = data;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_to(&mut self, record: EMR_POLYLINETO) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        let mut data = self.path.clone();

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.xform.transform_point_l(point.clone());
            data = data.line_to(format!("{} {}", point.x, point.y));
        }

        self.path = data;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_to_16(
        &mut self,
        record: EMR_POLYLINETO16,
    ) -> Result<(), PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(());
        }

        let mut data = self.path.clone();

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(point);

            let point = self.context.xform.transform_point_s(point.clone());
            data = data.line_to(format!("{} {}", point.x, point.y));
        }

        self.path = data;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn rectangle(&mut self, record: EMR_RECTANGLE) -> Result<(), PlayError> {
        let pen = &self.selected_emf_object.pen;
        let brush = &self.selected_emf_object.brush;
        let stroke = Stroke::from(pen.clone());
        let fill = match Fill::from(&self.context, brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            &self.context.graphics_environment.drawing.polyfill_mode,
        );

        let top_left =
            self.context.xform.transform_point_l(wmf_core::parser::PointL {
                x: record.bx.left,
                y: record.bx.top,
            });
        let bottom_right =
            self.context.xform.transform_point_l(wmf_core::parser::PointL {
                x: record.bx.right,
                y: record.bx.bottom,
            });

        let rect = Node::new("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("x", top_left.x)
            .set("y", top_left.y)
            .set("height", (bottom_right.x - top_left.x).to_string())
            .set("width", (bottom_right.y - top_left.y).to_string());
        let rect = stroke.set_props(&self.context, rect);

        self.elements.push(rect);

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
        record: EMR_STROKEANDFILLPATH,
    ) -> Result<(), PlayError> {
        if self.path.is_empty() {
            return Ok(());
        }

        let pen = &self.selected_emf_object.pen;
        let brush = &self.selected_emf_object.brush;
        let stroke = Stroke::from(pen.clone());
        let fill = match Fill::from(&self.context, brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.issue_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            &self.context.graphics_environment.drawing.polyfill_mode,
        );

        let path = Node::new("path")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("d", self.path.to_string());
        let path = stroke.set_props(&self.context, path);

        self.elements.push(path);
        self.path = Data::new();

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stroke_path(&mut self, record: EMR_STROKEPATH) -> Result<(), PlayError> {
        if self.path.is_empty() {
            return Ok(());
        }

        let pen = &self.selected_emf_object.pen;
        let stroke = Stroke::from(pen.clone());
        let path = Node::new("path")
            .set("fill", "none")
            .set("d", self.path.to_string());
        let path = stroke.set_props(&self.context, path);

        self.elements.push(path);
        self.path = Data::new();

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
        self.path = Data::new();
        self.context.graphics_environment.drawing.path_bracket = false;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn begin_path(&mut self, _record: EMR_BEGINPATH) -> Result<(), PlayError> {
        if self.context.graphics_environment.drawing.path_bracket {
            return Err(PlayError::InvalidRecord {
                cause: "Path bracket construction MUST be closed by an \
                        EMR_ABORTPATH or EMR_ENDPATH record."
                    .to_owned(),
            });
        }

        self.context.graphics_environment.drawing.path_bracket = true;

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
        self.context.graphics_environment.drawing.path_bracket = false;
        self.path = self.path.clone().close();

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn end_path(&mut self, _record: EMR_ENDPATH) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.path_bracket = false;
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
        if self.path.is_empty() {
            return Ok(());
        }

        let pen = &self.selected_emf_object.pen;
        let stroke = Stroke::from(pen.clone());
        let path = Node::new("path")
            .set("fill", "none")
            .set("d", self.path.to_string());
        let path = stroke.set_props(&self.context, path);

        self.elements.push(path);

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
        record: EMR_FORCEUFIMAPPING,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.text.force_ufi_mapping =
            record.ufi.into();

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
    fn move_to_ex(&mut self, record: EMR_MOVETOEX) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.current_position =
            record.offset.clone();

        let point = self.context.xform.transform_point_l(record.offset);

        self.path =
            self.path.clone().move_to(format!("{} {}", point.x, point.y));

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn pixel_format(
        &mut self,
        record: EMR_PIXELFORMAT,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.color.pixel_format =
            record.pfd.into();

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
        record: EMR_SETARCDIRECTION,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.arc_direction =
            record.arc_direction;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_color(
        &mut self,
        record: EMR_SETBKCOLOR,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.background_color =
            record.color;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_mode(&mut self, record: EMR_SETBKMODE) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.background_mode =
            record.background_mode;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_brush_org_ex(
        &mut self,
        record: EMR_SETBRUSHORGEX,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.brush_origin = record.origin;
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_color_adjustment(
        &mut self,
        record: EMR_SETCOLORADJUSTMENT,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.color.color_adjustment =
            record.color_adjustment;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_icm_mode(
        &mut self,
        record: EMR_SETICMMODE,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.color.icm_mode = record.icm_mode;
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
    fn set_layout(&mut self, record: EMR_SETLAYOUT) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.layout_mode =
            record.layout_mode;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_linked_ufis(
        &mut self,
        record: EMR_SETLINKEDUFIS,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.text.linked_ufis = record.ufis;
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_map_mode(
        &mut self,
        record: EMR_SETMAPMODE,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.mapping_mode =
            record.map_mode;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_mapper_flags(
        &mut self,
        record: EMR_SETMAPPERFLAGS,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.text.font_mapper_flags = record.flags;
        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_miter_limit(
        &mut self,
        record: EMR_SETMITERLIMIT,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.miter_limit =
            record.miter_limit.into();

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_polyfill_mode(
        &mut self,
        record: EMR_SETPOLYFILLMODE,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.polyfill_mode =
            record.polygon_fill_mode;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_rop2(&mut self, record: EMR_SETROP2) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.rop2 = record.rop2_mode;

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_stretch_blt_mode(
        &mut self,
        record: EMR_SETSTRETCHBLTMODE,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.drawing.stretch_blt_mode =
            record.stretch_mode;

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
        record: EMR_SETTEXTJUSTIFICATION,
    ) -> Result<(), PlayError> {
        self.context.graphics_environment.text.text_justification =
            (record.n_break_extra, record.n_break_count);

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
        record: EMR_MODIFYWORLDTRANSFORM,
    ) -> Result<(), PlayError> {
        let (a, b) = match record.modify_world_transform_mode {
            ModifyWorldTransformMode::MWT_IDENTITY => {
                // NOOP
                return Ok(());
            }
            ModifyWorldTransformMode::MWT_LEFTMULTIPLY => {
                (record.x_form, self.context.xform.clone())
            }
            ModifyWorldTransformMode::MWT_RIGHTMULTIPLY => {
                (self.context.xform.clone(), record.x_form)
            }
            ModifyWorldTransformMode::MWT_SET => {
                self.context.xform = record.x_form;
                return Ok(());
            }
        };

        self.context.xform = XForm {
            m11: a.m11 * b.m11 + a.m12 * b.m21,
            m12: a.m11 * b.m12 + a.m12 * b.m22,
            m21: a.m21 * b.m11 + a.m22 * b.m21,
            m22: a.m21 * b.m12 + a.m22 * b.m22,
            dx: a.dx * b.m11 + a.dy * b.m21 + b.dx,
            dy: a.dy * b.m12 + a.dy * b.m22 + b.dy,
        };

        Ok(())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_world_transform(
        &mut self,
        record: EMR_SETWORLDTRANSFORM,
    ) -> Result<(), PlayError> {
        self.context.xform = record.x_form;

        Ok(())
    }
}
