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
    window: Window,
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
            window: Window {
                extent: wmf_core::parser::SizeL { cx: 0, cy: 0 },
                origin: wmf_core::parser::PointL { x: 0, y: 0 },
            },
        }
    }
}

impl SVGPlayer {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn generate_definition_id(&self) -> String {
        format!("defs{}", self.definitions.len())
    }

    #[inline]
    fn push_element(&mut self, record_number: usize, element: Node) {
        let element = element.set("id", format!("elem{record_number}"));
        self.elements.push(element);
    }
}

impl crate::converter::Player for SVGPlayer {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn generate(self) -> Result<Vec<u8>, PlayError> {
        let Self { definitions, elements, window, .. } = self;

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
        mut self,
        record_number: usize,
        record: EMR_ALPHABLEND,
    ) -> Result<Self, PlayError> {
        let dib_header_info = {
            let mut buf = &record.bmi_src[..];
            let (dib_header_info, _) =
                wmf_core::parser::BitmapInfoHeader::parse(&mut buf).map_err(
                    |err| PlayError::InvalidRecord { cause: err.to_string() },
                )?;

            dib_header_info
        };
        let colors = {
            let mut buf =
                &record.bmi_src[dib_header_info.header_size() as usize..];
            let (colors, _) = wmf_core::parser::Colors::parse(
                &mut buf,
                record.usage_src.into(),
                &dib_header_info,
            )
            .map_err(|err| PlayError::InvalidRecord {
                cause: err.to_string(),
            })?;

            colors
        };

        let (width, height) =
            (dib_header_info.width(), dib_header_info.height());
        let bitmap: wmf_core::converter::Bitmap =
            wmf_core::parser::DeviceIndependentBitmap {
                dib_header_info,
                colors,
                bitmap_buffer: wmf_core::parser::BitmapBuffer {
                    undefined_space: vec![],
                    a_data: record.bits_src,
                },
            }
            .into();

        let point = self.context.transform_point_l(&wmf_core::parser::PointL {
            x: record.x_dest,
            y: record.y_dest,
        });

        let image = Node::new("image")
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("width", width.to_string())
            .set("height", height.to_string())
            .set("href", bitmap.as_data_url());

        self.push_element(record_number, image);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn bit_blt(
        self,
        record_number: usize,
        record: EMR_BITBLT,
    ) -> Result<Self, PlayError> {
        info!("EMR_BITBLT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn mask_blt(
        self,
        record_number: usize,
        record: EMR_MASKBLT,
    ) -> Result<Self, PlayError> {
        info!("EMR_MASKBLT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn plg_blt(
        self,
        record_number: usize,
        record: EMR_PLGBLT,
    ) -> Result<Self, PlayError> {
        info!("EMR_PLGBLT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_dibits_to_device(
        self,
        record_number: usize,
        record: EMR_SETDIBITSTODEVICE,
    ) -> Result<Self, PlayError> {
        info!("EMR_SETDIBITSTODEVICE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stretch_blt(
        self,
        record_number: usize,
        record: EMR_STRETCHBLT,
    ) -> Result<Self, PlayError> {
        info!("EMR_STRETCHBLT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stretch_dibits(
        mut self,
        record_number: usize,
        record: EMR_STRETCHDIBITS,
    ) -> Result<Self, PlayError> {
        let dib_header_info = {
            let mut buf = &record.bmi_src[..];
            let (dib_header_info, _) =
                wmf_core::parser::BitmapInfoHeader::parse(&mut buf).map_err(
                    |err| PlayError::InvalidRecord { cause: err.to_string() },
                )?;

            dib_header_info
        };
        let colors = {
            let mut buf =
                &record.bmi_src[dib_header_info.header_size() as usize..];
            let (colors, _) = wmf_core::parser::Colors::parse(
                &mut buf,
                record.usage_src.into(),
                &dib_header_info,
            )
            .map_err(|err| PlayError::InvalidRecord {
                cause: err.to_string(),
            })?;

            colors
        };

        let (width, height) =
            (dib_header_info.width(), dib_header_info.height());
        let bitmap: wmf_core::converter::Bitmap =
            wmf_core::parser::DeviceIndependentBitmap {
                dib_header_info,
                colors,
                bitmap_buffer: wmf_core::parser::BitmapBuffer {
                    undefined_space: vec![],
                    a_data: record.bits_src,
                },
            }
            .into();

        let point = self.context.transform_point_l(&wmf_core::parser::PointL {
            x: record.x_dest,
            y: record.y_dest,
        });

        let image = Node::new("image")
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("width", width.to_string())
            .set("height", height.to_string())
            .set("href", bitmap.as_data_url());

        self.push_element(record_number, image);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn transparent_blt(
        self,
        record_number: usize,
        record: EMR_TRANSPARENTBLT,
    ) -> Result<Self, PlayError> {
        info!("EMR_TRANSPARENTBLT: not implemented");
        Ok(self)
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
        self,
        record_number: usize,
        record: EMR_EXCLUDECLIPRECT,
    ) -> Result<Self, PlayError> {
        info!("EMR_EXCLUDECLIPRECT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_select_clip_rgn(
        self,
        record_number: usize,
        record: EMR_EXTSELECTCLIPRGN,
    ) -> Result<Self, PlayError> {
        info!("EMR_EXTSELECTCLIPRGN: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn intersect_clip_rect(
        self,
        record_number: usize,
        record: EMR_INTERSECTCLIPRECT,
    ) -> Result<Self, PlayError> {
        info!("EMR_INTERSECTCLIPRECT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn offset_clip_rgn(
        self,
        record_number: usize,
        record: EMR_OFFSETCLIPRGN,
    ) -> Result<Self, PlayError> {
        info!("EMR_OFFSETCLIPRGN: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_clip_path(
        self,
        record_number: usize,
        record: EMR_SELECTCLIPPATH,
    ) -> Result<Self, PlayError> {
        info!("EMR_SELECTCLIPPATH: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_meta_rgn(
        self,
        record_number: usize,
        record: EMR_SETMETARGN,
    ) -> Result<Self, PlayError> {
        info!("EMR_SETMETARGN: not implemented");
        Ok(self)
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
    fn comment(
        self,
        record_number: usize,
        record: EMR_COMMENT,
    ) -> Result<Self, PlayError> {
        info!("EMR_COMMENT: not implemented");
        Ok(self)
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
    fn eof(
        self,
        record_number: usize,
        record: EMR_EOF,
    ) -> Result<Self, PlayError> {
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn header(
        mut self,
        record_number: usize,
        record: EMR_HEADER,
    ) -> Result<Self, PlayError> {
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

        self.window = Window { extent: extent.clone(), origin: origin.clone() };
        self.context.graphics_environment = GraphicsEnvironment {
            regions: PlaybackStateRegions {
                // clipping: None,
                // meta_clipping: None,
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

        Ok(self)
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
    fn angle_arc(
        self,
        record_number: usize,
        record: EMR_ANGLEARC,
    ) -> Result<Self, PlayError> {
        info!("EMR_ANGLEARC: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn arc(
        self,
        record_number: usize,
        record: EMR_ARC,
    ) -> Result<Self, PlayError> {
        info!("EMR_ARC: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn arc_to(
        self,
        record_number: usize,
        record: EMR_ARCTO,
    ) -> Result<Self, PlayError> {
        info!("EMR_ARCTO: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn chord(
        self,
        record_number: usize,
        record: EMR_CHORD,
    ) -> Result<Self, PlayError> {
        info!("EMR_CHORD: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ellipse(
        mut self,
        record_number: usize,
        record: EMR_ELLIPSE,
    ) -> Result<Self, PlayError> {
        let r = self.context.transform_point_l(&wmf_core::parser::PointL {
            x: (record.bx.right - record.bx.left) / 2,
            y: (record.bx.bottom - record.bx.top) / 2,
        });

        if r.x == 0 || r.y == 0 {
            info!(
                %r.x, %r.y,
                "EMR_ELLIPSE is skipped because rx or ry is zero.",
            );

            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let fill = match Fill::from(
            &self.context,
            self.selected_emf_object.brush.clone(),
        ) {
            Fill::Pattern { pattern } => {
                let id = self.generate_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            self.context.graphics_environment.drawing.polyfill_mode,
        );
        let c = self.context.transform_point_l(&wmf_core::parser::PointL {
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

        self.push_element(record_number, ellipse);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_flood_fill(
        self,
        record_number: usize,
        record: EMR_EXTFLOODFILL,
    ) -> Result<Self, PlayError> {
        info!("EMR_EXTFLOODFILL: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_text_out_a(
        self,
        record_number: usize,
        record: EMR_EXTTEXTOUTA,
    ) -> Result<Self, PlayError> {
        info!("EMR_EXTTEXTOUTA: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_text_out_w(
        mut self,
        record_number: usize,
        record: EMR_EXTTEXTOUTW,
    ) -> Result<Self, PlayError> {
        let font = if let Some(ref font) = self.selected_emf_object.font_ex_dv {
            &font.log_font_ex.log_font
        } else if let Some(ref font) = self.selected_emf_object.font {
            font
        } else {
            return Err(PlayError::UnexpectedGraphicsObject {
                cause: "font is not selected".to_owned(),
            });
        };
        let color = color_from_color_ref(
            &self.context.graphics_environment.drawing.text_color,
        );
        let text_align =
            text_align(self.context.graphics_environment.text.text_alignment);
        let point =
            self.context.transform_point_l(&record.w_emr_text.reference);

        let text = Node::new("text")
            .set("x", point.x.to_string())
            .set("y", point.y.to_string())
            .set("text-anchor", text_align)
            .set("fill", color)
            .add(Node::new_text(record.w_emr_text.string_buffer));
        let (text, styles) = font.set_props(&self.context, text, &point);
        let text = text.set("style", styles.join(""));

        self.push_element(record_number, text);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn fill_path(
        mut self,
        record_number: usize,
        record: EMR_FILLPATH,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.path_bracket = false;
        if self.path.is_empty() {
            return Ok(self);
        }

        let fill = match Fill::from(
            &self.context,
            self.selected_emf_object.brush.clone(),
        ) {
            Fill::Pattern { pattern } => {
                let id = self.generate_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            self.context.graphics_environment.drawing.polyfill_mode,
        );

        let path = Node::new("path")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("d", self.path.to_string());

        self.push_element(record_number, path);
        self.path = Data::new();

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn fill_rgn(
        self,
        record_number: usize,
        record: EMR_FILLRGN,
    ) -> Result<Self, PlayError> {
        info!("EMR_FILLRGN: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn frame_rgn(
        self,
        record_number: usize,
        record: EMR_FRAMERGN,
    ) -> Result<Self, PlayError> {
        info!("EMR_FRAMERGN: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn gradient_fill(
        self,
        record_number: usize,
        record: EMR_GRADIENTFILL,
    ) -> Result<Self, PlayError> {
        info!("EMR_GRADIENTFILL: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn line_to(
        mut self,
        record_number: usize,
        record: EMR_LINETO,
    ) -> Result<Self, PlayError> {
        let point = self.context.transform_point_l(&record.point);
        self.path = self.path.line_to(format!("{} {}", point.x, point.y));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn paint_rgn(
        self,
        record_number: usize,
        record: EMR_PAINTRGN,
    ) -> Result<Self, PlayError> {
        info!("EMR_PAINTRGN: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn pie(
        self,
        record_number: usize,
        record: EMR_PIE,
    ) -> Result<Self, PlayError> {
        info!("EMR_PIE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier(
        mut self,
        record_number: usize,
        record: EMR_POLYBEZIER,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
        }

        // NOTE: ignore move to first point for SVG.
        // let Some(point) = record.a_points.first() else {
        //     return Err(PlayError::InvalidRecord {
        //         cause: "aPoints[0] is not defined".to_owned(),
        //     });
        // };
        // let point = self.context.transform_point_l(point);
        // self.path = self.path.clone().move_to(format!("{} {}", point.x,
        // point.y));

        let mut c = vec![];

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.transform_point_l(point);
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                self.path = self.path.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );

                // reset for next curve.
                c = vec![];
            }
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_16(
        mut self,
        record_number: usize,
        record: EMR_POLYBEZIER16,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
        }

        // NOTE: ignore move to first point for SVG.
        // let Some(point) = record.a_points.first() else {
        //     return Err(PlayError::InvalidRecord {
        //         cause: "aPoints[0] is not defined".to_owned(),
        //     });
        // };
        // let point = self.context.transform_point_s(point);
        // self.path = self.path.clone().move_to(format!("{} {}", point.x,
        // point.y));

        let mut c = vec![];

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(point);

            let point = self.context.transform_point_s(point);
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                self.path = self.path.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                c = vec![];
            }
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_to(
        mut self,
        record_number: usize,
        record: EMR_POLYBEZIERTO,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
        }

        // NOTE: ignore move to first point for SVG.
        // self.path = self.path.clone().move_to(format!(
        //     "{} {}",
        //     self.context.graphics_environment.drawing.current_position.x,
        //     self.context.graphics_environment.drawing.current_position.y
        // ));

        let mut c = vec![];

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.transform_point_l(point);
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                self.path = self.path.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                c = vec![];
            }
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_bezier_to_16(
        mut self,
        record_number: usize,
        record: EMR_POLYBEZIERTO16,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
        }

        // NOTE: ignore move to first point for SVG.
        // let point = self.context.transform_point_l(&
        //     self.context.graphics_environment.drawing.current_position.
        // clone(), );
        // self.path = self.path.move_to(format!("{} {}", point.x, point.y));

        let mut c = vec![];

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(point);

            let point = self.context.transform_point_s(point);
            c.extend([point.x, point.y]);

            if c.len() % 3 == 0 {
                self.path = self.path.curve_to(
                    c.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" "),
                );
                c = vec![];
            }
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_draw(
        self,
        record_number: usize,
        record: EMR_POLYDRAW,
    ) -> Result<Self, PlayError> {
        info!("EMR_POLYDRAW: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_draw_16(
        self,
        record_number: usize,
        record: EMR_POLYDRAW16,
    ) -> Result<Self, PlayError> {
        info!("EMR_POLYDRAW16: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polygon(
        self,
        record_number: usize,
        record: EMR_POLYPOLYGON,
    ) -> Result<Self, PlayError> {
        info!("EMR_POLYPOLYGON: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polygon_16(
        mut self,
        record_number: usize,
        record: EMR_POLYPOLYGON16,
    ) -> Result<Self, PlayError> {
        if record.number_of_polygons == 0 || record.count == 0 {
            info!(%record.number_of_polygons, %record.count, "polygon has no points");
            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let fill = match Fill::from(
            &self.context,
            self.selected_emf_object.brush.clone(),
        ) {
            Fill::Pattern { pattern } => {
                let id = self.generate_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            self.context.graphics_environment.drawing.polyfill_mode,
        );

        let mut a_point: VecDeque<_> = record.a_points.into();
        let mut current_point_index = 0;

        for i in 0..record.number_of_polygons {
            let Some(points_of_polygon) =
                record.polygon_point_count.get(i as usize)
            else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("PolygonPointCount[{i}] is not defined"),
                });
            };

            let mut points = vec![];

            for _ in 0..*points_of_polygon {
                let Some(point) = a_point.pop_front() else {
                    return Err(PlayError::InvalidRecord {
                        cause: format!(
                            "aPoints[{current_point_index}] is not defined"
                        ),
                    });
                };

                self.context.graphics_environment.drawing.current_position =
                    point_s_to_point_l(&point);

                let point = self.context.transform_point_s(&point);
                points.push(as_point_string_from_point_s(&point));
                current_point_index += 1;
            }

            let polygon = Node::new("polygon")
                .set("fill", fill.as_str())
                .set("fill-rule", fill_rule.as_str())
                .set("points", points.join(" "));
            let polygon = stroke.set_props(&self.context, polygon);

            self.push_element(record_number, polygon);
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polyline(
        self,
        record_number: usize,
        record: EMR_POLYPOLYLINE,
    ) -> Result<Self, PlayError> {
        info!("EMR_POLYPOLYLINE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_polyline_16(
        self,
        record_number: usize,
        record: EMR_POLYPOLYLINE16,
    ) -> Result<Self, PlayError> {
        info!("EMR_POLYPOLYLINE16: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_text_out_a(
        self,
        record_number: usize,
        record: EMR_POLYTEXTOUTA,
    ) -> Result<Self, PlayError> {
        info!("EMR_POLYTEXTOUTA: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn poly_text_out_w(
        self,
        record_number: usize,
        record: EMR_POLYTEXTOUTW,
    ) -> Result<Self, PlayError> {
        info!("EMR_POLYTEXTOUTW: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polygon(
        mut self,
        record_number: usize,
        record: EMR_POLYGON,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polygon has no points");
            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let fill = match Fill::from(
            &self.context,
            self.selected_emf_object.brush.clone(),
        ) {
            Fill::Pattern { pattern } => {
                let id = self.generate_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            self.context.graphics_environment.drawing.polyfill_mode,
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

            let point = self.context.transform_point_l(point);
            points.push(as_point_string_from_point_l(&point));
        }

        let polygon = Node::new("polygon")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("points", points.join(" "));
        let polygon = stroke.set_props(&self.context, polygon);

        self.push_element(record_number, polygon);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polygon_16(
        mut self,
        record_number: usize,
        record: EMR_POLYGON16,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polygon has no points");
            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let fill = match Fill::from(
            &self.context,
            self.selected_emf_object.brush.clone(),
        ) {
            Fill::Pattern { pattern } => {
                let id = self.generate_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            self.context.graphics_environment.drawing.polyfill_mode,
        );

        let mut points = vec![];

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point_s_to_point_l(point);

            let point = self.context.transform_point_s(point);
            points.push(as_point_string_from_point_s(&point));
        }

        let polygon = Node::new("polygon")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("points", points.join(" "));
        let polygon = stroke.set_props(&self.context, polygon);

        self.push_element(record_number, polygon);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline(
        mut self,
        record_number: usize,
        record: EMR_POLYLINE,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
        }

        let Some(point) = record.a_points.first() else {
            return Err(PlayError::InvalidRecord {
                cause: "aPoints[0] is not defined".to_owned(),
            });
        };

        let point = self.context.transform_point_l(point);
        self.path = self.path.move_to(format!("{} {}", point.x, point.y));

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.transform_point_l(point);
            self.path = self.path.line_to(format!("{} {}", point.x, point.y));
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_16(
        mut self,
        record_number: usize,
        record: EMR_POLYLINE16,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
        }

        let mut data = Data::new();

        data = {
            let Some(point) = record.a_points.first() else {
                return Err(PlayError::InvalidRecord {
                    cause: "aPoints[0] is not defined".to_owned(),
                });
            };

            let point = self.context.transform_point_s(point);

            data.move_to(format!("{} {}", point.x, point.y))
        };

        for i in 1..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            let point = self.context.transform_point_s(point);
            data = data.line_to(format!("{} {}", point.x, point.y));
        }

        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let path =
            Node::new("path").set("fill", "none").set("d", data.to_string());
        let path = stroke.set_props(&self.context, path);

        self.push_element(record_number, path);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_to(
        mut self,
        record_number: usize,
        record: EMR_POLYLINETO,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
        }

        for i in 0..record.count {
            let Some(point) = record.a_points.get(i as usize) else {
                return Err(PlayError::InvalidRecord {
                    cause: format!("aPoints[{i}] is not defined"),
                });
            };

            self.context.graphics_environment.drawing.current_position =
                point.clone();

            let point = self.context.transform_point_l(point);
            self.path = self.path.line_to(format!("{} {}", point.x, point.y));
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn polyline_to_16(
        mut self,
        record_number: usize,
        record: EMR_POLYLINETO16,
    ) -> Result<Self, PlayError> {
        if record.count == 0 {
            info!(%record.count, "polyline has no points");
            return Ok(self);
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

            let point = self.context.transform_point_s(point);
            data = data.line_to(format!("{} {}", point.x, point.y));
        }

        self.path = data;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn rectangle(
        mut self,
        record_number: usize,
        record: EMR_RECTANGLE,
    ) -> Result<Self, PlayError> {
        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let fill = match Fill::from(
            &self.context,
            self.selected_emf_object.brush.clone(),
        ) {
            Fill::Pattern { pattern } => {
                let id = self.generate_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            self.context.graphics_environment.drawing.polyfill_mode,
        );

        let top_left =
            self.context.transform_point_l(&wmf_core::parser::PointL {
                x: record.bx.left,
                y: record.bx.top,
            });
        let bottom_right =
            self.context.transform_point_l(&wmf_core::parser::PointL {
                x: record.bx.right,
                y: record.bx.bottom,
            });

        let rect = Node::new("rect")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("x", top_left.x.to_string())
            .set("y", top_left.y.to_string())
            .set("height", (bottom_right.x - top_left.x).to_string())
            .set("width", (bottom_right.y - top_left.y).to_string());
        let rect = stroke.set_props(&self.context, rect);

        self.push_element(record_number, rect);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn round_rect(
        self,
        record_number: usize,
        record: EMR_ROUNDRECT,
    ) -> Result<Self, PlayError> {
        info!("EMR_ROUNDRECT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_pixel_v(
        self,
        record_number: usize,
        record: EMR_SETPIXELV,
    ) -> Result<Self, PlayError> {
        info!("EMR_SETPIXELV: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn small_text_out(
        self,
        record_number: usize,
        record: EMR_SMALLTEXTOUT,
    ) -> Result<Self, PlayError> {
        info!("EMR_SMALLTEXTOUT: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stroke_and_fill_path(
        mut self,
        record_number: usize,
        record: EMR_STROKEANDFILLPATH,
    ) -> Result<Self, PlayError> {
        if self.path.is_empty() {
            return Ok(self);
        }

        let brush = &self.selected_emf_object.brush;
        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let fill = match Fill::from(&self.context, brush.clone()) {
            Fill::Pattern { pattern } => {
                let id = self.generate_definition_id();
                self.definitions.push(pattern.set("id", id.as_str()));
                url_string(format!("#{id}").as_str())
            }
            Fill::Value { value } => value,
        };
        let fill_rule = polygon_fill_rule(
            self.context.graphics_environment.drawing.polyfill_mode,
        );

        let path = Node::new("path")
            .set("fill", fill.as_str())
            .set("fill-rule", fill_rule.as_str())
            .set("d", self.path.to_string());
        let path = stroke.set_props(&self.context, path);

        self.push_element(record_number, path);
        self.path = Data::new();

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn stroke_path(
        mut self,
        record_number: usize,
        record: EMR_STROKEPATH,
    ) -> Result<Self, PlayError> {
        if self.path.is_empty() {
            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let path = Node::new("path")
            .set("fill", "none")
            .set("d", self.path.to_string());
        let path = stroke.set_props(&self.context, path);

        self.push_element(record_number, path);
        self.path = Data::new();

        Ok(self)
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
        self,
        record_number: usize,
        record: EMR_DRAWESCAPE,
    ) -> Result<Self, PlayError> {
        info!("EMR_DRAWESCAPE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_escape(
        self,
        record_number: usize,
        record: EMR_EXTESCAPE,
    ) -> Result<Self, PlayError> {
        info!("EMR_EXTESCAPE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn named_escape(
        self,
        record_number: usize,
        record: EMR_NAMEDESCAPE,
    ) -> Result<Self, PlayError> {
        info!("EMR_NAMEDESCAPE: not implemented");
        Ok(self)
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
        mut self,
        record_number: usize,
        record: EMR_CREATEBRUSHINDIRECT,
    ) -> Result<Self, PlayError> {
        self.emf_object_table.set(
            record.ih_brush as usize,
            GraphicsObject::LogBrushEx(record.log_brush),
        );

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_color_space(
        self,
        record_number: usize,
        record: EMR_CREATECOLORSPACE,
    ) -> Result<Self, PlayError> {
        info!("EMR_CREATECOLORSPACE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_color_space_w(
        self,
        record_number: usize,
        record: EMR_CREATECOLORSPACEW,
    ) -> Result<Self, PlayError> {
        info!("EMR_CREATECOLORSPACEW: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_dib_pattern_brush_pt(
        mut self,
        record_number: usize,
        record: EMR_CREATEDIBPATTERNBRUSHPT,
    ) -> Result<Self, PlayError> {
        self.emf_object_table.set(
            record.ih_brush as usize,
            GraphicsObject::DeviceIndependentBitmap(record.into()),
        );

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_mono_brush(
        self,
        record_number: usize,
        record: EMR_CREATEMONOBRUSH,
    ) -> Result<Self, PlayError> {
        info!("EMR_CREATEMONOBRUSH: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_palette(
        self,
        record_number: usize,
        record: EMR_CREATEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("EMR_CREATEPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn create_pen(
        mut self,
        record_number: usize,
        record: EMR_CREATEPEN,
    ) -> Result<Self, PlayError> {
        self.emf_object_table.set(
            record.ih_pen as usize,
            GraphicsObject::LogPenEx(record.log_pen.into()),
        );

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_create_font_indirect_w(
        mut self,
        record_number: usize,
        record: EMR_EXTCREATEFONTINDIRECTW,
    ) -> Result<Self, PlayError> {
        let font = match record.elw {
            crate::parser::ELW::LogFontExDv(v) => {
                v.first().expect("should be set").clone()
            }
            crate::parser::ELW::LogFontPanose(v) => v.into(),
        };

        self.emf_object_table
            .set(record.ih_fonts as usize, GraphicsObject::LogFontExDv(font));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn ext_create_pen(
        mut self,
        record_number: usize,
        record: EMR_EXTCREATEPEN,
    ) -> Result<Self, PlayError> {
        self.emf_object_table
            .set(record.ih_pen as usize, GraphicsObject::LogPenEx(record.elp));

        Ok(self)
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
        self,
        record_number: usize,
        record: EMR_COLORCORRECTPALETTE,
    ) -> Result<Self, PlayError> {
        info!("EMR_COLORCORRECTPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn delete_color_space(
        self,
        record_number: usize,
        record: EMR_DELETECOLORSPACE,
    ) -> Result<Self, PlayError> {
        info!("EMR_DELETECOLORSPACE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn delete_object(
        mut self,
        record_number: usize,
        record: EMR_DELETEOBJECT,
    ) -> Result<Self, PlayError> {
        self.emf_object_table.delete(record.in_object as usize);
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn resize_palette(
        self,
        record_number: usize,
        record: EMR_RESIZEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("EMR_RESIZEPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_object(
        mut self,
        record_number: usize,
        record: EMR_SELECTOBJECT,
    ) -> Result<Self, PlayError> {
        let emf_object =
            match self.emf_object_table.get(record.in_object as usize) {
                GraphicsObject::Null => {
                    if let Some(stock_object) =
                        StockObject::from_repr(record.in_object)
                    {
                        GraphicsObject::from(
                            &self.selected_emf_object,
                            stock_object,
                        )
                    } else {
                        return Err(PlayError::InvalidRecord {
                            cause: format!(
                                "stock object is not found: index={}",
                                record.in_object,
                            ),
                        });
                    }
                }
                v => v.clone(),
            };

        match emf_object {
            GraphicsObject::DeviceIndependentBitmap(v) => {
                self.selected_emf_object.dib = v.into();
            }
            GraphicsObject::LogBrushEx(v) => {
                self.selected_emf_object.brush = v;
            }
            // GraphicsObject::LogColorSpace(v) => {
            //     self.selected_emf_object.color_space = v.into();
            // }
            // GraphicsObject::LogColorSpaceW(v) => {
            //     self.selected_emf_object.color_space_w = v.into();
            // }
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
                self.selected_emf_object.pen = v;
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

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn select_palette(
        self,
        record_number: usize,
        record: EMR_SELECTPALETTE,
    ) -> Result<Self, PlayError> {
        info!("EMR_SELECTPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_color_space(
        self,
        record_number: usize,
        record: EMR_SETCOLORSPACE,
    ) -> Result<Self, PlayError> {
        info!("EMR_SETCOLORSPACE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_palette_entries(
        self,
        record_number: usize,
        record: EMR_SETPALETTEENTRIES,
    ) -> Result<Self, PlayError> {
        info!("EMR_SETPALETTEENTRIES: not implemented");
        Ok(self)
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
        self,
        record_number: usize,
        record: EMR_GLSBOUNDEDRECORD,
    ) -> Result<Self, PlayError> {
        info!("EMR_GLSBOUNDEDRECORD: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn gls_record(
        self,
        record_number: usize,
        record: EMR_GLSRECORD,
    ) -> Result<Self, PlayError> {
        info!("EMR_GLSRECORD: not implemented");
        Ok(self)
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
    fn abort_path(
        mut self,
        record_number: usize,
        record: EMR_ABORTPATH,
    ) -> Result<Self, PlayError> {
        self.path = Data::new();
        self.context.graphics_environment.drawing.path_bracket = false;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn begin_path(
        mut self,
        record_number: usize,
        record: EMR_BEGINPATH,
    ) -> Result<Self, PlayError> {
        if self.context.graphics_environment.drawing.path_bracket {
            return Err(PlayError::InvalidRecord {
                cause: "Path bracket construction MUST be closed by an \
                        EMR_ABORTPATH or EMR_ENDPATH record."
                    .to_owned(),
            });
        }

        self.context.graphics_environment.drawing.path_bracket = true;
        self.path = Data::new();

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn close_figure(
        mut self,
        record_number: usize,
        record: EMR_CLOSEFIGURE,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.path_bracket = false;
        self.path = self.path.close();

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn end_path(
        mut self,
        record_number: usize,
        record: EMR_ENDPATH,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.path_bracket = false;
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn flatten_path(
        mut self,
        record_number: usize,
        record: EMR_FLATTENPATH,
    ) -> Result<Self, PlayError> {
        if self.path.is_empty() {
            return Ok(self);
        }

        let stroke = Stroke::from(self.selected_emf_object.pen.clone());
        let path = Node::new("path")
            .set("fill", "none")
            .set("d", self.path.to_string());
        let path = stroke.set_props(&self.context, path);

        self.push_element(record_number, path);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn widen_path(
        self,
        record_number: usize,
        record: EMR_WIDENPATH,
    ) -> Result<Self, PlayError> {
        info!("EMR_WIDENPATH: not implemented");
        Ok(self)
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
        self,
        record_number: usize,
        record: EMR_COLORMATCHTOTARGETW,
    ) -> Result<Self, PlayError> {
        info!("EMR_COLORMATCHTOTARGETW: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn force_ufi_mapping(
        mut self,
        record_number: usize,
        record: EMR_FORCEUFIMAPPING,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.text.force_ufi_mapping =
            record.ufi.into();

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn invert_rgn(
        self,
        record_number: usize,
        record: EMR_INVERTRGN,
    ) -> Result<Self, PlayError> {
        info!("EMR_INVERTRGN: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn move_to_ex(
        mut self,
        record_number: usize,
        record: EMR_MOVETOEX,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.current_position =
            record.offset.clone();

        let point = self.context.transform_point_l(&record.offset);

        self.path =
            self.path.clone().move_to(format!("{} {}", point.x, point.y));

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn pixel_format(
        mut self,
        record_number: usize,
        record: EMR_PIXELFORMAT,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.color.pixel_format =
            record.pfd.into();

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn realize_palette(
        self,
        record_number: usize,
        record: EMR_REALIZEPALETTE,
    ) -> Result<Self, PlayError> {
        info!("EMR_REALIZEPALETTE: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn restore_dc(
        mut self,
        record_number: usize,
        record: EMR_RESTOREDC,
    ) -> Result<Self, PlayError> {
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

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn save_dc(
        mut self,
        record_number: usize,
        record: EMR_SAVEDC,
    ) -> Result<Self, PlayError> {
        self.context_stack.push(self.context.clone());

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn scale_viewport_ext_ex(
        mut self,
        record_number: usize,
        record: EMR_SCALEVIEWPORTEXTEX,
    ) -> Result<Self, PlayError> {
        let wmf_core::parser::SizeL { cx, cy } =
            self.context.graphics_environment.regions.viewport.extent;

        self.context.graphics_environment.regions.viewport.extent =
            wmf_core::parser::SizeL {
                cx: ((cx as i32 * record.x_num) / record.x_denom)
                    .unsigned_abs(),
                cy: ((cy as i32 * record.y_num) / record.y_denom)
                    .unsigned_abs(),
            };

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn scale_window_ext_ex(
        mut self,
        record_number: usize,
        record: EMR_SCALEWINDOWEXTEX,
    ) -> Result<Self, PlayError> {
        let wmf_core::parser::SizeL { cx, cy } =
            self.context.graphics_environment.regions.window.extent;

        self.context.graphics_environment.regions.window.extent =
            wmf_core::parser::SizeL {
                cx: ((cx as i32 * record.x_num) / record.x_denom)
                    .unsigned_abs(),
                cy: ((cy as i32 * record.y_num) / record.y_denom)
                    .unsigned_abs(),
            };

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_arc_direction(
        mut self,
        record_number: usize,
        record: EMR_SETARCDIRECTION,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.arc_direction =
            record.arc_direction;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_color(
        mut self,
        record_number: usize,
        record: EMR_SETBKCOLOR,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.background_color =
            record.color;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_bk_mode(
        mut self,
        record_number: usize,
        record: EMR_SETBKMODE,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.background_mode =
            record.background_mode;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_brush_org_ex(
        mut self,
        record_number: usize,
        record: EMR_SETBRUSHORGEX,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.brush_origin = record.origin;
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_color_adjustment(
        mut self,
        record_number: usize,
        record: EMR_SETCOLORADJUSTMENT,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.color.color_adjustment =
            record.color_adjustment;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_icm_mode(
        mut self,
        record_number: usize,
        record: EMR_SETICMMODE,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.color.icm_mode = record.icm_mode;
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_icm_profile_a(
        self,
        record_number: usize,
        record: EMR_SETICMPROFILEA,
    ) -> Result<Self, PlayError> {
        info!("EMR_SETICMPROFILEA: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_icm_profile_w(
        self,
        record_number: usize,
        record: EMR_SETICMPROFILEW,
    ) -> Result<Self, PlayError> {
        info!("EMR_SETICMPROFILEW: not implemented");
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_layout(
        mut self,
        record_number: usize,
        record: EMR_SETLAYOUT,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.layout_mode =
            record.layout_mode;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_linked_ufis(
        mut self,
        record_number: usize,
        record: EMR_SETLINKEDUFIS,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.text.linked_ufis = record.ufis;
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_map_mode(
        mut self,
        record_number: usize,
        record: EMR_SETMAPMODE,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.mapping_mode =
            record.map_mode;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_mapper_flags(
        mut self,
        record_number: usize,
        record: EMR_SETMAPPERFLAGS,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.text.font_mapper_flags = record.flags;
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_miter_limit(
        mut self,
        record_number: usize,
        record: EMR_SETMITERLIMIT,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.miter_limit =
            record.miter_limit.into();

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_polyfill_mode(
        mut self,
        record_number: usize,
        record: EMR_SETPOLYFILLMODE,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.polyfill_mode =
            record.polygon_fill_mode;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_rop2(
        mut self,
        record_number: usize,
        record: EMR_SETROP2,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.rop2 = record.rop2_mode;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_stretch_blt_mode(
        mut self,
        record_number: usize,
        record: EMR_SETSTRETCHBLTMODE,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.stretch_blt_mode =
            record.stretch_mode;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_align(
        mut self,
        record_number: usize,
        record: EMR_SETTEXTALIGN,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.text.text_alignment =
            record.text_alignment_mode;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_color(
        mut self,
        record_number: usize,
        record: EMR_SETTEXTCOLOR,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.drawing.text_color = record.color;
        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_text_justification(
        mut self,
        record_number: usize,
        record: EMR_SETTEXTJUSTIFICATION,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.text.text_justification =
            (record.n_break_extra, record.n_break_count);

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_viewport_ext_ex(
        mut self,
        record_number: usize,
        record: EMR_SETVIEWPORTEXTEX,
    ) -> Result<Self, PlayError> {
        if matches!(
            self.context.graphics_environment.drawing.mapping_mode,
            MapMode::MM_ISOTROPIC | MapMode::MM_ANISOTROPIC
        ) {
            self.context.graphics_environment.regions.viewport.extent =
                record.extent;

            self.context.apply_transformation();
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_viewport_org_ex(
        mut self,
        record_number: usize,
        record: EMR_SETVIEWPORTORGEX,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.regions.viewport.origin =
            record.origin;

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_window_ext_ex(
        mut self,
        record_number: usize,
        record: EMR_SETWINDOWEXTEX,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.regions.window.extent =
            record.extent.clone();

        if matches!(
            self.context.graphics_environment.drawing.mapping_mode,
            MapMode::MM_ISOTROPIC | MapMode::MM_ANISOTROPIC
        ) {
            self.context.apply_transformation();
        } else {
            self.window.extent = record.extent;
        }

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_window_org_ex(
        mut self,
        record_number: usize,
        record: EMR_SETWINDOWORGEX,
    ) -> Result<Self, PlayError> {
        self.context.graphics_environment.regions.window.origin =
            record.origin.clone();

        if !matches!(
            self.context.graphics_environment.drawing.mapping_mode,
            MapMode::MM_ISOTROPIC | MapMode::MM_ANISOTROPIC
        ) {
            self.window.origin = record.origin;
        }

        Ok(self)
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
        mut self,
        record_number: usize,
        record: EMR_MODIFYWORLDTRANSFORM,
    ) -> Result<Self, PlayError> {
        let (a, b) = match record.modify_world_transform_mode {
            ModifyWorldTransformMode::MWT_IDENTITY => {
                // NOOP
                return Ok(self);
            }
            ModifyWorldTransformMode::MWT_LEFTMULTIPLY => {
                (record.x_form, self.context.xform.clone())
            }
            ModifyWorldTransformMode::MWT_RIGHTMULTIPLY => {
                (self.context.xform.clone(), record.x_form)
            }
            ModifyWorldTransformMode::MWT_SET => {
                self.context.xform = record.x_form;
                return Ok(self);
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

        Ok(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip(self),
        err(level = tracing::Level::ERROR, Display),
    ))]
    fn set_world_transform(
        mut self,
        record_number: usize,
        record: EMR_SETWORLDTRANSFORM,
    ) -> Result<Self, PlayError> {
        self.context.xform = record.x_form;

        Ok(self)
    }
}
