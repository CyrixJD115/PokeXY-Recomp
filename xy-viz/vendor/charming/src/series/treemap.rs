use crate::{
    datatype::CompositeValue,
    element::{Emphasis, ItemStyle, Label, Tooltip},
};
use charming_macros::CharmingSetters;
use serde::{Deserialize, Serialize};

/// A treemap data node: `{name, value, children, itemStyle, label}` —
/// RDComp patch: upstream 0.6.0's Treemap has no `data` field at all.
#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TreemapNode {
    name: Option<String>,
    value: Option<f64>,
    children: Option<Vec<TreemapNode>>,
    item_style: Option<ItemStyle>,
    label: Option<Label>,
}

#[serde_with::apply(
  Option => #[serde(skip_serializing_if = "Option::is_none")],
  Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")]
)]
#[derive(Serialize, Deserialize, CharmingSetters, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Treemap {
    #[serde(rename = "type")]
    #[charming_type = "treemap"]
    type_: String,
    id: Option<String>,
    name: Option<String>,
    zlevel: Option<f64>,
    z: Option<f64>,
    left: Option<CompositeValue>,
    top: Option<CompositeValue>,
    right: Option<CompositeValue>,
    bottom: Option<CompositeValue>,
    width: Option<CompositeValue>,
    height: Option<CompositeValue>,
    label: Option<Label>,
    upper_label: Option<Label>,
    item_style: Option<ItemStyle>,
    emphasis: Option<Emphasis>,
    tooltip: Option<Tooltip>,
    #[charming_set_vec]
    data: Vec<TreemapNode>,
    /// RDComp patch: hide the default-on breadcrumb bar in static renders.
    breadcrumb: Option<serde_json::Value>,
}
