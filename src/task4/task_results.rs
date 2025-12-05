#[derive(Debug)]
pub struct GraphResults {
    pub directed: bool,
    pub deg_in: Vec<usize>,
    pub deg_out: Vec<usize>,
    pub weak_components: Vec<Vec<usize>>,
    pub strong_components: Vec<Vec<usize>>,

    pub selected_pairs: Vec<(usize, usize)>,

    pub distances: DistancesMatrix,
    pub paths: PathsMatrix,

    pub diameter: Option<f64>,
    pub radius: Option<f64>,
    pub centers: Vec<usize>,
    pub periphery: Vec<usize>,
}

pub type Distance = Option<f64>; // None = +Infinity
pub type Path = Option<Vec<usize>>; // None = no path
pub type DistancesMatrix = Vec<Vec<Distance>>;
pub type PathsMatrix = Vec<Vec<Path>>;

impl GraphResults {
    pub fn compute_graph_metrics(&mut self) {
        let n = self.distances.len();
        let mut eccentricities = vec![0.0; n];

        (0..n).for_each(|u| {
            let mut max_dist: f64 = 0.0;
            let mut has_infinite = false;

            for v in 0..n {
                if u != v {
                    match self.distances[u][v] {
                        Some(d) => max_dist = max_dist.max(d),
                        None => has_infinite = true,
                    }
                }
            }

            eccentricities[u] = if has_infinite {
                f64::INFINITY
            } else {
                max_dist
            };
        });

        self.diameter = Some(eccentricities.iter().copied().fold(0.0, f64::max));
        self.radius = Some(eccentricities.iter().copied().fold(f64::INFINITY, f64::min));

        self.centers = Self::filter_vertices_by_value(&eccentricities, self.radius);
        self.periphery = Self::filter_vertices_by_value(&eccentricities, self.diameter);
    }

    fn filter_vertices_by_value(eccentricities: &[f64], value: Option<f64>) -> Vec<usize> {
        eccentricities
            .iter()
            .enumerate()
            .filter_map(|(i, &e)| match value {
                Some(v) if v.is_infinite() && e.is_infinite() => Some(i),
                Some(v) if (e - v).abs() < 1e-12 => Some(i),
                _ => None,
            })
            .collect()
    }
}
