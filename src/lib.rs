use std::any::Any;

use resource::Resource;

pub mod resource;

#[derive(Default)]
pub struct World {
    resources: Resource,
}

impl World {
    pub fn add_resource(&mut self, resource_data: impl Any) {
        self.resources.add(resource_data);
    }
    pub fn get_resource<T: Any>(&self) -> Option<&T> {
        self.resources.get_ref::<T>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
