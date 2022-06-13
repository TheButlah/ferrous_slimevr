use crate::bone::{Bone, BoneKind, BoneMap};
use crate::joint::Joint;

use core::ops::Index;
use daggy::{Dag, EdgeIndex};

/// Used to initialize the [`Skeleton`](crate::skeleton::Skeleton) with its initial parameters
pub struct SkeletonConfig {
    bone_lengths: BoneMap<f32>,
}
impl SkeletonConfig {
    pub fn new(bone_lengths: BoneMap<f32>) -> Self {
        SkeletonConfig { bone_lengths }
    }
}

pub struct Skeleton {
    bone_map: BoneMap<EdgeIndex>,
    graph: Dag<Joint, Bone>,
}
impl Skeleton {
    pub fn new(config: &SkeletonConfig) -> Self {
        let mut g = Dag::new();

        // Option is used for resiliance against bugs while the map is being built
        let mut bone_map: BoneMap<Option<EdgeIndex>> = BoneMap::default();

        // Create root bone: edge (bone) connects to nodes (joints)
        {
            let head = g.add_node(Joint::new());
            let (edge, _tail) = g.add_child(
                head,
                Bone::new(BoneKind::Neck, config.bone_lengths[BoneKind::Neck]),
                Joint::new(),
            );
            bone_map[BoneKind::Neck] = Some(edge);
        }

        // This closure adds all the immediate children of `parent_bone` to the graph
        let mut add_child_bones = |parent_bone: BoneKind| {
            let parent_edge =
                bone_map[parent_bone].expect("Bone was not yet added to graph");
            let head = g.edge_endpoints(parent_edge).unwrap().1; // Get child node of edge
            for child_kind in parent_bone.children() {
                // No need to work with a ref, `child_kind` is `Copy`
                let child_kind = *child_kind;

                let (edge, _tail) = g.add_child(
                    head,
                    Bone::new(child_kind, config.bone_lengths[child_kind]),
                    Joint::new(),
                );

                bone_map[child_kind] = Some(edge);
            }
        };

        // Call `add_child_bones` in a depth-first traversal to build the actual graph.
        let mut bone_stack = vec![BoneKind::Neck];
        while !bone_stack.is_empty() {
            let parent_bone = bone_stack.pop().unwrap();
            add_child_bones(parent_bone);
            bone_stack.extend(parent_bone.children());
        }

        // Map is populated, get rid of the `Optional`
        let bone_map: BoneMap<EdgeIndex> = bone_map.map(|_kind, bone| bone.unwrap());

        Self { graph: g, bone_map }
    }
}
impl Index<BoneKind> for Skeleton {
    type Output = Bone;

    fn index(&self, index: BoneKind) -> &Self::Output {
        let edge = self.bone_map[index];
        &self.graph[edge]
    }
}

#[cfg(test)]
mod test {
    use crate::bone::BoneMap;
    use crate::prelude::*;
    use crate::skeleton::SkeletonConfig;
    use crate::Skeleton;

    /// Tests that all lengths of the skeleton are properly initialized based on `SkeletonConfig`
    #[test]
    fn test_lengths() {
        let mut bone_lengths = BoneMap::new([0.; BoneKind::num_types()]);

        bone_lengths[BoneKind::FootL] = 4.0;

        let config = SkeletonConfig::new(bone_lengths);

        let skeleton = Skeleton::new(&config);

        for (i, j) in bone_lengths.iter() {
            assert_eq!(&skeleton[i].length(), j);
        }
    }
}
