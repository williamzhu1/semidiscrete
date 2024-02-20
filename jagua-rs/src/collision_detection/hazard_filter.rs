use itertools::Itertools;

use crate::collision_detection::hazard::Hazard;
use crate::collision_detection::hazard::HazardEntity;

/// Trait that allows for filtering out irrelevant hazards depending on the context.
/// Enables ignoring certain hazards when querying the CDE.
pub trait HazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool;
}


/// Returns the entities that are deemed irrelevant by the given filter from a set of `Hazard`s
pub fn get_irrelevant_hazard_entities<'a>(filter: &impl HazardFilter, hazards: impl Iterator<Item=&'a Hazard>) -> Vec<HazardEntity> {
    hazards.filter_map(|h| {
        match filter.is_irrelevant(&h.entity){
            true => Some(h.entity.clone()),
            false => None
        }
    }).collect_vec()
}

/// Deems all hazards induced by the `Bin` as irrelevant.
#[derive(Clone)]
pub struct BinHazardFilter;

/// Deems hazards induced by `QualityZone`s above a certain quality as irrelevant
#[derive(Clone, Debug)]
pub struct QZHazardFilter {
    pub cutoff_quality: usize,
}

/// Combines multiple `HazardFilter`s into a single filter
pub struct CombinedHazardFilter<'a> {
    pub filters: Vec<Box<&'a dyn HazardFilter>>,
}

/// Deems hazards induced by specific entities as irrelevant
pub struct EntityHazardFilter {
    pub entities: Vec<HazardEntity>,
}

impl HazardFilter for BinHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::PlacedItem(_) => false,
            HazardEntity::BinExterior => true,
            HazardEntity::BinHole { .. } => true,
            HazardEntity::QualityZoneInferior { .. } => true,
        }
    }
}

impl<'a> HazardFilter for CombinedHazardFilter<'a> {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        self.filters.iter()
            .any(|f| f.is_irrelevant(entity))
    }
}

impl HazardFilter for EntityHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        self.entities.contains(entity)
    }
}

impl HazardFilter for QZHazardFilter {
    fn is_irrelevant(&self, entity: &HazardEntity) -> bool {
        match entity {
            HazardEntity::QualityZoneInferior { quality, .. } => *quality >= self.cutoff_quality,
            _ => false,
        }
    }
}