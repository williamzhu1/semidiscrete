#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;
    use jagua_rs::geometry::primitives::point::Point;
    use jagua_rs::geometry::transformation::Transformation;
    use jagua_rs::geometry::geo_enums::AllowedRotation;
    use jagua_rs::util::config::SPSurrogateConfig;
    use jagua_rs::entities::item::{Item};
    use test_case::test_case;
    use sdr::discrete_item::Discretizable;

    /// Helper function to create a sample Item with a square shape
    fn create_sample_item(resolution: f32) -> Item {
        let shape = SimplePolygon::new(vec![
            Point(0.0, 0.0), // Bottom-left
            Point(2.0, 0.0), // Bottom-right
            Point(2.0, 2.0), // Top-right
            Point(0.0, 2.0), // Top-left
        ]);

        let allowed_rotation = AllowedRotation::None;
        let pretransform = Transformation::empty();
        let surrogate_config = SPSurrogateConfig {
            pole_coverage_goal: 0.9,
            max_poles: 10,
            n_ff_poles: 2,
            n_ff_piers: 0,
        };
        let base_quality = Some(100);

        Item::new(
            1,           // id
            shape,       // shape
            100,         // value
            allowed_rotation,
            pretransform,
            base_quality,
            surrogate_config,
        )
    }

    /// Test the discretization of the Item at different resolutions
    #[test_case(0.5; "resolution_0.5")]
    #[test_case(0.25; "resolution_0.25")]
    #[test_case(1.0; "resolution_1.0")]
    fn test_item_discretization(resolution: f32) {
        // Create an item
        let item = create_sample_item(resolution);

        // Discretize the shape
        let discretized_shape = item.discretize_shape(resolution);

        for (x_idx, segment) in discretized_shape.iter().enumerate() {
            println!("At x = {}:", x_idx);
            for (start_y, end_y, side) in segment {
                println!("  Segment: ({:.2}, {:.2}, {})", start_y, end_y, side);
            }
        }
        assert_eq!(discretized_shape.len(), 2, "Expected two rows for a square at resolution {}", resolution)

        // Additional assertions can be added for specific edge cases if needed
    }
}
