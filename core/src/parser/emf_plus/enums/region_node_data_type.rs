/// The RegionNodeDataType enumeration defines types of region node data
/// (MS-EMFPLUS 2.1.1.27).
///
/// Region node data is specified by EmfPlusRegionNode objects.
///
/// Values below 0x10000000 combine two child nodes; the remaining
/// values are terminal nodes.
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
pub enum RegionNodeDataType {
    /// A region node with child nodes. A Boolean AND operation SHOULD
    /// be applied to the left and right child nodes specified by an
    /// EmfPlusRegionNodeChildNodes object.
    RegionNodeDataTypeAnd = 0x00000001,
    /// A region node with child nodes. A Boolean OR operation SHOULD be
    /// applied to the left and right child nodes specified by an
    /// EmfPlusRegionNodeChildNodes object.
    RegionNodeDataTypeOr = 0x00000002,
    /// A region node with child nodes. A Boolean XOR operation SHOULD
    /// be applied to the left and right child nodes specified by an
    /// EmfPlusRegionNodeChildNodes object.
    RegionNodeDataTypeXor = 0x00000003,
    /// A region node with child nodes. A Boolean operation, defined as
    /// "the part of region 1 that is excluded from region 2", SHOULD be
    /// applied to the left and right child nodes specified by an
    /// EmfPlusRegionNodeChildNodes object.
    RegionNodeDataTypeExclude = 0x00000004,
    /// A region node with child nodes. A Boolean operation, defined as
    /// "the part of region 2 that is excluded from region 1", SHOULD be
    /// applied to the left and right child nodes specified by an
    /// EmfPlusRegionNodeChildNodes object.
    RegionNodeDataTypeComplement = 0x00000005,
    /// A region node with no child nodes. The RegionNodeData field
    /// SHOULD specify a boundary with an EmfPlusRectF object.
    RegionNodeDataTypeRect = 0x10000000,
    /// A region node with no child nodes. The RegionNodeData field
    /// SHOULD specify a boundary with an EmfPlusRegionNodePath object.
    RegionNodeDataTypePath = 0x10000001,
    /// A region node with no child nodes. The RegionNodeData field
    /// SHOULD NOT be present.
    RegionNodeDataTypeEmpty = 0x10000002,
    /// A region node with no child nodes, and its bounds are not
    /// defined.
    RegionNodeDataTypeInfinite = 0x10000003,
}

crate::parser::enums::impl_parser!(RegionNodeDataType, u32);

impl RegionNodeDataType {
    /// Whether this node type combines two child nodes.
    pub fn has_child_nodes(self) -> bool {
        matches!(
            self,
            Self::RegionNodeDataTypeAnd
                | Self::RegionNodeDataTypeOr
                | Self::RegionNodeDataTypeXor
                | Self::RegionNodeDataTypeExclude
                | Self::RegionNodeDataTypeComplement
        )
    }
}
