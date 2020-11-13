use amethyst::core::math;

pub trait Rectangle<T: math::RealField> {
    fn upper_left(&self) -> math::Vector2<T>;
    fn lower_right(&self) -> math::Vector2<T>;
}

impl<T: math::RealField> Rectangle<T> for math::Vector4<T> {
    fn upper_left(&self) -> math::Vector2<T> {
        math::Vector2::new(self.x, self.y)
    }
    fn lower_right(&self) -> math::Vector2<T> {
        math::Vector2::new(self.z, self.w)
    }
}

pub fn overlap<T: math::RealField, U: Rectangle<T>>(rectangle1: U, rectangle2: U) -> bool {
    let (l1, r1) = (rectangle1.upper_left(), rectangle1.lower_right());
    let (l2, r2) = (rectangle2.upper_left(), rectangle2.lower_right());
    l1.x <= r2.x && r1.x >= l2.x && l1.y >= r2.y && r1.y <= l2.y
}
