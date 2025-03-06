use std::{collections::HashSet, f32::EPSILON};

use jagua_rs::entities::item::Item;

use crate::approx_eq::ApproxEq;


// Define a trait to be implemented for Item
pub trait Discretizable {
    fn discretize_shape(&self, resolution: f32) -> Vec<(f32, Vec<(f32, f32)>)>;
    fn intersect_vertical_line(&self, x_line: f32) -> Vec<(f32, f32)>;
}

// Implement the trait for Item
impl Discretizable for Item {
    fn intersect_vertical_line(&self, x_line: f32) -> Vec<(f32, f32)> {
        let mut intersections = Vec::new();
        let mut last_intersection: Option<f32> = None;
        let mut unique_intersections = HashSet::new(); // Store unpaired start

        for edge in self.shape.edge_iter() {
            let (x_low, x_high) = if edge.start.0 < edge.end.0 {
                (edge.start.0, edge.end.0)
            } else {
                (edge.end.0, edge.start.0)
            };
            if x_line < x_low || x_line > x_high {
                continue;
            }

            let t = (x_line - edge.start.0) / (edge.end.0 - edge.start.0);
            if t >= 0.0 && t <= 1.0 {
                let y_intersect = edge.start.1 + t * (edge.end.1 - edge.start.1);

                let approx_intersect = ApproxEq::from(y_intersect); // Approximate equality
                if !unique_intersections.contains(&approx_intersect) {
                    unique_intersections.insert(approx_intersect);
                    
                    if let Some(last_y_val) = last_intersection {
                        intersections.push((last_y_val, y_intersect));
                        last_intersection = None; // Reset for next pair
                    } else {
                        last_intersection = Some(y_intersect);
                    }
                }
            }
        }

        // **Final Merge Pass**
        intersections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut merged_segments = Vec::new();
        
        if !intersections.is_empty() {
            let mut current_segment = intersections[0];

            for &segment in intersections.iter().skip(1) {
                if (current_segment.1 - segment.0).abs() < EPSILON {
                    current_segment.1 = segment.1; // Merge
                } else {
                    merged_segments.push(current_segment);
                    current_segment = segment;
                }
            }
            merged_segments.push(current_segment);
        }

        merged_segments
    }
    fn discretize_shape(&self, resolution: f32) -> Vec<(f32, Vec<(f32, f32)>)> {
        let rect = &self.shape.bbox;
        let mut results = Vec::new();

        let mut x_line = rect.x_min;
        while x_line <= rect.x_max + EPSILON {
            let ys = self.intersect_vertical_line(x_line);

            if !ys.is_empty() {
                results.push((x_line, ys));
            }

            x_line += resolution;
        }

        results
    }
}
