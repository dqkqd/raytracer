pub(crate) trait Vec4 {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

macro_rules! apply_equal_vec4 {
    ($struct:ident) => {
        impl PartialEq for $struct {
            fn eq(&self, other: &Self) -> bool {
                $crate::util::equal(self.x(), other.x())
                    && $crate::util::equal(self.y(), other.y())
                    && $crate::util::equal(self.z(), other.z())
                    && $crate::util::equal(self.w(), other.w())
            }
        }
    };
}

pub(crate) use apply_equal_vec4;
