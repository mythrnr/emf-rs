/// The RecordType enumeration defines values that uniquely identify records in
/// an EMF metafile. These values are specified in the Type fields of EMF
/// records
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    strum::FromRepr,
    strum::EnumIter,
)]
#[repr(u32)]
pub enum RecordType {
    /// This record defines the start of the metafile and specifies its
    /// characteristics; its contents, including the dimensions of the embedded
    /// image; the number of records in the metafile; and the resolution of the
    /// device on which the embedded image was created. These values make it
    /// possible for the metafile to be device-independent.
    EMR_HEADER = 0x00000001,
    /// This record defines one or more Bezier curves. Cubic Bezier curves are
    /// defined using specified endpoints and control points, and are stroked
    /// with the current pen.
    EMR_POLYBEZIER = 0x00000002,
    /// This record defines a polygon consisting of two or more vertexes
    /// connected by straight lines. The polygon is outlined by using the
    /// current pen and filled by using the current brush and polygon fill
    /// mode. The polygon is closed automatically by drawing a line from the
    /// last vertex to the first.
    EMR_POLYGON = 0x00000003,
    /// This record defines a series of line segments by connecting the points
    /// in the specified array.
    EMR_POLYLINE = 0x00000004,
    /// This record defines one or more Bezier curves based upon the current
    /// drawing position.
    EMR_POLYBEZIERTO = 0x00000005,
    /// This record defines one or more straight lines based upon the current
    /// drawing position. A line is drawn from the current drawing position to
    /// the first point specified by the points field by using the current pen.
    /// For each additional line, drawing is performed from the ending point of
    /// the previous line to the next point specified by points.
    EMR_POLYLINETO = 0x00000006,
    /// This record defines multiple series of connected line segments. The
    /// line segments are drawn by using the current pen. The figures formed by
    /// the segments are not filled. The current position is neither used nor
    /// updated by this record.
    EMR_POLYPOLYLINE = 0x00000007,
    /// This record defines a series of closed polygons. Each polygon is
    /// outlined by using the current pen and filled by using the current brush
    /// and polygon fill mode. The polygons defined by this record can overlap.
    EMR_POLYPOLYGON = 0x00000008,
    /// This record defines the window extent.
    EMR_SETWINDOWEXTEX = 0x00000009,
    /// This record defines the window origin.
    EMR_SETWINDOWORGEX = 0x0000000A,
    /// This record defines the viewport extent.
    EMR_SETVIEWPORTEXTEX = 0x0000000B,
    /// This record defines the viewport origin.
    EMR_SETVIEWPORTORGEX = 0x0000000C,
    /// This record defines the origin of the current brush.
    EMR_SETBRUSHORGEX = 0x0000000D,
    /// This record indicates the end of the metafile.
    EMR_EOF = 0x0000000E,
    /// This record defines the color of the pixel at the specified logical
    /// coordinates.
    EMR_SETPIXELV = 0x0000000F,
    /// This record specifies parameters for the process of matching logical
    /// fonts to physical fonts, which is performed by the font mapper.
    EMR_SETMAPPERFLAGS = 0x00000010,
    /// This record defines the mapping mode, which defines the unit of measure
    /// used to transform page space units into device space units, and defines
    /// the orientation of the device's X and Y axes.
    EMR_SETMAPMODE = 0x00000011,
    /// This record defines the background mix mode, which is used with text,
    /// hatched brushes, and pen styles that are not solid lines.
    EMR_SETBKMODE = 0x00000012,
    /// This record defines polygon fill mode.
    EMR_SETPOLYFILLMODE = 0x00000013,
    /// This record defines binary raster operation mode.
    EMR_SETROP2 = 0x00000014,
    /// This record defines bitmap stretch mode.
    EMR_SETSTRETCHBLTMODE = 0x00000015,
    /// This record defines text alignment.
    EMR_SETTEXTALIGN = 0x00000016,
    /// This record defines the color adjustment values using the specified
    /// values.
    EMR_SETCOLORADJUSTMENT = 0x00000017,
    /// This record defines the current text color.
    EMR_SETTEXTCOLOR = 0x00000018,
    /// This record defines the background color.
    EMR_SETBKCOLOR = 0x00000019,
    /// This record redefines the current clipping region by the specified
    /// offsets.
    EMR_OFFSETCLIPRGN = 0x0000001A,
    /// This record defines coordinates of the new drawing position in logical
    /// units.
    EMR_MOVETOEX = 0x0000001B,
    /// This record intersects the current clipping region with the current
    /// metaregion and saves the combined region as the new current metaregion.
    EMR_SETMETARGN = 0x0000001C,
    /// This record defines a new clipping region that consists of the current
    /// clipping region intersected with the specified rectangle.
    EMR_EXCLUDECLIPRECT = 0x0000001D,
    /// This record defines a new clipping region from the intersection of the
    /// current clipping region and the specified rectangle.
    EMR_INTERSECTCLIPRECT = 0x0000001E,
    /// This record redefines the viewport using the ratios formed by the
    /// specified multiplicands and divisors.
    EMR_SCALEVIEWPORTEXTEX = 0x0000001F,
    /// This record redefines the window using the ratios formed by the
    /// specified multiplicands and divisors.
    EMR_SCALEWINDOWEXTEX = 0x00000020,
    /// This record saves the current state of the playback device context in
    /// an array of states saved by preceding EMR_SAVEDC records if any.
    EMR_SAVEDC = 0x00000021,
    /// This record restores the playback device context to the specified
    /// state, which was saved by a preceding EMR_SAVEDC record.
    EMR_RESTOREDC = 0x00000022,
    /// This record defines a two-dimensional linear transform between world
    /// space and page space [MSDN-WRLDPGSPC].
    EMR_SETWORLDTRANSFORM = 0x00000023,
    /// This record redefines the world transform by using the specified mode.
    EMR_MODIFYWORLDTRANSFORM = 0x00000024,
    /// This record selects an object in the playback device context, which is
    /// identified by its index in the EMF object table.
    EMR_SELECTOBJECT = 0x00000025,
    /// This record defines a logical pen that has the specified style, width,
    /// and color.
    EMR_CREATEPEN = 0x00000026,
    /// This record defines a logical brush for filling figures in graphics
    /// operations.
    EMR_CREATEBRUSHINDIRECT = 0x00000027,
    /// This record deletes a graphics object, clearing its index in the EMF
    /// object table.
    EMR_DELETEOBJECT = 0x00000028,
    /// This record defines a line segment of an arc. The line segment is drawn
    /// from the current drawing position to the beginning of the arc. The arc
    /// is drawn along the perimeter of a circle with the given radius and
    /// center. The length of the arc is defined by the given start and sweep
    /// angles.
    EMR_ANGLEARC = 0x00000029,
    /// This record defines an ellipse. The center of the ellipse is the center
    /// of the specified bounding rectangle. The ellipse is outlined by using
    /// the current pen and is filled by using the current brush.
    EMR_ELLIPSE = 0x0000002A,
    /// This record defines a rectangle. The rectangle is outlined by using the
    /// current pen and filled by using the current brush.
    EMR_RECTANGLE = 0x0000002B,
    /// This record defines a rectangle with rounded corners. The rectangle is
    /// outlined by using the current pen and filled by using the current
    /// brush.
    EMR_ROUNDRECT = 0x0000002C,
    /// This record defines an elliptical arc.
    EMR_ARC = 0x0000002D,
    /// This record defines a chord, which is a region bounded by the
    /// intersection of an ellipse and a line segment, called a secant. The
    /// chord is outlined by using the current pen and filled by using the
    /// current brush.
    EMR_CHORD = 0x0000002E,
    /// This record defines a pie-shaped wedge bounded by the intersection of
    /// an ellipse and two radials. The pie is outlined by using the current
    /// pen and filled by using the current brush.
    EMR_PIE = 0x0000002F,
    /// This record selects a LogPalette object into the playback device
    /// context, identifying it by its index in the EMF object table.
    EMR_SELECTPALETTE = 0x00000030,
    /// This record defines a LogPalette object.
    EMR_CREATEPALETTE = 0x00000031,
    /// This record defines RGB color values in a range of entries in a
    /// LogPalette object.
    EMR_SETPALETTEENTRIES = 0x00000032,
    /// This record increases or decreases the size of a logical palette.
    EMR_RESIZEPALETTE = 0x00000033,
    /// This record maps entries from the current logical palette to the system
    /// palette.
    EMR_REALIZEPALETTE = 0x00000034,
    /// This record fills an area of the display surface with the current
    /// brush.
    EMR_EXTFLOODFILL = 0x00000035,
    /// This record defines a line from the current drawing position up to, but
    /// not including, the specified point. It resets the current drawing
    /// position to the specified point.
    EMR_LINETO = 0x00000036,
    /// This record defines an elliptical arc. It resets the current position
    /// to the endpoint of the arc.
    EMR_ARCTO = 0x00000037,
    /// This record defines a set of line segments and Bezier curves.
    EMR_POLYDRAW = 0x00000038,
    /// This record defines the drawing direction to be used for arc and
    /// rectangle operations.
    EMR_SETARCDIRECTION = 0x00000039,
    /// This record defines the limit for the length of miter joins.
    EMR_SETMITERLIMIT = 0x0000003A,
    /// This record opens a path bracket for specifying the current path.
    EMR_BEGINPATH = 0x0000003B,
    /// This record closes an open path bracket and selects the path into the
    /// playback device context.
    EMR_ENDPATH = 0x0000003C,
    /// This record closes an open figure in a path.
    EMR_CLOSEFIGURE = 0x0000003D,
    /// This record closes any open figures in the current path bracket and
    /// fills its interior by using the current brush and polygon-filling mode.
    EMR_FILLPATH = 0x0000003E,
    /// This record closes any open figures in a path, strokes the outline of
    /// the path by using the current pen, and fills its interior by using the
    /// current brush.
    EMR_STROKEANDFILLPATH = 0x0000003F,
    /// This record renders the specified path by using the current pen.
    EMR_STROKEPATH = 0x00000040,
    /// This record turns each curve in the path into a sequence of lines.
    EMR_FLATTENPATH = 0x00000041,
    /// This record redefines the current path bracket as the area that would
    /// be painted if the path were stroked using the current pen.
    EMR_WIDENPATH = 0x00000042,
    /// This record specifies a clipping region as the current clipping region
    /// combined with the current path bracket, using the specified mode.
    EMR_SELECTCLIPPATH = 0x00000043,
    /// This record aborts a path bracket or discards the path from a closed
    /// path bracket.
    EMR_ABORTPATH = 0x00000044,
    /// This record specifies arbitrary private data.
    EMR_COMMENT = 0x00000046,
    /// This record fills the specified region by using the specified brush.
    EMR_FILLRGN = 0x00000047,
    /// This record draws a border around the specified region using the
    /// specified brush.
    EMR_FRAMERGN = 0x00000048,
    /// This record inverts the colors in the specified region.
    EMR_INVERTRGN = 0x00000049,
    /// This record paints the specified region by using the current brush.
    EMR_PAINTRGN = 0x0000004A,
    /// This record combines the specified region with the current clipping
    /// region, using the specified mode.
    EMR_EXTSELECTCLIPRGN = 0x0000004B,
    /// This record specifies a block transfer of pixels from a source bitmap
    /// to a destination rectangle, optionally in combination with a brush
    /// pattern, according to a specified raster operation.
    EMR_BITBLT = 0x0000004C,
    /// This record specifies a block transfer of pixels from a source bitmap
    /// to a destination rectangle, optionally in combination with a brush
    /// pattern, according to a specified raster operation, stretching or
    /// compressing the output to fit the dimensions of the destination, if
    /// necessary.
    EMR_STRETCHBLT = 0x0000004D,
    /// This record specifies a block transfer of pixels from a source bitmap
    /// to a destination rectangle, optionally in combination with a brush
    /// pattern and with the application of a color mask bitmap, according to
    /// specified foreground and background raster operations.
    EMR_MASKBLT = 0x0000004E,
    /// This record specifies a block transfer of pixels from a source bitmap
    /// to a destination parallelogram, with the application of a color mask
    /// bitmap.
    EMR_PLGBLT = 0x0000004F,
    /// This record specifies a block transfer of pixels from specified
    /// scanlines of a source bitmap to a destination rectangle.
    EMR_SETDIBITSTODEVICE = 0x00000050,
    /// This record specifies a block transfer of pixels from a source bitmap
    /// to a destination rectangle, optionally in combination with a brush
    /// pattern, according to a specified raster operation, stretching or
    /// compressing the output to fit the dimensions of the destination, if
    /// necessary.
    EMR_STRETCHDIBITS = 0x00000051,
    /// This record defines a logical font that has the specified
    /// characteristics. The font can subsequently be selected as the current
    /// font.
    EMR_EXTCREATEFONTINDIRECTW = 0x00000052,
    /// This record draws an ASCII text string using the current font and text
    /// colors.
    EMR_EXTTEXTOUTA = 0x00000053,
    /// This record draws a Unicode text string using the current font and text
    /// colors.
    EMR_EXTTEXTOUTW = 0x00000054,
    /// This record defines one or more Bezier curves. The curves are drawn
    /// using the current pen.
    EMR_POLYBEZIER16 = 0x00000055,
    /// This record defines a polygon consisting of two or more vertexes
    /// connected by straight lines. The polygon is outlined by using the
    /// current pen and filled by using the current brush and polygon fill
    /// mode. The polygon is closed automatically by drawing a line from the
    /// last vertex to the first.
    EMR_POLYGON16 = 0x00000056,
    /// This record defines a series of line segments by connecting the points
    /// in the specified array.
    EMR_POLYLINE16 = 0x00000057,
    /// This record defines one or more Bezier curves based on the current
    /// position.
    EMR_POLYBEZIERTO16 = 0x00000058,
    /// This record defines one or more straight lines based upon the current
    /// position. A line is drawn from the current position to the first point
    /// specified by the Points field by using the current pen. For each
    /// additional line, drawing is performed from the ending point of the
    /// previous line to the next point specified by Points.
    EMR_POLYLINETO16 = 0x00000059,
    /// This record defines multiple series of connected line segments.
    EMR_POLYPOLYLINE16 = 0x0000005A,
    /// This record defines a series of closed polygons. Each polygon is
    /// outlined by using the current pen and filled by using the current brush
    /// and polygon fill mode. The polygons specified by this record can
    /// overlap.
    EMR_POLYPOLYGON16 = 0x0000005B,
    /// This record defines a set of line segments and Bezier curves.
    EMR_POLYDRAW16 = 0x0000005C,
    /// This record defines a logical brush with the specified bitmap pattern.
    /// The bitmap can be a device-independent bitmap (DIB) section bitmap or
    /// it can be a device- dependent bitmap.
    EMR_CREATEMONOBRUSH = 0x0000005D,
    /// This record defines a logical brush that has the pattern specified by
    /// the DIB.
    EMR_CREATEDIBPATTERNBRUSHPT = 0x0000005E,
    /// This record defines an extended logical pen that has the specified
    /// style, width, color, and brush attributes.
    EMR_EXTCREATEPEN = 0x0000005F,
    /// This record draws one or more ASCII text strings using the current font
    /// and text colors.
    ///
    /// Note: EMR_POLYTEXTOUTA SHOULD be emulated with a series of
    /// EMR_EXTTEXTOUTW records, one per string. Windows NT 3.1 is the only
    /// Windows version in which GDI uses EMR_POLYTEXTOUTA records for text
    /// output. All other versions emulate EMR_POLYTEXTOUTA with
    /// EMR_EXTTEXTOUTW records.
    EMR_POLYTEXTOUTA = 0x00000060,
    /// This record draws one or more Unicode text strings using the current
    /// font and text colors.
    ///
    /// Note: EMR_POLYTEXTOUTW SHOULD be emulated with a series of
    /// EMR_EXTTEXTOUTW records, one per string. Windows NT 3.1 is the only
    /// Windows version in which GDI uses EMR_POLYTEXTOUTW records for text
    /// output. All other versions emulate EMR_POLYTEXTOUTW with
    /// EMR_EXTTEXTOUTW records.
    EMR_POLYTEXTOUTW = 0x00000061,
    /// This record specifies the mode of Image Color Management (ICM) for
    /// graphics operations.
    ///
    /// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
    /// EMR_SETICMMODE.
    EMR_SETICMMODE = 0x00000062,
    /// This record creates a logical color space object from a color profile
    /// with a name consisting of ASCII characters.
    ///
    /// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
    /// EMR_CREATECOLORSPACE.
    EMR_CREATECOLORSPACE = 0x00000063,
    /// This record defines the current logical color space object for graphics
    /// operations.
    ///
    /// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
    /// EMR_SETCOLORSPACE.
    EMR_SETCOLORSPACE = 0x00000064,
    /// This record deletes a logical color space object.
    /// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
    /// EMR_DELETECOLORSPACE.
    ///
    /// Note: An EMR_DELETEOBJECT record SHOULD be used instead of
    /// EMR_DELETECOLORSPACE to delete a logical color space object.
    /// Windows uses an EMR_DELETEOBJECT record to delete a logical color space
    /// object.
    EMR_DELETECOLORSPACE = 0x00000065,
    /// This record specifies an OpenGL function.
    /// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
    /// EMR_GLSRECORD.
    EMR_GLSRECORD = 0x00000066,
    /// This record specifies an OpenGL function with a bounding rectangle for
    /// output. Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not
    /// support EMR_GLSBOUNDEDRECORD.
    EMR_GLSBOUNDEDRECORD = 0x00000067,
    /// This record specifies the pixel format to use for graphics operations.
    /// Windows NT 3.1, Windows NT 3.5, and Windows NT 3.51 do not support
    /// EMR_PIXELFORMAT.
    EMR_PIXELFORMAT = 0x00000068,
    /// This record passes arbitrary information to the driver. The intent is
    /// that the information results in drawing being done.
    EMR_DRAWESCAPE = 0x00000069,
    /// This record passes arbitrary information to the driver. The intent is
    /// that the information does not result in drawing being done.
    EMR_EXTESCAPE = 0x0000006A,
    /// This record outputs a string.
    EMR_SMALLTEXTOUT = 0x0000006C,
    /// This record forces the font mapper to match fonts based on their
    /// UniversalFontId in preference to their LogFont information.
    EMR_FORCEUFIMAPPING = 0x0000006D,
    /// This record passes arbitrary information to the given named driver.
    EMR_NAMEDESCAPE = 0x0000006E,
    /// This record specifies how to correct the entries of a logical palette
    /// object using Windows Color System (WCS) 1.0 values. Windows NT 3.1,
    /// Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not support
    /// EMR_COLORCORRECTPALETTE.
    EMR_COLORCORRECTPALETTE = 0x0000006F,
    /// This record specifies a color profile in a file with a name consisting
    /// of ASCII characters, for graphics output. Windows NT 3.1, Windows
    /// NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not support
    /// EMR_SETICMPROFILEA.
    EMR_SETICMPROFILEA = 0x00000070,
    /// This record specifies a color profile in a file with a name consisting
    /// of Unicode characters, for graphics output. Windows NT 3.1, Windows
    /// NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not support
    /// EMR_SETICMPROFILEW.
    EMR_SETICMPROFILEW = 0x00000071,
    /// This record specifies a block transfer of pixels from a source bitmap
    /// to a destination rectangle, including alpha transparency data,
    /// according to a specified blending operation. Windows NT 3.1,
    /// Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not support
    /// EMR_ALPHABLEND.
    EMR_ALPHABLEND = 0x00000072,
    /// This record specifies the order in which text and graphics are drawn.
    /// Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do
    /// not support EMR_SETLAYOUT.
    EMR_SETLAYOUT = 0x00000073,
    /// This record specifies a block transfer of pixels from a source bitmap
    /// to a destination rectangle, treating a specified color as transparent,
    /// stretching or compressing the output to fit the dimensions of the
    /// destination, if necessary. Windows NT 3.1, Windows NT 3.5, Windows
    /// NT 3.51, and Windows NT 4.0 do not support EMR_TRANSPARENTBLT.
    EMR_TRANSPARENTBLT = 0x00000074,
    /// This record specifies filling rectangles or triangles with gradients of
    /// color. Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and Windows
    /// NT 4.0 do not support EMR_GRADIENTFILL.
    EMR_GRADIENTFILL = 0x00000076,
    /// This record sets the UniversalFontIds of linked fonts to use during
    /// character lookup.
    EMR_SETLINKEDUFIS = 0x00000077,
    /// This record specifies the amount of extra space to add to break
    /// characters for justification purposes. Windows GDI uses an
    /// EMR_EXTTEXTOUTW record to perform this function.
    EMR_SETTEXTJUSTIFICATION = 0x00000078,
    /// This record specifies whether to perform color matching with a color
    /// profile that is specified in a file with a name consisting of Unicode
    /// characters. Windows NT 3.1, Windows NT 3.5, Windows NT 3.51, and
    /// Windows NT 4.0 do not support EMR_COLORMATCHTOTARGETW.
    EMR_COLORMATCHTOTARGETW = 0x00000079,
    /// This record creates a logical color space object from a color profile
    /// with a name consisting of Unicode characters. Windows NT 3.1,
    /// Windows NT 3.5, Windows NT 3.51, and Windows NT 4.0 do not support
    /// EMR_CREATECOLORSPACEW.
    EMR_CREATECOLORSPACEW = 0x0000007A,
}

crate::parser::enums::impl_parser!(RecordType, u32);
