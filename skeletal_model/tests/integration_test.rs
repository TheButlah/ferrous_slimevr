use skeletal_model::bone::BoneMap;
use skeletal_model::prelude::*;
use skeletal_model::skeleton::SkeletonConfig;
use skeletal_model::Skeleton;

/// Tests that all lengths of the skeleton are properly initialized based on `SkeletonConfig`
#[test]
fn test_lengths() {
    let mut bone_lengths = BoneMap::new([0.; BoneKind::num_types()]);

    bone_lengths[BoneKind::FootL] = 4.0;

    let config = SkeletonConfig { bone_lengths };

    let skeleton = Skeleton::new(&config);

    for (i, j) in bone_lengths.iter() {
        assert_eq!(&skeleton[i].length(), j);
    }
}
