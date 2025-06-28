// use wmf_core::parser::*;

use crate::converter::{svg::*, *};

impl LogFont {
    pub fn set_props(
        &self,
        ctx: &PlaybackDeviceContext,
        mut elem: Node,
        point: &wmf_core::parser::PointL,
    ) -> (Node, Vec<String>) {
        let mut styles = vec![];

        if self.italic {
            styles.push("font-style: italic;".to_owned());
        }

        {
            let mut v = vec![];

            if self.underline {
                v.push("underline");
            }

            if self.strike_out {
                v.push("line-through");
            }

            if !v.is_empty() {
                styles.push(format!("text-decoration: {};", v.join(" ")));
            }
        };

        if self.orientation != 0 {
            elem = elem.set("rotate", (-self.orientation / 10).to_string());
        }

        if self.escapement != 0 {
            elem = elem.set(
                "transform",
                format!(
                    "rotate({}, {} {})",
                    -self.escapement / 10,
                    point.x,
                    point.y
                ),
            );
        }

        let scale = ctx.xform.calc_scale();
        let font_size =
            (f64::from(self.height.abs()) * f64::from(scale)).round() as i32;

        elem = elem
            .set("font-family", self.facename.as_str())
            .set("font-size", font_size.to_string())
            .set("font-weight", self.weight.to_string());

        (elem, styles)
    }
}

#[derive(Clone, Debug)]
pub enum Fill {
    Pattern { pattern: Node },
    Value { value: String },
}

impl Fill {
    pub fn from(ctx: &PlaybackDeviceContext, v: LogBrushEx) -> Self {
        match v {
            LogBrushEx::Solid { color } => {
                Fill::Value { value: color_from_color_ref(&color) }
            }
            LogBrushEx::Null => Fill::Value { value: "none".to_owned() },
            LogBrushEx::Hatched { color, brush_hatch } => {
                let ten = (10_f32 * ctx.xform.calc_scale()) as i32;
                let path = match brush_hatch {
                    HatchStyle::HS_HORIZONTAL => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to(format!("{ten} 0"));

                        Node::new("path")
                            .set("stroke", color_from_color_ref(&color))
                            .set("data", data.to_string())
                    }
                    HatchStyle::HS_VERTICAL => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to(format!("0 {ten}"));

                        Node::new("path")
                            .set("stroke", color_from_color_ref(&color))
                            .set("data", data.to_string())
                    }
                    HatchStyle::HS_FDIAGONAL => {
                        let data = Data::new()
                            .move_to(format!("0 {ten}"))
                            .line_to(format!("{ten} 0"));

                        Node::new("path")
                            .set("stroke", color_from_color_ref(&color))
                            .set("data", data.to_string())
                    }
                    HatchStyle::HS_BDIAGONAL => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to(format!("{ten} {ten}"));

                        Node::new("path")
                            .set("stroke", color_from_color_ref(&color))
                            .set("data", data.to_string())
                    }
                    HatchStyle::HS_CROSS => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to(format!("{ten} 0"))
                            .move_to("0 0")
                            .line_to(format!("0 {ten}"));

                        Node::new("path")
                            .set("stroke", color_from_color_ref(&color))
                            .set("data", data.to_string())
                    }
                    HatchStyle::HS_DIAGCROSS => {
                        let data = Data::new()
                            .move_to("0 0")
                            .line_to(format!("{ten} {ten}"))
                            .move_to(format!("{ten} 0"))
                            .line_to(format!("0 {ten}"));

                        Node::new("path")
                            .set("stroke", color_from_color_ref(&color))
                            .set("data", data.to_string())
                    }
                    HatchStyle::HS_SOLIDCLR => {
                        return Fill::Value {
                            value: color_from_color_ref(&color),
                        };
                    }
                    HatchStyle::HS_DITHEREDCLR => {
                        info!(?brush_hatch, "HatchStyle is not implemented.");

                        return Fill::Value {
                            value: color_from_color_ref(&color),
                        };
                    }
                    HatchStyle::HS_SOLIDTEXTCLR => {
                        return Fill::Value {
                            value: color_from_color_ref(
                                &ctx.graphics_environment.drawing.text_color,
                            ),
                        };
                    }
                    HatchStyle::HS_DITHEREDTEXTCLR => {
                        info!(?brush_hatch, "HatchStyle is not implemented.");

                        return Fill::Value {
                            value: color_from_color_ref(
                                &ctx.graphics_environment.drawing.text_color,
                            ),
                        };
                    }
                    HatchStyle::HS_SOLIDBKCLR => {
                        return Fill::Value {
                            value: color_from_color_ref(
                                &ctx.graphics_environment
                                    .drawing
                                    .background_color,
                            ),
                        };
                    }
                    HatchStyle::HS_DITHEREDBKCLR => {
                        info!(?brush_hatch, "HatchStyle is not implemented.");

                        return Fill::Value {
                            value: color_from_color_ref(
                                &ctx.graphics_environment
                                    .drawing
                                    .background_color,
                            ),
                        };
                    }
                };

                let pattern = Node::new("pattern")
                    .set("patternUnits", "userSpaceOnUse")
                    .set("patternContentUnits", "userSpaceOnUse")
                    .set("x", 0.to_string())
                    .set("y", 0.to_string())
                    .set("width", ten.to_string())
                    .set("height", ten.to_string())
                    .add(path);

                Fill::Pattern { pattern }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Stroke {
    /// sets the color of the line around an element
    color: wmf_core::parser::ColorRef,
    /// sets the width of the line around an element
    width: i16,
    /// sets the opacity of the line around an element
    opacity: f32,
    /// sets the shape of the end-lines for a line or open path
    line_cap: String,
    /// sets the line to show as a dashed line
    dash_array: String,
    /// sets the shape of the corners where two lines meet
    line_join: String,
}

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: wmf_core::parser::ColorRef::black(),
            width: 1,
            opacity: 1_f32,
            line_cap: "butt".to_owned(),
            dash_array: "none".to_owned(),
            line_join: "miter".to_owned(),
        }
    }
}

