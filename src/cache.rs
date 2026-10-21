use crate::function::compute_points as compute_points_no_cache;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
struct ComputePointsIn {
    expression: String,
    render_distance: i32,
    sample_size: i32,
    root_splits: i32,
    root_tries: i32,
}

pub struct Cache {
    compute_points_hm: HashMap<ComputePointsIn, Vec<(f32, f32)>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            compute_points_hm: HashMap::new(),
        }
    }

    pub fn compute_points(
        &mut self,
        expression: String,
        render_distance: i32,
        sample_size: i32,
        root_splits: i32,
        root_tries: i32,
    ) -> &Vec<(f32, f32)> {
        let input = ComputePointsIn {
            expression: expression.clone(),
            render_distance,
            sample_size,
            root_splits,
            root_tries,
        };
        self.compute_points_hm.entry(input).or_insert_with(|| {
            compute_points_no_cache(
                expression,
                render_distance,
                sample_size,
                root_splits,
                root_tries,
            )
        })
    }
}
