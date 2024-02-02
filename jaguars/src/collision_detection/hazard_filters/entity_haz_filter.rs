use crate::collision_detection::hazard_filters::hazard_filter::HazardFilter;
use crate::collision_detection::hazard::HazardEntity;

pub struct EntityHazardFilter {
    entities: Vec<HazardEntity>,
}


impl HazardFilter for EntityHazardFilter {
    fn is_relevant(&self, entity: &HazardEntity) -> bool {
        !self.entities.contains(entity)
    }
}

impl EntityHazardFilter {
    pub fn new() -> Self {
        Self {
            entities: vec![],
        }
    }

    pub fn add(mut self, entity: HazardEntity) -> Self {
        self.entities.push(entity);
        self
    }


    pub fn entities(&self) -> &Vec<HazardEntity> {
        &self.entities
    }

    pub fn drain(self) -> Vec<HazardEntity> {
        self.entities
    }
}