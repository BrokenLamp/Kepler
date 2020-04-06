use quaternion::Quaternion;
use specs::prelude::*;

pub struct Rotation(Quaternion<f32>);

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}
