use std::ops::{Deref, DerefMut};

use super::shape::Shape;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Group {
    children: Vec<Shape>,
}

#[allow(dead_code)]
impl Group {
    pub fn add_shape(&mut self, shape: Shape) {
        self.children.push(shape)
    }
}

impl Deref for Group {
    type Target = Vec<Shape>;
    fn deref(&self) -> &Self::Target {
        &self.children
    }
}

impl DerefMut for Group {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.children
    }
}

#[cfg(test)]
mod test {

    use crate::{
        point::Point,
        ray::Ray,
        shapes::ShapeWorld,
        transform::{Transform, Transformable},
        vector::Vector,
    };

    use super::*;

    #[test]
    fn intersecting_ray_with_empty_group() {
        let g = Shape::group();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn intersecting_ray_with_nonempty_group() {
        let mut g = Shape::group();
        let s1 = Shape::sphere();
        let s2 = Shape::sphere().with_transform(Transform::translation(0.0, 0.0, -3.0));
        let s3 = Shape::sphere().with_transform(Transform::translation(5.0, 0.0, 0.0));
        g.add_shape(s1);
        g.add_shape(s2);
        g.add_shape(s3);
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.count(), 4);
        assert_eq!(
            xs.get(0).map(|v| v.object()),
            g.as_group().map(|g| &g.children[1])
        );
        assert_eq!(
            xs.get(1).map(|v| v.object()),
            g.as_group().map(|g| &g.children[1])
        );
        assert_eq!(
            xs.get(2).map(|v| v.object()),
            g.as_group().map(|g| &g.children[0])
        );
        assert_eq!(
            xs.get(3).map(|v| v.object()),
            g.as_group().map(|g| &g.children[0])
        );
    }

    #[test]
    fn intersecting_transformed_group() {
        let s = Shape::sphere().with_transform(Transform::translation(5.0, 0.0, 0.0));
        let mut g = Shape::group().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        g.add_shape(s);
        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.count(), 2);
    }

    #[test]
    fn convert_point_from_world_to_object_space() {
        let mut g1 =
            Shape::group().with_transform(Transform::rotation_y(std::f64::consts::PI / 2.0));
        let mut g2 = Shape::group().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let s = Shape::sphere().with_transform(Transform::translation(5.0, 0.0, 0.0));
        g2.add_shape(s);
        g1.add_shape(g2);

        let shape = &g1.as_group().unwrap().children[0]
            .as_group()
            .unwrap()
            .children[0];
        let p = shape.world_to_object(&Point::new(-2.0, 0.0, -10.0));
        assert_eq!(p, Some(Point::new(0.0, 0.0, -1.0)));
    }

    #[test]
    fn convert_normal_from_object_to_world() {
        let mut g1 =
            Shape::group().with_transform(Transform::rotation_y(std::f64::consts::PI / 2.0));
        let mut g2 = Shape::group().with_transform(Transform::scaling(1.0, 2.0, 3.0));
        let s = Shape::sphere().with_transform(Transform::translation(5.0, 0.0, 0.0));
        g2.add_shape(s);
        g1.add_shape(g2);

        let shape = &g1.as_group().unwrap().children[0]
            .as_group()
            .unwrap()
            .children[0];
        let n = shape.normal_to_world(&Vector::new(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, Some(Vector::new(0.28571, 0.42857, -0.85714)));
    }

    #[test]
    fn finding_normal_on_a_child_object() {
        let mut g1 =
            Shape::group().with_transform(Transform::rotation_y(std::f64::consts::PI / 2.0));
        let mut g2 = Shape::group().with_transform(Transform::scaling(1.0, 2.0, 3.0));
        let s = Shape::sphere().with_transform(Transform::translation(5.0, 0.0, 0.0));
        g2.add_shape(s);
        g1.add_shape(g2);

        let shape = &g1.as_group().unwrap().children[0]
            .as_group()
            .unwrap()
            .children[0];
        let n = shape.normal_at(&Point::new(1.7321, 1.1547, -5.5774));
        assert_eq!(n, Some(Vector::new(0.28570, 0.42854, -0.85716)));
    }
}
