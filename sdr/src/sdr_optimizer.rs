use std::cmp::{Ordering, Reverse};
use std::time::Instant;

use itertools::Itertools;
use log::{debug, info};
use ordered_float::NotNan;
use rand::prelude::SmallRng;
use rand::Rng;
use thousands::Separable;

use jagua_rs::collision_detection::hazard_filter;
use jagua_rs::entities::instances::instance::{self, Instance};
use jagua_rs::entities::instances::instance_generic::InstanceGeneric;
use jagua_rs::entities::item::Item;
use jagua_rs::entities::layout::Layout;
use jagua_rs::entities::placing_option::PlacingOption;
use jagua_rs::entities::problems::bin_packing::BPProblem;
use jagua_rs::entities::problems::problem::{self, Problem};
use jagua_rs::entities::problems::problem_generic::{LayoutIndex, ProblemGeneric};
use jagua_rs::entities::problems::strip_packing::SPProblem;
use jagua_rs::entities::solution::Solution;
use jagua_rs::fsize;
use jagua_rs::geometry::convex_hull::convex_hull_from_points;
use jagua_rs::geometry::geo_traits::{Shape, TransformableFrom};
use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;

use crate::sdr_config::SDRConfig;
use crate::sdr_cost::SDRPlacingCost;

//limits the number of items to be placed, for debugging purposes
pub const ITEM_LIMIT: usize = usize::MAX;

pub struct SDROptimizer{
    pub instance: Instance,
    pub problem: Problem,
    pub config: SDRConfig,
    pub sample_counter: usize,
}

impl SDROptimizer{

}