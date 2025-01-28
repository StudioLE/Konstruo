use super::Container;
use super::*;
use bevy::prelude::*;
use taffy::prelude::*;

pub struct FlexFactory {
    pub(super) main_axis: Vec3,
    pub(super) cross_axis: Vec3,
    pub(super) items: Vec<Box<dyn Distributable>>,
}

impl Default for FlexFactory {
    fn default() -> Self {
        FlexFactory {
            main_axis: Vec3::X,
            cross_axis: Vec3::Y,
            items: Vec::new(),
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
                    // flex_direction: FlexDirection::Column,
                    // size: Size { width: a, height: length(600.0) },
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
