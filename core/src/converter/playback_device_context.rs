use crate::imports::*;

#[derive(Clone, Debug)]
pub struct PlaybackDeviceContext {
    pub graphics_environment: GraphicsEnvironment,
    pub xform: crate::parser::XForm,
}

impl Default for PlaybackDeviceContext {
    fn default() -> Self {
        Self {
            graphics_environment: GraphicsEnvironment::default(),
            xform: crate::parser::XForm::default(),
        }
    }
}

impl PlaybackDeviceContext {
    pub fn apply_transformation(&mut self) {
        let wmf_core::parser::SizeL { cx: ve_cx, cy: ve_cy } =
            &self.graphics_environment.regions.viewport.extent;
        let wmf_core::parser::SizeL { cx: we_cx, cy: we_cy } =
            &self.graphics_environment.regions.window.extent;
        let (sx, sy) =
            (*ve_cx as f32 / *we_cx as f32, *ve_cy as f32 / *we_cy as f32);

        self.set_scale(sx, sy);
    }

    pub fn set_scale(&mut self, sx: f32, sy: f32) {
        self.xform = crate::parser::XForm {
            m11: 1.0 * sx,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0 * sy,
            dx: 0.0,
            dy: 0.0,
        };
    }

    pub fn transform_point_l(
        &self,
        p: wmf_core::parser::PointL,
    ) -> wmf_core::parser::PointL {
        wmf_core::parser::PointL {
            x: (f64::from(self.xform.m11) * f64::from(p.x)
                + f64::from(self.xform.m21) * f64::from(p.y)
                + f64::from(self.xform.dx)) as i32,
            y: (f64::from(self.xform.m12) * f64::from(p.x)
                + f64::from(self.xform.m22) * f64::from(p.y)
                + f64::from(self.xform.dy)) as i32,
        }
    }

    pub fn transform_point_s(
        &self,
        p: wmf_core::parser::PointS,
    ) -> wmf_core::parser::PointS {
        wmf_core::parser::PointS {
            x: (self.xform.m11 * f32::from(p.x)
                + self.xform.m21 * f32::from(p.y)
                + self.xform.dx) as i16,
            y: (self.xform.m12 * f32::from(p.x)
                + self.xform.m22 * f32::from(p.y)
                + self.xform.dy) as i16,
        }
    }
}

pub fn point_s_to_point_l(
    v: &wmf_core::parser::PointS,
) -> wmf_core::parser::PointL {
    wmf_core::parser::PointL { x: v.x.into(), y: v.y.into() }
}

/// The EMF object table is an element of the state maintained during EMF
/// metafile playback. It contains data used for managing graphics objects as
/// they are created, activated, used, deactivated, and deleted by the
/// processing of EMF records.
///
/// EMFObject (variable): A graphics object and its associated index.
///
/// When a graphics object is created by an object creation record, the record
/// specifies a numerical index. The object can be referenced by its index
/// during metafile processing until the object is deleted. Object indexes start
/// at 1; zero is reserved for references to the metafile itself.
///
/// An object manipulation record can use the index of a graphics object to
/// select it into the playback device context. This has the effect of
/// activating the object so that it can be used in graphics operations
/// specified by subsequent metafile records. Until the object is activated, it
/// is not used. Later, if a different object of the same type is activated, the
/// former object is deactivated but not deleted. An object is not deleted until
/// an object manipulation record is processed that deletes it.
///
/// Before a graphics object is instantiated and activated, a default stock
/// object for that type is used in graphics operations.
#[derive(Clone, Debug)]
pub struct EmfObjectTable(Vec<GraphicsObject>);

impl EmfObjectTable {
    pub fn new(v: usize) -> Self {
        let mut objects = vec![GraphicsObject::Null; v + 1];
        objects[0] = GraphicsObject::ReferenceSelf;

        Self(objects)
    }

    pub fn delete(&mut self, i: usize) {
        self.0[i] = GraphicsObject::Null;
    }

