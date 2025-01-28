use super::Container;
use super::*;
use bevy::prelude::*;
use bevy::prelude::{AlignContent, AlignItems, JustifyContent};
use taffy::prelude::*;

pub struct FlexFactory {
    pub(super) main_axis: Vec3,
    pub(super) cross_axis: Vec3,
    pub(super) justify_content: JustifyContent,
    pub(super) align_content: AlignContent,
    pub(super) align_items: AlignItems,
    pub(super) items: Vec<Box<dyn Distributable>>,
}

impl Default for FlexFactory {
    fn default() -> Self {
        FlexFactory {
            main_axis: Vec3::X,
            cross_axis: Vec3::Y,
            items: Vec::new(),
            justify_content: JustifyContent::FlexStart,
            align_content: AlignContent::FlexStart,
            align_items: AlignItems::FlexStart,
        }
    }
}

impl FlexFactory {
    #[must_use]
    pub fn execute(self) -> Container {
        let mut tree: TaffyTree<()> = TaffyTree::new();
        let item_nodes: Vec<NodeId> = self
            .items
            .iter()
            .map(|item| {
                let main_size = (item.get_size() * self.main_axis).length();
                let cross_size = (item.get_size() * self.cross_axis).length();
                tree.new_leaf(Style {
                    size: Size {
                        // TODO: TAFFY LENGTHS ARE RETURNED ROUNDED
                        width: length(main_size),
                        height: length(cross_size),
                    },
                    flex_grow: 0.0,
                    flex_shrink: 0.0,
                    ..default()
                })
                .expect("taffy new_leaf should not fail")
            })
            .collect();
        let root_node = tree
            .new_with_children(
                Style {
                    display: taffy::Display::Flex,
                    justify_content: Some(justify_content_to_taffy(self.justify_content)),
                    align_content: Some(align_content_to_taffy(self.align_content)),
                    align_items: Some(align_items_to_taffy(self.align_items)),
                    ..Default::default()
                },
                &item_nodes,
            )
            .expect("taffy new_with_children should not fail");
        tree.compute_layout(root_node, Size::MAX_CONTENT)
            .expect("taffy compute_layout should not fail");
        let root_layout = tree.layout(root_node).expect("root layout should not fail");
        let container_size = self.get_size(root_layout);
        let items: Vec<DistributedItem> = item_nodes
            .into_iter()
            .map(|node| {
                let layout = tree.layout(node).expect("item layout should not fail");
                let size = self.get_size(layout);
                let translation = self.get_translation(layout, container_size);
                DistributedItem {
                    item: Box::new(Placeholder),
                    size,
                    translation,
                }
            })
            .collect();
        let items = items
            .into_iter()
            .zip(self.items)
            .map(|(mut distributed, boxed)| {
                distributed.item = boxed;
                distributed
            })
            .collect();
        Container {
            size: container_size,
            items,
        }
    }

    fn get_size(&self, layout: &Layout) -> Vec3 {
        let main = layout.size.width * self.main_axis;
        let cross = layout.size.height * self.cross_axis;
        main + cross
    }

    /// Get the translation to the center of the item
    fn get_translation(&self, layout: &Layout, container_size: Vec3) -> Vec3 {
        let main = (layout.location.x + layout.size.width * 0.5) * self.main_axis;
        let cross = (layout.location.y + layout.size.height * 0.5) * self.cross_axis;
        main + cross - container_size * 0.5
    }
}

struct Placeholder;

impl Distributable for Placeholder {
    #[allow(clippy::panic)]
    fn get_size(&self) -> Vec3 {
        panic!("Tried to get size of a placeholder");
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
