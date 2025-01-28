use super::*;
use bevy::prelude::*;
use taffy::prelude::{
    auto, length, Dimension, Layout, NodeId, Size, Style, TaffyMaxContent, TaffyTree,
};
use taffy::Point;

/// Precision modifier to compensate for taffy values being rounded to integers
const PRECISION: f32 = 0.000_001;

pub(super) struct TaffyFlexFactory {
    pub(super) main_axis: Vec3,
    pub(super) cross_axis: Vec3,
    pub(super) flex_wrap: FlexWrap,
    pub(super) justify_content: JustifyContent,
    pub(super) align_content: AlignContent,
    pub(super) align_items: AlignItems,
    pub(super) bounds: Option<Vec3>,
}

impl TaffyFlexFactory {
    pub(super) fn execute(&self, container: &mut Container) {
        let (root_layout, item_layouts) = self.layout_with_taffy(container);
        let container_size = self.from_size(root_layout.size);
        for (item, layout) in container.items.iter_mut().zip(item_layouts) {
            item.size += self.from_size(layout.size);
            item.translation += self.get_translation(&layout, container_size);
        }
        container.size += container_size;
    }

    fn layout_with_taffy(&self, container: &Container) -> (Layout, Vec<Layout>) {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let item_nodes: Vec<NodeId> = container
            .items
            .iter()
            .map(|item| {
                tree.new_leaf(self.get_item_style(&item.size))
                    .expect("taffy new_leaf should not fail")
            })
            .collect();
        let root_node = tree
            .new_with_children(self.get_parent_style(), &item_nodes)
            .expect("taffy new_with_children should not fail");
        tree.compute_layout(root_node, Size::MAX_CONTENT)
            .expect("taffy compute_layout should not fail");
        let root_layout = *tree.layout(root_node).expect("root layout should not fail");
        let layouts = item_nodes
            .into_iter()
            .map(|node| *tree.layout(node).expect("item layout should not fail"))
            .collect();
        (root_layout, layouts)
    }

    #[allow(clippy::borrowed_box)]
    fn get_item_style(&self, item: &Vec3) -> Style {
        Style {
            size: self.to_size(item),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            ..default()
        }
    }

    fn get_parent_style(&self) -> Style {
        let size = match self.bounds {
            None => Size {
                width: auto(),
                height: auto(),
            },
            Some(bounds) => self.to_size(&bounds)
        };
        Style {
            display: taffy::Display::Flex,
            size,
            justify_content: Some(justify_content_to_taffy(self.justify_content)),
            align_content: Some(align_content_to_taffy(self.align_content)),
            align_items: Some(align_items_to_taffy(self.align_items)),
            flex_wrap: flex_wrap_to_taffy(self.flex_wrap),
            ..Default::default()
        }
    }

    /// Get the translation to the center of the item
    fn get_translation(&self, layout: &Layout, container_size: Vec3) -> Vec3 {
        let translation = self.from_point(layout.location) + self.from_size(layout.size) * 0.5;
        translation - container_size * 0.5 * (self.main_axis + self.cross_axis)
    }

    /// Convert from a taffy [`Point`] to a [`Vec3`].
    ///
    /// Values are mapped to the main and cross axis and multiplied by the [`PRECISION`].
    #[allow(clippy::wrong_self_convention)]
    fn from_point(&self, point: Point<f32>) -> Vec3 {
        let main = point.x * self.main_axis * PRECISION;
        let cross = point.y * self.cross_axis * PRECISION;
        main + cross
    }

    /// Convert from a taffy [`Size`] to a [`Vec3`].
    ///
    /// Values are mapped to the main and cross axis and multiplied by the [`PRECISION`].
    #[allow(clippy::wrong_self_convention)]
    fn from_size(&self, size: Size<f32>) -> Vec3 {
        let main = size.width * self.main_axis * PRECISION;
        let cross = size.height * self.cross_axis * PRECISION;
        main + cross
    }

    /// Convert from a [`Vec3`] to a taffy [`Size`].
    ///
    /// Values are mapped from the main and cross axis and divided by the [`PRECISION`].
    fn to_size(&self, vector: &Vec3) -> Size<Dimension> {
        let main_size = (vector * self.main_axis).length() / PRECISION;
        let cross_size = (vector * self.cross_axis).length() / PRECISION;
        Size {
            width: length(main_size),
            height: length(cross_size),
        }
    }
}

fn align_content_to_taffy(align_content: AlignContent) -> taffy::AlignContent {
    match align_content {
        AlignContent::Start => taffy::AlignContent::Start,
        AlignContent::End => taffy::AlignContent::End,
        AlignContent::Default | AlignContent::FlexStart => taffy::AlignContent::FlexStart,
        AlignContent::FlexEnd => taffy::AlignContent::FlexEnd,
        AlignContent::Center => taffy::AlignContent::Center,
        AlignContent::Stretch => taffy::AlignContent::Stretch,
        AlignContent::SpaceBetween => taffy::AlignContent::SpaceBetween,
        AlignContent::SpaceEvenly => taffy::AlignContent::SpaceEvenly,
        AlignContent::SpaceAround => taffy::AlignContent::SpaceAround,
    }
}

fn justify_content_to_taffy(justify_content: JustifyContent) -> taffy::JustifyContent {
    match justify_content {
        JustifyContent::Start => taffy::JustifyContent::Start,
        JustifyContent::End => taffy::JustifyContent::End,
        JustifyContent::Default | JustifyContent::FlexStart => taffy::JustifyContent::FlexStart,
        JustifyContent::FlexEnd => taffy::JustifyContent::FlexEnd,
        JustifyContent::Center => taffy::JustifyContent::Center,
        JustifyContent::Stretch => taffy::JustifyContent::Stretch,
        JustifyContent::SpaceBetween => taffy::JustifyContent::SpaceBetween,
        JustifyContent::SpaceEvenly => taffy::JustifyContent::SpaceEvenly,
        JustifyContent::SpaceAround => taffy::JustifyContent::SpaceAround,
    }
}

fn align_items_to_taffy(align_items: AlignItems) -> taffy::AlignItems {
    match align_items {
        AlignItems::Start => taffy::AlignItems::Start,
        AlignItems::End => taffy::AlignItems::End,
        AlignItems::Default | AlignItems::FlexStart => taffy::AlignItems::FlexStart,
        AlignItems::FlexEnd => taffy::AlignItems::FlexEnd,
        AlignItems::Center => taffy::AlignItems::Center,
        AlignItems::Stretch => taffy::AlignItems::Stretch,
        AlignItems::Baseline => taffy::AlignItems::Baseline,
    }
}

fn flex_wrap_to_taffy(flex_wrap: FlexWrap) -> taffy::FlexWrap {
    match flex_wrap {
        FlexWrap::NoWrap => taffy::FlexWrap::NoWrap,
        FlexWrap::Wrap => taffy::FlexWrap::Wrap,
        FlexWrap::WrapReverse => taffy::FlexWrap::WrapReverse,
    }
}
