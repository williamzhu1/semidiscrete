use jagua_rs::entities::item::Item;
use jagua_rs::geometry::primitives::edge::Edge;
use crate::edge_extension::EdgeExtensions;

// Define a trait to be implemented for Item
pub trait Discretizable {
    fn discretize_shape(&self, resolution: f32) -> Vec<Vec<(f32, f32, i32)>>;
}

// Implement the trait for Item
impl Discretizable for Item {
    fn discretize_shape(&self, resolution: f32) -> Vec<Vec<(f32, f32, i32)>> {
        let mut edges: Vec<Edge> = self.shape.edge_iter().collect(); 
        edges.sort_by(|a, b| a.start.0.partial_cmp(&b.start.0).unwrap());

        let mut active_edges: Vec<Edge> = Vec::new();
        let mut raw_discretized_shape: Vec<Vec<(f32, f32, i32)>> = Vec::new();

        let mut x_idx = 0;
        let mut x = x_idx as f32 * resolution;

        while x <= self.shape.diameter + f32::EPSILON {
            let mut discretized_segment: Vec<(f32, f32, i32)> = Vec::new();
            let mut vertical_edges: Vec<Edge> = Vec::new();
            let mut need_sort = false;
            edges.retain(|edge| {
                if edge.start.0 <= x + f32::EPSILON {
                    if edge.coefficient().is_finite() {
                        if edge.end.0 > x {
                            active_edges.push(edge.clone());
                            need_sort = true;
                        }
                    } else if edge.start.0 == x {
                        vertical_edges.push(edge.clone());
                    }
                    false // Remove processed edge
                } else {
                    true // Keep unprocessed edge
                }
            });

            if need_sort {
                active_edges.sort_by(|a, b| a.start.1.partial_cmp(&b.start.1).unwrap());
            }

            vertical_edges.sort_by(|a, b| a.start.1.partial_cmp(&b.start.1).unwrap());

            let mut temp_y = -1.0;
            let mut active_iter = active_edges.iter();
            let mut vertical_iter = vertical_edges.iter();

            while let Some(active_edge) = active_iter.next() {
                let y = active_edge.y_at_x(x);

                if let Some(vertical_edge) = vertical_iter.clone().next() {
                    if (y - vertical_edge.start.1).abs() < f32::EPSILON {
                        if active_edge.coefficient() == 1.0 {
                            if temp_y == -1.0 {
                                temp_y = y;
                            } else {
                                discretized_segment.push((temp_y, y, 0));
                                temp_y = -1.0;
                            }
                        }

                        let side = if active_edge.end.1 == x { -1 } else { 1 } * active_edge.coefficient().round() as i32;
                        discretized_segment.push((y, vertical_edge.end.1, side));

                        vertical_iter.next();
                    }
                } else {
                    if (temp_y - y).abs() < f32::EPSILON && ((active_edge.end.1 - active_edge.start.1) / (active_edge.end.0 - active_edge.start.0))
                        != active_edges.iter().rev().next().map_or(0.0, |e| (e.end.1 - e.start.1) / (e.end.0 - e.start.0))
                    {
                        let side = if active_edge.start.0 == x { -1 } else { 1 } * active_edge.coefficient().round() as i32;
                        discretized_segment.push((y, y, side));
                        temp_y = -1.0;
                    } else {
                        let last_y = discretized_segment.last().map_or(-1.0, |(_, end_y, _)| *end_y);
                        if last_y != y || active_edge.coefficient() != active_edges.iter().rev().next().map_or(0.0, |e| e.coefficient()) {
                            if temp_y == -1.0 {
                                temp_y = y;
                            } else {
                                discretized_segment.push((temp_y, y, 0));
                                temp_y = -1.0;
                            }
                        }
                    }
                }
            }

            raw_discretized_shape.push(discretized_segment);
            x_idx += 1;
            x = resolution * x_idx as f32;

            active_edges.retain(|edge| edge.end.0 + f32::EPSILON >= x);
        }

        raw_discretized_shape
    }
}
