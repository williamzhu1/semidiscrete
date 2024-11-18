use jagua_rs::geometry::primitives::edge::Edge;
use jagua_rs::geometry::primitives::point::Point;

pub trait EdgeExtensions {
    /// Calculates the y-coordinate at a given x-coordinate on the edge.
    fn y_at_x(&self, x: f32) -> f32;

    /// Calculates the coefficient (slope) of the edge.
    fn coefficient(&self) -> f32;
}

impl EdgeExtensions for Edge {
    fn y_at_x(&self, x: f32) -> f32 {
        let Point(x1, y1) = self.start;
        let Point(x2, y2) = self.end;

        // Ensure x is within the bounds of the edge's x-range
        if x < f32::min(x1, x2) || x > f32::max(x1, x2) {
            return f32::NAN; // Return NaN to signify x is out of range
        }

        // Handle vertical line segments where the slope is undefined
        if x1 == x2 {
            return y1; // For vertical lines, return the y of any point (y1 == y2)
        }

        // Calculate the slope (m) of the line
        let m = (y2 - y1) / (x2 - x1);

        // Calculate the y-intercept (b)
        let b = y1 - m * x1;

        // Calculate and return y at the given x
        m * x + b
    }

    fn coefficient(&self) -> f32 {
        let Point(x1, y1) = self.start;
        let Point(x2, y2) = self.end;

        // Handle vertical line segments where the slope is undefined
        if (x2 - x1).abs() < f32::EPSILON {
            return f32::INFINITY; // Return infinity for vertical lines
        }

        // Calculate and return the slope (m)
        (y2 - y1) / (x2 - x1)
    }
}