impl From<LogPenEx> for Stroke {
    fn from(v: LogPenEx) -> Self {
        let mut stroke = match v.brush {
            LogPenExBrush::DIBPattern { .. }
            | LogPenExBrush::DIBPatternPT { .. } => {
                Self { opacity: 0_f32, ..Default::default() }
            }
            LogPenExBrush::Hatched { color_ref, .. }
            | LogPenExBrush::Solid { color_ref } => {
                Self { color: color_ref, ..Default::default() }
            }
            LogPenExBrush::Pattern { .. } => Self::default(),
            LogPenExBrush::Null => {
                Self { width: 0, opacity: 0_f32, ..Default::default() }
            }
        };

        stroke.width = v.width as i16;

        for style in v.pen_style {
            stroke = match style {
                PenStyle::PS_SOLID => stroke,
                PenStyle::PS_DASH => {
                    stroke.dash_array =
                        format!("{v} {v}", v = stroke.width * 10);
                    stroke
                }
                PenStyle::PS_DOT | PenStyle::PS_ALTERNATE => {
                    stroke.dash_array =
                        format!("{} {}", stroke.width, stroke.width * 10);
                    stroke
                }
                PenStyle::PS_DASHDOT => {
                    stroke.dash_array = format!(
                        "{} {} {} {}",
                        stroke.width * 10,
                        stroke.width * 2,
                        stroke.width,
                        stroke.width * 2,
                    );
                    stroke
                }
                PenStyle::PS_DASHDOTDOT => {
                    stroke.dash_array = format!(
                        "{} {} {} {} {} {}",
                        stroke.width * 10,
                        stroke.width * 2,
                        stroke.width,
                        stroke.width * 2,
                        stroke.width,
                        stroke.width * 2,
                    );
                    stroke
                }
                PenStyle::PS_NULL => {
                    stroke.opacity = 0_f32;
                    stroke
                }
                PenStyle::PS_ENDCAP_SQUARE | PenStyle::PS_ENDCAP_FLAT => {
                    "square".clone_into(&mut stroke.line_cap);
                    stroke
                }
                PenStyle::PS_JOIN_BEVEL => {
                    "bevel".clone_into(&mut stroke.line_join);
                    stroke
                }
                PenStyle::PS_JOIN_MITER => {
                    "miter".clone_into(&mut stroke.line_join);
                    stroke
                }
                // not implemented
                PenStyle::PS_INSIDEFRAME
                | PenStyle::PS_USERSTYLE
                | PenStyle::PS_GEOMETRIC => {
                    info!(?style, "pen style is not implemented");
                    stroke
                }
            };
        }

        stroke
    }
}

impl Stroke {
    pub fn color(&self) -> String {
        color_from_color_ref(&self.color)
    }

    pub fn dash_array(&self) -> String {
        self.dash_array.clone()
    }

    pub fn line_cap(&self) -> String {
        self.line_cap.clone()
    }

    pub fn line_join(&self) -> String {
        self.line_join.clone()
    }

    pub fn opacity(&self) -> String {
        format!("{:.02}", self.opacity)
    }

    pub fn width(&self) -> i16 {
        self.width
    }

    pub fn set_props(&self, ctx: &PlaybackDeviceContext, elem: Node) -> Node {
        if self.opacity == 0_f32 {
            return elem.set("stroke", "none");
        }

        let scale = ctx.xform.calc_scale();
        let width = core::cmp::max((f32::from(self.width()) * scale) as i32, 1);
        let mut elem = elem
            .set("stroke", self.color())
            .set("stroke-dasharray", self.dash_array())
            .set("stroke-linecap", self.line_cap())
            .set("stroke-linejoin", self.line_join())
            .set("stroke-opacity", self.opacity())
            .set("stroke-width", width.to_string());

        if let Some(ref limit) = ctx.graphics_environment.drawing.miter_limit {
            elem = elem.set(
                "stroke-miterlimit",
                ((*limit as f32) * scale).to_string(),
            );
        }

        elem
    }
}

#[inline]
pub fn text_align(value: u32) -> String {
    use strum::IntoEnumIterator;

    let align = wmf_core::parser::TextAlignmentMode::iter()
        .filter(|c| value as u16 & (*c as u16) == (*c as u16))
        .collect::<BTreeSet<_>>();

    if align.contains(&wmf_core::parser::TextAlignmentMode::TA_RIGHT) {
        "end".to_owned()
    } else if align.contains(&wmf_core::parser::TextAlignmentMode::TA_CENTER) {
        "middle".to_owned()
    } else {
        "start".to_owned()
    }
}

#[inline]
pub fn as_point_string_from_point_l(
    point: &wmf_core::parser::PointL,
) -> String {
    format!("{},{}", point.x, point.y)
}

#[inline]
pub fn as_point_string_from_point_s(
    point: &wmf_core::parser::PointS,
) -> String {
    format!("{},{}", point.x, point.y)
}

#[inline]
pub fn color_from_color_ref(c: &wmf_core::parser::ColorRef) -> String {
    format!("#{:02X}{:02X}{:02X}", c.red, c.green, c.blue)
}

#[inline]
pub fn url_string(link: &str) -> String {
    format!("url({link})")
}

#[inline]
pub fn polygon_fill_rule(polyfill_mode: PolygonFillMode) -> String {
    match polyfill_mode {
        PolygonFillMode::ALTERNATE => "evenodd",
        PolygonFillMode::WINDING => "nonzero",
    }
    .to_owned()
}
