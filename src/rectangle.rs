use amethyst::core::math;

pub trait Rectangle<T: math::RealField> {
    fn upper_left(&self) -> math::Vector2<T>;
    fn lower_right(&self) -> math::Vector2<T>;
    fn center(&self) -> math::Vector2<T>;
}

impl Rectangle<f32> for math::Vector4<f32> {
    fn upper_left(&self) -> math::Vector2<f32> {
        math::Vector2::new(self.x, self.y)
    }
    fn lower_right(&self) -> math::Vector2<f32> {
        math::Vector2::new(self.z, self.w)
    }

    fn center(&self) -> math::Vector2<f32> {
        (self.upper_left() + self.lower_right()) * 0.5
    }
}

pub fn overlap<T: math::RealField, U: Rectangle<T>>(rectangle1: U, rectangle2: U) -> bool {
    let (l1, r1) = (rectangle1.upper_left(), rectangle1.lower_right());
    let (l2, r2) = (rectangle2.upper_left(), rectangle2.lower_right());
    l1.x <= r2.x && r1.x >= l2.x && l1.y >= r2.y && r1.y <= l2.y
}
