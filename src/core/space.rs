use hibitset as hb;

/// Opaque id type so they cannot be accidentally mutated and break things.
#[derive(Clone, Copy, Debug)]
pub struct Id(pub(crate) usize);

use super::container::ContainerAccess;
pub trait FeatureSet: 'static {
    fn init(capacity: usize) -> Self;
    fn containers(&mut self) -> Vec<&mut dyn ContainerAccess>;
    fn tick(&mut self, dt: f32);
}

//

pub struct Space<F: FeatureSet> {
    alive_objects: hb::BitSet,
    enabled_objects: hb::BitSet,
    next_obj_id: usize,
    capacity: usize,
    pub features: F,
}

impl<F: FeatureSet> Space<F> {
    pub fn with_capacity(capacity: usize) -> Self {
        Space {
            alive_objects: hb::BitSet::with_capacity(capacity as u32),
            enabled_objects: hb::BitSet::with_capacity(capacity as u32),
            next_obj_id: 0,
            capacity,
            features: F::init(capacity),
        }
    }

    /// Create a new object in this Space. An object does not do anything on its own;
    /// use SpaceFeatures to add functionality to it.
    /// Returns None if the Space is full.
    pub fn create_object(&mut self) -> Option<MasterObjectHandle<F>> {
        if self.next_obj_id < self.capacity {
            let id = self.next_obj_id;
            self.next_obj_id += 1;
            self.create_object_at(id);
            Some(MasterObjectHandle {
                id: Id(id),
                space: self,
            })
        } else {
            // find a dead object
            use hb::BitSetLike;
            match (!&self.alive_objects).iter().nth(0) {
                Some(id) if id < self.capacity as u32 => {
                    self.create_object_at(id as usize);
                    Some(MasterObjectHandle {
                        id: Id(id as usize),
                        space: self,
                    })
                }
                _ => None,
            }
        }
    }

    fn create_object_at(&mut self, id: usize) {
        self.alive_objects.add(id as u32);
        self.enabled_objects.add(id as u32);
    }

    // TODO: disabling objects (add this while adding pools)

    fn kill_object(&mut self, id: Id) {
        let id = id.0 as u32;
        self.alive_objects.remove(id);
        for container in self.features.containers() {
            container.users().remove(id);
        }
    }

    /// Create an object and immediately add some Features to it.
    pub fn create_object_with(
        &mut self,
        add_fn: impl FnOnce(Id, &mut F),
    ) -> Option<MasterObjectHandle<F>> {
        let mut handle = self.create_object()?;
        handle.manage_features(add_fn);
        Some(handle)
    }

    pub fn spawn<R: super::Recipe<F>>(&mut self, recipe: R) -> Option<MasterObjectHandle<F>> {
        self.create_object_with(|id, feat| recipe.spawn(id, feat))
    }

    /// Spawn objects described in a RON file into this Space.
    #[cfg(feature = "ron-recipes")]
    pub fn read_ron_file<R>(&mut self, file: std::fs::File) -> Result<(), ron::de::Error>
    where
        R: super::recipe::DeserializeRecipes<F>,
    {
        let mut reader = std::io::BufReader::new(file);
        let mut bytes = Vec::new();
        use std::io::Read;
        reader.read_to_end(&mut bytes)?;

        let mut deser = ron::de::Deserializer::from_bytes(bytes.as_slice())?;
        R::deserialize_into_space(&mut deser, self)
    }

    pub fn tick(&mut self, dt: f32) {
        self.features.tick(dt);
    }
}

pub struct MasterObjectHandle<'a, F: FeatureSet> {
    id: Id,
    space: &'a mut Space<F>,
}

impl<'a, F: FeatureSet> MasterObjectHandle<'a, F> {
    pub fn manage_features(&mut self, add_fn: impl FnOnce(Id, &mut F)) {
        add_fn(self.id, &mut self.space.features);
    }

    pub fn kill(&mut self) {
        self.space.kill_object(self.id);
    }
}
