use crate::math::SphericalCoordinates;

pub struct State {
    pub position: SphericalCoordinates,
    pub input_mode_enabled: bool,
    pub expression: String,
    pub render_distance: i32,
    pub root_splits: i32,
    pub root_find_tries: i32,
    pub sample_size: i32,
    pub distance_draw_cutoff: f32,
}

impl State {
    pub fn new() -> State {
        State {
            position: SphericalCoordinates::from_distance(10.),
            input_mode_enabled: false,
            expression: String::new(),
            render_distance: 100,
            root_splits: 100,
            root_find_tries: 10,
            sample_size: 1000,
            distance_draw_cutoff: 40.,
        }
    }
}
