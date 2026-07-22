//! Region objects (MS-EMFPLUS 2.2.1.8 EmfPlusRegion, 2.2.2.40
//! EmfPlusRegionNode, 2.2.2.41 EmfPlusRegionNodeChildNodes, 2.2.2.42
//! EmfPlusRegionNodePath).

use crate::{
    imports::*,
    parser::emf_plus::objects::{
        EmfPlusGraphicsVersion, EmfPlusPath, EmfPlusRectF,
        custom_line_cap::parse_length_prefixed_path,
    },
};

/// Maximum region node tree depth accepted by the parser.
///
/// The tree is parsed recursively; without a bound, a crafted stream of
/// nested combine nodes (4 bytes each) could exhaust the stack. Real
/// regions combine at most a handful of nodes; 256 leaves generous
/// headroom while keeping the worst-case recursion shallow.
const MAX_REGION_NODE_DEPTH: usize = 256;

/// The EmfPlusRegion object specifies line and curve segments that
/// define a nonrectilinear shape (MS-EMFPLUS 2.2.1.8).
#[derive(Clone, Debug, PartialEq)]
pub struct EmfPlusRegion {
    /// Version (4 bytes): An EmfPlusGraphicsVersion object (section
    /// 2.2.2.19) that specifies the version of operating system
    /// graphics that was used to create this object.
    pub version: EmfPlusGraphicsVersion,
    /// RegionNodeCount (4 bytes): An unsigned integer that specifies
    /// the number of child nodes in the RegionNode field.
    pub region_node_count: u32,
    /// RegionNode (variable): An array of RegionNodeCount+1
    /// EmfPlusRegionNode objects (section 2.2.2.40). Regions are
    /// specified as a binary tree of region nodes, and each node MUST
    /// either be a terminal node or specify one or two child nodes.
    /// RegionNode MUST contain at least one element.
    ///
    /// The nodes are parsed into a tree: this field holds the root
    /// node, and child nodes are nested within it instead of being
    /// stored as a flat array.
    pub region_node: EmfPlusRegionNode,
}

/// The EmfPlusRegionNode object specifies nodes of a graphics region
/// (MS-EMFPLUS 2.2.2.40). The RegionNodeDataType field of the wire
/// format is folded into the variant.
///
/// Graphics regions are specified by EmfPlusRegion objects (section
/// 2.2.1.8), which define a binary tree of region nodes. Each node
/// MUST either be a terminal node or specify additional region nodes.
///
/// This object is generic and is used to specify different types of
/// region node data, including:
///
/// - An EmfPlusRegionNodePath object (section 2.2.2.42), for a terminal node;
/// - An EmfPlusRectF object (section 2.2.2.39), for a terminal node; and
/// - An EmfPlusRegionNodeChildNodes object (section 2.2.2.41), for a
///   non-terminal node.
#[derive(Clone, Debug, PartialEq)]
pub enum EmfPlusRegionNode {
    /// The EmfPlusRegionNodeChildNodes object specifies child nodes of
    /// a graphics region node (MS-EMFPLUS 2.2.2.41).
    ChildNodes {
        /// How the two children combine (And / Or / Xor / Exclude /
        /// Complement).
        combine: crate::parser::emf_plus::RegionNodeDataType,
        /// Left (variable): An EmfPlusRegionNode object (section
        /// 2.2.2.40) that specifies the left child node of this region
        /// node.
        left: Box<EmfPlusRegionNode>,
        /// Right (variable): An EmfPlusRegionNode object that defines
        /// the right child node of this region node.
        right: Box<EmfPlusRegionNode>,
    },
    /// A terminal rectangle node.
    Rect(EmfPlusRectF),
    /// The EmfPlusRegionNodePath object specifies a graphics path for
    /// drawing the boundary of a region node (MS-EMFPLUS 2.2.2.42).
    ///
    /// The RegionNodePathLength prefix of the envelope is consumed at
    /// parse time; only the inner EmfPlusPath is kept.
    Path(EmfPlusPath),
    /// A terminal empty region node.
    Empty,
    /// A terminal infinite region node.
    Infinite,
}