    pub fn get(&self, i: usize) -> &GraphicsObject {
        self.0.get(i).expect("should be set")
    }

    pub fn set(&mut self, idx: usize, g: GraphicsObject) {
        self.0[idx] = g;
    }
}

#[derive(Clone, Debug)]
pub struct SelectedObject {
    pub dib: Option<wmf_core::parser::DeviceIndependentBitmap>,
    pub brush: crate::parser::LogBrushEx,
    pub color_space: Option<wmf_core::parser::LogColorSpace>,
    pub color_space_w: Option<wmf_core::parser::LogColorSpaceW>,
    pub font: Option<crate::parser::LogFont>,
    pub font_ex_dv: Option<crate::parser::LogFontExDv>,
    pub palette: Option<crate::parser::LogPalette>,
    pub pen: crate::parser::LogPenEx,
}

impl Default for SelectedObject {
    fn default() -> Self {
        Self {
            dib: None,
            brush: crate::parser::LogBrushEx::black_brush(),
            color_space: None,
            color_space_w: None,
            font: None,
            font_ex_dv: None,
            palette: None,
            pen: crate::parser::LogPenEx::black_pen(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum GraphicsObject {
    DeviceIndependentBitmap(wmf_core::parser::DeviceIndependentBitmap),
    LogBrushEx(crate::parser::LogBrushEx),
    LogColorSpace(wmf_core::parser::LogColorSpace),
    LogColorSpaceW(wmf_core::parser::LogColorSpaceW),
    LogFont(crate::parser::LogFont),
    LogFontExDv(crate::parser::LogFontExDv),
    LogPalette(crate::parser::LogPalette),
    LogPenEx(crate::parser::LogPenEx),
    ReferenceSelf,
    Null,
}

impl GraphicsObject {
    pub fn from(
        selected_object: &SelectedObject,
        v: crate::parser::StockObject,
    ) -> Self {
        use crate::parser::StockObject::*;

        match v {
            WHITE_BRUSH => {
                Self::LogBrushEx(crate::parser::LogBrushEx::white_brush())
            }
            LTGRAY_BRUSH => {
                Self::LogBrushEx(crate::parser::LogBrushEx::Solid {
                    color: wmf_core::parser::ColorRef {
                        red: 0xC0,
                        green: 0xC0,
                        blue: 0xC0,
                        reserved: 0x00,
                    },
                })
            }
            GRAY_BRUSH => Self::LogBrushEx(crate::parser::LogBrushEx::Solid {
                color: wmf_core::parser::ColorRef {
                    red: 0x80,
                    green: 0x80,
                    blue: 0x80,
                    reserved: 0x00,
                },
            }),
            DKGRAY_BRUSH => {
                Self::LogBrushEx(crate::parser::LogBrushEx::Solid {
                    color: wmf_core::parser::ColorRef {
                        red: 0x40,
                        green: 0x40,
                        blue: 0x40,
                        reserved: 0x00,
                    },
                })
            }
            BLACK_BRUSH => {
                Self::LogBrushEx(crate::parser::LogBrushEx::black_brush())
            }
            NULL_BRUSH => Self::LogBrushEx(crate::parser::LogBrushEx::Null),
            WHITE_PEN => Self::LogPenEx(crate::parser::LogPenEx::white_pen()),
            BLACK_PEN => Self::LogPenEx(crate::parser::LogPenEx::black_pen()),
            NULL_PEN => Self::LogPenEx(crate::parser::LogPenEx::null_pen()),
            OEM_FIXED_FONT => Self::LogFont(crate::parser::LogFont {
                height: 0,
                width: 0,
                escapement: 0,
                orientation: 0,
                weight: 400,
                italic: false,
                underline: false,
                strike_out: false,
                charset: wmf_core::parser::CharacterSet::OEM_CHARSET,
                out_precision:
                    wmf_core::parser::OutPrecision::OUT_DEFAULT_PRECIS,
                clip_precision: BTreeSet::new(),
                quality: wmf_core::parser::FontQuality::DEFAULT_QUALITY,
                pitch_and_family: wmf_core::parser::PitchAndFamily {
                    family: wmf_core::parser::FamilyFont::FF_DONTCARE,
                    pitch: wmf_core::parser::PitchFont::FIXED_PITCH,
                },
                facename: "none".to_owned(),
            }),
            ANSI_FIXED_FONT => Self::LogFont(crate::parser::LogFont {
                height: 0,
                width: 0,
                escapement: 0,
                orientation: 0,
                weight: 400,
                italic: false,
                underline: false,
                strike_out: false,
                charset: wmf_core::parser::CharacterSet::ANSI_CHARSET,
                out_precision:
                    wmf_core::parser::OutPrecision::OUT_DEFAULT_PRECIS,
                clip_precision: BTreeSet::new(),
                quality: wmf_core::parser::FontQuality::DEFAULT_QUALITY,
                pitch_and_family: wmf_core::parser::PitchAndFamily {
                    family: wmf_core::parser::FamilyFont::FF_DONTCARE,
                    pitch: wmf_core::parser::PitchFont::FIXED_PITCH,
                },
                facename: "none".to_owned(),
            }),
            ANSI_VAR_FONT => Self::LogFont(crate::parser::LogFont {
                height: 0,
                width: 0,
                escapement: 0,
                orientation: 0,
                weight: 400,
                italic: false,
                underline: false,
                strike_out: false,
                charset: wmf_core::parser::CharacterSet::ANSI_CHARSET,
                out_precision:
                    wmf_core::parser::OutPrecision::OUT_DEFAULT_PRECIS,
                clip_precision: BTreeSet::new(),
                quality: wmf_core::parser::FontQuality::DEFAULT_QUALITY,
                pitch_and_family: wmf_core::parser::PitchAndFamily {
                    family: wmf_core::parser::FamilyFont::FF_DONTCARE,
                    pitch: wmf_core::parser::PitchFont::VARIABLE_PITCH,
                },
                facename: "none".to_owned(),
            }),
            DEFAULT_GUI_FONT | SYSTEM_FONT => {
                Self::LogFont(crate::parser::LogFont {
                    height: 0,
                    width: 0,
                    escapement: 0,
                    orientation: 0,
                    weight: 400,
                    italic: false,
                    underline: false,
                    strike_out: false,
                    charset: wmf_core::parser::CharacterSet::ANSI_CHARSET,
                    out_precision:
                        wmf_core::parser::OutPrecision::OUT_DEFAULT_PRECIS,
                    clip_precision: BTreeSet::new(),
                    quality: wmf_core::parser::FontQuality::DEFAULT_QUALITY,
                    pitch_and_family: wmf_core::parser::PitchAndFamily {
                        family: wmf_core::parser::FamilyFont::FF_DONTCARE,
                        pitch: wmf_core::parser::PitchFont::FIXED_PITCH,
                    },
                    facename: "Tahoma".to_owned(),
                })
            }
            DEVICE_DEFAULT_FONT => Self::LogFont(crate::parser::LogFont {
                height: 0,
                width: 0,
                escapement: 0,
                orientation: 0,
                weight: 400,
                italic: false,
                underline: false,
                strike_out: false,
                charset: wmf_core::parser::CharacterSet::ANSI_CHARSET,
                out_precision:
                    wmf_core::parser::OutPrecision::OUT_DEFAULT_PRECIS,
                clip_precision: BTreeSet::new(),
                quality: wmf_core::parser::FontQuality::DEFAULT_QUALITY,
                pitch_and_family: wmf_core::parser::PitchAndFamily {
                    family: wmf_core::parser::FamilyFont::FF_DONTCARE,
                    pitch: wmf_core::parser::PitchFont::FIXED_PITCH,
                },
                facename: "Tahoma".to_owned(),
            }),
            DEFAULT_PALETTE => Self::LogPalette(crate::parser::LogPalette {
                version: 0x0300,
                number_of_entries: 0,
                palette_entries: vec![],
            }),
            SYSTEM_FIXED_FONT => Self::LogFont(crate::parser::LogFont {
                height: 0,
                width: 0,
                escapement: 0,
                orientation: 0,
                weight: 400,
                italic: false,
                underline: false,
                strike_out: false,
                charset: wmf_core::parser::CharacterSet::ANSI_CHARSET,
                out_precision:
                    wmf_core::parser::OutPrecision::OUT_DEFAULT_PRECIS,
                clip_precision: BTreeSet::new(),
                quality: wmf_core::parser::FontQuality::DEFAULT_QUALITY,
                pitch_and_family: wmf_core::parser::PitchAndFamily {
                    family: wmf_core::parser::FamilyFont::FF_DONTCARE,
                    pitch: wmf_core::parser::PitchFont::FIXED_PITCH,
                },
                facename: "serif".to_owned(),
            }),
            DC_BRUSH => Self::LogBrushEx(selected_object.brush.clone()),
            DC_PEN => Self::LogPenEx(selected_object.pen.clone()),
        }
    }
}

/// PlaybackState: The playback device context at some point in EMF record
/// processing, including region definitions, color profiles, fonts and text
/// properties, and graphics drawing metadata. The elements of the PlaybackState
/// are grouped as shown in the following table.
#[derive(Clone, Debug)]
pub struct GraphicsEnvironment {
    pub regions: PlaybackStateRegions,
    pub color: PlaybackStateColors,
    pub text: PlaybackStateText,
    pub drawing: PlaybackStateDrawing,
}

impl Default for GraphicsEnvironment {
    fn default() -> Self {
        let (extent, origin) = (
            wmf_core::parser::SizeL { cx: 1000, cy: 1000 },
            wmf_core::parser::PointL { x: 0, y: 0 },
        );

        Self {
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
        }
    }
}

/// The Regions group of elements control the output area and clipping
/// properties in the playback device context.
#[derive(Clone, Debug)]
pub struct PlaybackStateRegions {
    /// Clipping: The current clipping region, which with MetaClipping defines
    /// the bounds of the drawing area. The default value for the Clipping
    /// element is implementation-specific.
    pub clipping: Option<crate::parser::RegionData>,
    /// MetaClipping: The current metaregion, which with the Clipping region
    /// defines the bounds of the drawing area.
    pub meta_clipping: Option<crate::parser::RegionData>,
    /// Viewport: A rectangular drawing area using coordinates in the device
    /// space.
    pub viewport: Viewport,
    /// Window: A rectangular drawing area using the coordinates of the page
    /// space.
    pub window: Window,
}

/// Viewport: A rectangular drawing area using coordinates in the device space.
#[derive(Clone, Debug)]
pub struct Viewport {
    /// Extent: Horizontal and vertical sizes of the drawing area in device
    /// units.
    pub extent: wmf_core::parser::SizeL,
    /// Origin: Point value of the origin of the drawing area in device units.
    pub origin: wmf_core::parser::PointL,
}

/// Window: A rectangular drawing area using the coordinates of the page space.
#[derive(Clone, Debug)]
pub struct Window {
    /// Extent: Horizontal and vertical sizes of the drawing area in logical
    /// units.
    pub extent: wmf_core::parser::SizeL,
    /// Origin: Point value of the origin of the drawing area in logical units.
    pub origin: wmf_core::parser::PointL,
}

/// The Colors group of elements define the current state of color management in
/// the playback device context.
#[derive(Clone, Debug)]
pub struct PlaybackStateColors {
    pub color_adjustment: crate::parser::ColorAdjustment,
    /// Data field in `EMR_COLORMATCHTOTARGETW`, `EMR_SETICMPROFILEA`,
    /// `EMR_SETICMPROFILEW`.
    pub color_profile: Vec<u8>,
    pub color_profile_embedded: bool,
    pub color_proofing: u32,
    pub color_transform: Vec<u8>,
    pub icm_mode: crate::parser::ICMMode,
    pub pixel_format: Option<crate::parser::PixelFormatDescriptor>,
}

impl Default for PlaybackStateColors {
    fn default() -> Self {
        Self {
            color_adjustment: crate::parser::ColorAdjustment::default(),
            color_profile: vec![],
            color_profile_embedded: false,
            color_proofing: 0,
            color_transform: vec![],
            icm_mode: crate::parser::ICMMode::ICM_DONE_OUTSIDEDC,
            pixel_format: None,
        }
    }
}

/// The Text group of elements define the current font and text properties in
/// the playback device context.
///
/// Text elements are used by the following EMF text drawing records.
///
/// - EMR_EXTTEXTOUTA
/// - EMR_EXTTEXTOUTW
/// - EMR_POLYTEXTOUTA
/// - EMR_POLYTEXTOUTW
/// - EMR_SMALLTEXTOUT
#[derive(Clone, Debug)]
pub struct PlaybackStateText {
    pub font_mapper_flags: u32,
    pub force_ufi_mapping: Option<crate::parser::UniversalFontId>,
    pub linked_ufis: Vec<crate::parser::UniversalFontId>,
    pub text_alignment: u32,
    pub text_justification: (i32, i32),
}

impl Default for PlaybackStateText {
    fn default() -> Self {
        Self {
            font_mapper_flags: 0,
            force_ufi_mapping: None,
            linked_ufis: vec![],
            text_alignment: 0,
            text_justification: (0, 0),
        }
    }
}

/// The Drawing group of elements define various graphics flags and other
/// metadata values that affect how the image in the EMF metafile is rendered.
#[derive(Clone, Debug)]
pub struct PlaybackStateDrawing {
    /// ArcDirection: The drawing direction for arcs and rectangles, from the
    /// ArcDirection enumeration.
    pub arc_direction: crate::parser::ArcDirection,
    /// BackgroundColor: The color used as background for drawing text, hatched
    /// brushes, and pen styles that are not solid lines, depending on the
    /// BackgroundMode value.
    pub background_color: wmf_core::parser::ColorRef,
    /// BackgroundMode: How to combine the drawing background with the
    /// BackgroundColor value, from the BackgroundMode enumeration.
    pub background_mode: crate::parser::BackgroundMode,
    /// BrushOrigin: The horizontal and vertical origin of the current brush in
    /// logical units, which is used as needed to maintain an alignment of
    /// patterns on the display surface.
    pub brush_origin: wmf_core::parser::PointL,
    pub current_position: wmf_core::parser::PointL,
    pub layout_mode: crate::parser::LayoutMode,
    pub line_cap: u32,
    pub line_join: u32,
    pub mapping_mode: crate::parser::MapMode,
    pub miter_limit: Option<u32>,
    pub path_bracket: bool,
    pub polyfill_mode: crate::parser::PolygonFillMode,
    pub rop2: wmf_core::parser::BinaryRasterOperation,
    pub stretch_blt_mode: crate::parser::StretchMode,
    pub text_color: wmf_core::parser::ColorRef,
}

impl Default for PlaybackStateDrawing {
    fn default() -> Self {
        Self {
            arc_direction: crate::parser::ArcDirection::AD_COUNTERCLOCKWISE,
            background_color: wmf_core::parser::ColorRef::white(),
            background_mode: crate::parser::BackgroundMode::TRANSPARENT,
            brush_origin: wmf_core::parser::PointL { x: 0, y: 0 },
            current_position: wmf_core::parser::PointL { x: 0, y: 0 },
            layout_mode: crate::parser::LayoutMode::LAYOUT_LTR,
            line_cap: 0,
            line_join: 0,
            mapping_mode: crate::parser::MapMode::MM_TEXT,
            miter_limit: None,
            path_bracket: false,
            polyfill_mode: crate::parser::PolygonFillMode::ALTERNATE,
            rop2: wmf_core::parser::BinaryRasterOperation::R2_BLACK,
            stretch_blt_mode: crate::parser::StretchMode::STRETCH_ANDSCANS,
            text_color: wmf_core::parser::ColorRef::black(),
        }
    }
}
