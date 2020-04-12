use nannou::prelude::*;
use rand::prelude::*;
use rand_distr::StandardNormal;

pub trait CommonPoint2<S: nalgebra::RealField> {
    // Todo: use associated type?
    fn new(x: S, y: S) -> Self;
    fn get_x(&self) -> S;
    fn get_y(&self) -> S;

    fn into_nannou(&self) -> Point2<S> {
        pt2(self.get_x(), self.get_y())
    }

    fn into_nalgebra(&self) -> nalgebra::Point2<S> {
        nalgebra::Point2::new(self.get_x(), self.get_y())
    }
}

impl<S: nalgebra::RealField> CommonPoint2<S> for Point2<S> {
    fn new(x: S, y: S) -> Self {
        pt2(x, y)
    }
    fn get_x(&self) -> S {
        self.x
    }
    fn get_y(&self) -> S {
        self.y
    }
}

impl<S: nalgebra::RealField> CommonPoint2<S> for nalgebra::Point2<S> {
    fn new(x: S, y: S) -> Self {
        nalgebra::Point2::new(x, y)
    }
    fn get_x(&self) -> S {
        self.coords.as_slice()[0]
    }
    // Todo: use index instead?
    fn get_y(&self) -> S {
        self.coords.as_slice()[1]
    }
}

/// Creates n_verts of a circle centered around 0 with random deviations in
/// radius and angle for each point, the points are not closed
pub fn rand_poly<T>(
    n_verts: usize,
    mean_rad: f32,
    std_rad: f32,
    std_phase: f32,
) -> impl Iterator<Item = T>
where
    T: CommonPoint2<f32>,
{
    let mut rng = thread_rng();

    assert!(n_verts >= 2);
    assert!(mean_rad > 0.0);

    let mut last_phase = 0.0f32;
    let points = (0..n_verts - 1).map(move |i| {
        // angle
        let fract: f32 = i as f32 / n_verts as f32; // one less for closing
        let rand_phase: f32 = rng.sample(StandardNormal);
        let phase = (fract + rand_phase * std_phase).max(last_phase);
        last_phase = phase;

        // radius
        let rand_rad: f32 = rng.sample(StandardNormal);
        let rad: f32 = (mean_rad + rand_rad * std_rad).max(0.0f32);

        let x = rad * (TAU * phase).cos();
        let y = rad * (TAU * phase).sin();

        T::new(x, y)
    });

    points
}
