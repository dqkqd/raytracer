use std::ops::Deref;

use crate::{
    intersect::intersection::IntersectionsFactor, point::Point, util::solve_linear_equation,
    vector::Vector,
};

use super::{shape::Shape, ShapeLocal};

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

impl ShapeLocal for Group {
    fn local_normal_at(&self, _: &Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
    fn local_intersection(&self, local_ray: &crate::ray::Ray) -> IntersectionsFactor {
        let a = local_ray.direction().y();
        let b = local_ray.origin().y();
        solve_linear_equation(a, b)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        ray::Ray,
        shapes::ShapeWorld,
        transform::{Transform, Transformable},
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
}
