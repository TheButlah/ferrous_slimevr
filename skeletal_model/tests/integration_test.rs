use skeletal_model::bone::BoneMap;
use skeletal_model::prelude::*;
use skeletal_model::Skeleton;
use skeletal_model::skeleton::SkeletonConfig;


// A minor example showing how the code is supposed to work
#[test]
fn skeleton_test() {
    let mut bone_lengths = BoneMap::new([0.; BoneKind::num_types()]);

    bone_lengths[BoneKind::FootL] = 4.0;

    let config = SkeletonConfig { bone_lengths };

    let skeleton = Skeleton::new(&config);

    assert_eq!(skeleton[BoneKind::FootL].length(), 4.);
    assert_eq!(skeleton[BoneKind::UpperArmR].length(), 0.);
}