impl EmfPlusRegion {
    #[cfg_attr(feature = "tracing", tracing::instrument(
        level = tracing::Level::TRACE,
        skip_all,
        err(level = tracing::Level::ERROR, Display),
    ))]
    pub fn parse<R: crate::Read>(
        buf: &mut R,
    ) -> Result<(Self, usize), crate::parser::ParseError> {
        use crate::parser::records::{read_field, read_with};

        let mut consumed_bytes: usize = 0;
        let version =
            read_with(buf, &mut consumed_bytes, EmfPlusGraphicsVersion::parse)?;
        let region_node_count: u32 = read_field(buf, &mut consumed_bytes)?;

        crate::parser::emf_plus::check_element_count(
            "RegionNodeCount",
            region_node_count,
        )?;

        let region_node =
            EmfPlusRegionNode::parse_node(buf, &mut consumed_bytes, 0)?;

        Ok((Self { version, region_node_count, region_node }, consumed_bytes))
    }
}

impl EmfPlusRegionNode {
    fn parse_node<R: crate::Read>(
        buf: &mut R,
        tracker: &mut usize,
        depth: usize,
    ) -> Result<Self, crate::parser::ParseError> {
        use crate::parser::{emf_plus::RegionNodeDataType, records::read_with};

        if depth >= MAX_REGION_NODE_DEPTH {
            return Err(crate::parser::ParseError::UnexpectedPattern {
                cause: alloc::format!(
                    "region node tree exceeds the maximum depth of \
                     {MAX_REGION_NODE_DEPTH}",
                )
                .into(),
            });
        }

        let node_type = read_with(buf, tracker, RegionNodeDataType::parse)?;

        let node = match node_type {
            RegionNodeDataType::RegionNodeDataTypeAnd
            | RegionNodeDataType::RegionNodeDataTypeOr
            | RegionNodeDataType::RegionNodeDataTypeXor
            | RegionNodeDataType::RegionNodeDataTypeExclude
            | RegionNodeDataType::RegionNodeDataTypeComplement => {
                let left = Self::parse_node(buf, tracker, depth + 1)?;
                let right = Self::parse_node(buf, tracker, depth + 1)?;

                Self::ChildNodes {
                    combine: node_type,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            }
            RegionNodeDataType::RegionNodeDataTypeRect => {
                Self::Rect(read_with(buf, tracker, EmfPlusRectF::parse)?)
            }
            RegionNodeDataType::RegionNodeDataTypePath => {
                Self::Path(parse_length_prefixed_path(buf, tracker)?)
            }
            RegionNodeDataType::RegionNodeDataTypeEmpty => Self::Empty,
            RegionNodeDataType::RegionNodeDataTypeInfinite => Self::Infinite,
        };

        Ok(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version_bytes() -> [u8; 4] {
        0xDBC0_1002_u32.to_le_bytes()
    }

    fn rect_node_bytes() -> Vec<u8> {
        let mut data = vec![];
        data.extend(0x1000_0000_u32.to_le_bytes());
        for v in [1.0_f32, 2.0, 3.0, 4.0] {
            data.extend(v.to_le_bytes());
        }
        data
    }

    #[test]
    fn parses_combined_nodes() {
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(2_u32.to_le_bytes());
        data.extend(0x0000_0001_u32.to_le_bytes()); // And
        data.extend(rect_node_bytes());
        data.extend(0x1000_0003_u32.to_le_bytes()); // Infinite

        let mut buf: &[u8] = &data;
        let (region, consumed) = EmfPlusRegion::parse(&mut buf).unwrap();

        assert_eq!(consumed, data.len());
        assert_eq!(region.region_node_count, 2);

        let EmfPlusRegionNode::ChildNodes { combine, left, right } =
            region.region_node
        else {
            panic!("expected combined region node");
        };
        assert_eq!(
            combine,
            crate::parser::emf_plus::RegionNodeDataType::RegionNodeDataTypeAnd,
        );
        assert!(matches!(*left, EmfPlusRegionNode::Rect(_)));
        assert!(matches!(*right, EmfPlusRegionNode::Infinite));
    }

    #[test]
    fn rejects_unbounded_nesting() {
        // A chain of And nodes with no terminals: each 4-byte node
        // descends one level, overrunning the depth limit long before
        // the buffer ends.
        let mut data = vec![];
        data.extend(version_bytes());
        data.extend(1_u32.to_le_bytes());
        for _ in 0..=MAX_REGION_NODE_DEPTH {
            data.extend(0x0000_0001_u32.to_le_bytes());
        }

        let mut buf: &[u8] = &data;

        assert!(EmfPlusRegion::parse(&mut buf).is_err());
    }
}
