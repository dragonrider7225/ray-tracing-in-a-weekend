use std::{
    fmt::{self, Debug, Formatter},
    ops::RangeInclusive,
    sync::Arc,
};

use crate::{
    ray::{Hittable, RayHit},
    Ray,
};

/// A list of multiple objects that could be hit by a ray.
#[derive(Clone, Default)]
pub struct List {
    objects: Vec<Arc<dyn Hittable>>,
}

impl List {
    /// Removes all objects from the list.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// Adds a new object to the back of the list.
    pub fn push(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Debug for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("HittableList")
            .field("len", &self.objects.len())
            .finish_non_exhaustive()
    }
}

impl Hittable for List {
    fn hit_by(&self, ray: &Ray, valid_t: RangeInclusive<f64>) -> Option<RayHit> {
        self.objects
            .iter()
            .fold(None, |acc, object| match (acc, object) {
                (None, object) => object.hit_by(ray, valid_t.clone()),
                (Some(acc), object) => object.hit_by(ray, *valid_t.start()..=acc.t).or(Some(acc)),
            })
    }
}
