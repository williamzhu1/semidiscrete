#[cfg(test)]
mod tests {
    use jagua_rs::entities::instances::instance_generic::InstanceGeneric;
    use jagua_rs::geometry::primitives::simple_polygon::SimplePolygon;
    use jagua_rs::geometry::primitives::point::Point;
    use jagua_rs::geometry::transformation::Transformation;
    use jagua_rs::geometry::geo_enums::AllowedRotation;
    use jagua_rs::util::config::SPSurrogateConfig;
    use jagua_rs::entities::item::Item;
    use jagua_rs::io::parser::Parser;
    use sdr::sdr_config::SDRConfig;
    use test_case::test_case;
    use sdr::discrete_item::Discretizable;
    use sdr::io;
    use svg::node::element::path::Data;
    use svg::node::element::{Path, Line};
    use svg::Document;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path as OtherPath;
    use jagua_rs::util::polygon_simplification::PolySimplConfig;
    use std::time::{Instant, Duration};

    /// Helper function to create a sample Item with a square shape
    fn create_sample_item() -> Item {
        let shape = SimplePolygon::new(vec![
            Point(0.0, 0.0),
            Point(8.0, 0.0),
            Point(8.0, 2.0),
            Point(6.0, 2.0),
            Point(4.0, 1.0),
            Point(3.0, 1.0),
            Point(2.0, 2.0),
            Point(-1.0, 2.0),
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
    // This is the function you showed previously for generating path data from a polygon
    fn simple_polygon_data(s_poly: &SimplePolygon) -> Data {
        let mut data = Data::new().move_to((s_poly.get_point(0).0, s_poly.get_point(0).1));
        for i in 1..s_poly.number_of_points() {
            let p = s_poly.get_point(i);
            data = data.line_to((p.0, p.1));
        }
        data.close()
    }


    /// Test the discretization of the Item at different resolutions
    #[test_case(1.0; "resolution_1")]
    #[test_case(0.5; "resolution_0.5")]
    #[test_case(0.1; "resolution_1.0")]
    fn test_item_discretization(resolution: f32) {
        // Create an item
        let item = create_sample_item();

        // Discretize the shape
        let discretized_shape = item.discretize_shape(resolution);

        let discretized_clone = discretized_shape.clone();
        for (x_line, segments) in discretized_clone {
            println!("x = {}:", x_line);
            for (y_start, y_end) in segments {
                println!("  segment: y = {} to {}", y_start, y_end);
            }
        }


        let width = 800;
        let height = 600;
        let polygon_data = simple_polygon_data(&item.shape);
        let polygon_path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.05)
            .set("d", polygon_data);
        let mut document = Document::new()
            .set("viewBox", (0, 0, 8, 8))
            .set("width", width)
            .set("height", height)
            .add(polygon_path);
        for (x_line, segments) in discretized_shape {
            for (y_start, y_end) in segments {
                let line = Line::new()
                    .set("x1", x_line)
                    .set("y1", y_start)
                    .set("x2", x_line)
                    .set("y2", y_end)
                    .set("stroke", "red")
                    .set("stroke-width", 0.02);
                document = document.add(line);
            }
        }

        // Save to file
        let mut file = File::create("discretized_shape.svg").expect("Unable to create file");
        file.write_all(document.to_string().as_bytes()).expect("Unable to write SVG data");

        println!("SVG generated and saved as discretized_shape.svg");
    }

    #[test]
    fn print_shape(){
        // Create a sample polygon (e.g., a simple rectangle or any shape)
        let polygon = SimplePolygon::new(vec![
            Point(0.0, 0.0),
            Point(7.0, 1.0),
            Point(7.0, 5.0),
            Point(0.0, 7.0),
            Point(-1.0, 5.0),
            Point(-1.0, 4.0),
            Point(-2.0, 3.0),
            Point(-1.0, 2.0),
        ]);

        // Generate SVG path data for the polygon
        let data = simple_polygon_data(&polygon);

        // Create an SVG path from the data
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.05)
            .set("d", data);

        // Determine a suitable viewport
        // Since the polygon spans from (0,0) to (4,3), we can set the viewBox accordingly
        let width = 400;
        let height = 300;
        let document = Document::new()
            .set("viewBox", (0, 0, 7, 7)) // scaling to actual polygon dimensions
            .set("width", width)
            .set("height", height)
            .add(path);

        // Save to file
        let mut file = File::create("item_polygon.svg").unwrap();
        file.write_all(document.to_string().as_bytes()).unwrap();

        println!("SVG generated and saved as item_polygon.svg");

    }

    #[test_case("../assets/swim.json"; "swim")]
    #[test_case("../assets/shirts.json"; "shirts")]
    #[test_case("../assets/trousers.json"; "trousers")]
    #[test_case("../assets/mao.json"; "mao")]
    fn test_instance(instance_path: &str) {
        let instance = OtherPath::new(instance_path);
        // parse the instance
        let mut config = SDRConfig::default();
        config.n_samples = 100;
        let json_instance = io::read_json_instance(&instance);
        let poly_simpl_config = match config.poly_simpl_tolerance {
            Some(tolerance) => PolySimplConfig::Enabled { tolerance },
            None => PolySimplConfig::Disabled,
        };

        let parser = Parser::new(poly_simpl_config, config.cde_config, true);
        let instance = parser.parse(&json_instance);
        
        let mut totalDuration = Duration::new(0, 0);
        for (i, item) in instance.items().iter().enumerate() {
            let start = Instant::now();
            let discretized_shape = item.0.discretize_shape(0.5);
            println!("item segments: {:?}", discretized_shape);
            let duration = start.elapsed();
            totalDuration += duration;
            let polygon_data = simple_polygon_data(&item.0.shape);
             // Obtain the bounding box from the polygon's shape
             let bbox = &item.0.shape.bbox;
             let width = bbox.width();
             let height = bbox.height();
            // Add some margin (e.g., 10%)
        let margin_x = width * 0.1;
        let margin_y = height * 0.1;

        // Calculate the viewBox coordinates
        let view_x = bbox.x_min - margin_x;
        let view_y = bbox.y_min - margin_y;
        let view_w = width + 2.0 * margin_x;
        let view_h = height + 2.0 * margin_y;

        // Create polygon path
        let polygon_path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.05)
            .set("d", polygon_data);

        // Use the dynamically computed viewBox here instead of a fixed one
        let mut document = Document::new()
            .set("viewBox", (view_x, view_y, view_w, view_h))
            // The width and height of the rendered SVG can remain fixed or be adjusted.
            // Using the same width/height in pixels is fine; SVG will scale to viewBox.
            .set("width", 800)
            .set("height", 600)
            .add(polygon_path);

            for (x_line, segments) in discretized_shape {
                for (y_start, y_end) in segments {
                    let line = Line::new()
                        .set("x1", x_line)
                        .set("y1", y_start)
                        .set("x2", x_line)
                        .set("y2", y_end)
                        .set("stroke", "red")
                        .set("stroke-width", 0.05);
                    document = document.add(line);
                }
            }

            // Save to file
            let filename = format!("discretized_shape_{}.svg", i);
            let mut file = File::create(&filename).expect("Unable to create file");
            file.write_all(document.to_string().as_bytes()).expect("Unable to write SVG data");

            println!("SVG generated and saved as discretized_shape_{}.svg", i);
        }

        println!("Total duration in micross: {} micross", totalDuration.as_micros());
    }
}
