use jaguars::util::config::{CDEConfig, HazProxConfig, QuadTreeConfig, SPSurrogateConfig};
use serde::{Deserialize, Serialize};
use jaguars::simplification::simplification_config::PolySimplConfig;
use crate::io::svg_util::SvgDrawOptions;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Config{
    pub cde_config: CDEConfig,
    pub poly_simpl_config: PolySimplConfig,
    pub deterministic_mode: bool,
    pub n_samples_per_item: usize,
    pub ls_samples_fraction: f32,
    #[serde(default)]
    pub svg_draw_options: SvgDrawOptions,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cde_config: CDEConfig{
                quadtree: QuadTreeConfig::FixedDepth(4),
                haz_prox: HazProxConfig::Enabled{n_cells: 10000},
                item_surrogate_config: SPSurrogateConfig{
                    pole_coverage_goal: 0.9,
                    max_poles: 10,
                    n_ff_poles: 1,
                    n_ff_piers: 1,
                }
            },
            poly_simpl_config: PolySimplConfig::Enabled {
                tolerance: 0.001
            },
            deterministic_mode: true,
            n_samples_per_item: 5000,
            ls_samples_fraction: 0.2,
            svg_draw_options: SvgDrawOptions::default(),
        }
    }
}