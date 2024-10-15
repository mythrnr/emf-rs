#[derive(Clone, Debug)]
pub struct PlaybackDeviceContext {
    emf_object_table: EmfObjectTable,
    graphics_environment: GraphicsEnvironment,
}

impl PlaybackDeviceContext {
    pub fn new(cap: usize) -> Self {
        Self {
            emf_object_table: EmfObjectTable::new(cap + 1),
            graphics_environment: GraphicsEnvironment::new(),
        }
    }
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
        let mut objects = vec![GraphicsObject::Null; v];
        objects[0] = GraphicsObject::ReferenceSelf;

        Self(objects)
    }

    pub fn delete(&mut self, i: usize) {
        self.0[i] = GraphicsObject::Null;
    }

    pub fn get(&self, i: usize) -> &GraphicsObject {
        self.0.get(i).expect("should be set")
    }

    pub fn push(&mut self, g: GraphicsObject) {
        for (i, v) in self.0.iter_mut().enumerate() {
            if matches!(&v, GraphicsObject::Null) {
                self.0[i] = g;
                break;
            }
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

/// The Graphics Environment describes the graphics state maintained during EMF
/// metafile playback. A possible implementation is described by the following
/// elements.
#[derive(Clone, Debug)]
pub struct GraphicsEnvironment(Vec<PlaybackState>);

impl GraphicsEnvironment {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn delete(&mut self, i: usize) {
        self.0.remove(i);
    }

    pub fn get(&self, i: usize) -> &PlaybackState {
        self.0.get(i).expect("should be set")
    }

    pub fn push(&mut self, s: PlaybackState) {
        self.0.push(s);
    }
}

/// PlaybackState: The playback device context at some point in EMF record
/// processing, including region definitions, color profiles, fonts and text
/// properties, and graphics drawing metadata. The elements of the PlaybackState
/// are grouped as shown in the following table.
#[derive(Clone, Debug)]
pub struct PlaybackState {
    pub regions: PlaybackStateRegions,
    pub color: PlaybackStateColors,
    pub text: PlaybackStateText,
    pub drawing: PlaybackStateDrawing,
}

/// The Regions group of elements control the output area and clipping
/// properties in the playback device context.
#[derive(Clone, Debug)]
pub struct PlaybackStateRegions {
    /// Clipping: The current clipping region, which with MetaClipping defines
    /// the bounds of the drawing area. The default value for the Clipping
    /// element is implementation-specific.
    pub clipping: crate::parser::RegionData,
    /// MetaClipping: The current metaregion, which with the Clipping region
    /// defines the bounds of the drawing area.
    pub meta_clipping: crate::parser::RegionData,
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
    pub pixel_format: crate::parser::PixelFormatDescriptor,
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
    pub force_ufi_mapping: crate::parser::UniversalFontId,
    pub linked_ufis: Vec<crate::parser::UniversalFontId>,
    pub text_alignment: u32,
    pub text_justification: (i32, i32),
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
    pub miter_limit: u32,
    pub path: Vec<wmf_core::parser::PointL>,
    pub path_bracket: bool,
    pub polyfill_mode: crate::parser::PolygonFillMode,
    pub rop2: wmf_core::parser::BinaryRasterOperation,
    pub stretch_blt_mode: crate::parser::StretchMode,
    pub text_color: wmf_core::parser::ColorRef,
}
