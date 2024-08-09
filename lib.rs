use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

#[derive(Debug, Copy, Clone)]
struct ForcedOrdF32(f32);

impl Hash for ForcedOrdF32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        assert!(!self.0.is_nan(), "NaN found!");
        self.0.to_bits().hash(state);
    }
}
impl PartialEq for ForcedOrdF32 {
    fn eq(&self, other: &Self) -> bool {
        assert!(!self.0.is_nan() && !other.0.is_nan(), "NaN found!");
        self.0 == other.0
    }
}

impl Eq for ForcedOrdF32 {}

impl PartialOrd for ForcedOrdF32 {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        assert!(!self.0.is_nan() && !other.0.is_nan(), "NaN found!");
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for ForcedOrdF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[pyclass]
struct LogitDatabase {
    storage: Mutex<HashMap<String, Vec<f32>>>,
}

#[pymethods]
impl LogitDatabase {
    #[new]
    fn new() -> Self {
        LogitDatabase {
            storage: Mutex::new(HashMap::new()),
        }
    }

    fn insert(&self, key: String, vector: Vec<f32>) -> PyResult<()> {
        let mut storage = self.storage.lock().unwrap();
        storage.insert(key, vector);
        Ok(())
    }

    fn get(&self, key: String) -> PyResult<Option<Vec<f32>>> {
        let storage = self.storage.lock().unwrap();
        Ok(storage.get(&key).cloned())
    }

    fn delete(&self, key: String) -> PyResult<()> {
        let mut storage = self.storage.lock().unwrap();
        storage.remove(&key);
        Ok(())
    }

    fn nearest_neighbor(&self, query: Vec<f32>) -> Option<(String, f32)> {
        let storage = self.storage.lock().unwrap();
        let mut nearest: Option<(String, f32)> = None;
        for (key, vector) in storage.iter() {
            let distance = euclidean_distance(&query, vector);
            if let Some((_, nearest_dist)) = &nearest {
                if &distance < nearest_dist {
                    nearest = Some((key.clone(), distance));
                }
            } else {
                nearest = Some((key.clone(), distance));
            }
        }
        nearest
    }
    fn nearest_neighbors(&self, query: Vec<f32>, k: usize) -> Vec<(String, f32)> {
        let storage = self.storage.lock().unwrap();
        let mut nearest = BinaryHeap::with_capacity(k);
        let mut farthest = ForcedOrdF32(f32::INFINITY);
        for (key, vector) in storage.iter() {
            let distance = ForcedOrdF32(euclidean_distance(&query, vector));
            let count = nearest.len();
            if count < k {
                nearest.push((distance, key));
                farthest = nearest.peek().unwrap().0;
            } else if farthest > distance {
                nearest.pop();
                nearest.push((distance, key));
                farthest = nearest.peek().unwrap().0;
            }
        }

        let mut results = nearest.into_sorted_vec();
        results.reverse();
        results.into_iter().map(|(x, y)| (y.clone(), x.0)).collect()
    }
}

fn euclidean_distance(vec1: &[f32], vec2: &[f32]) -> f32 {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x1, x2)| (x1 - x2).powi(2))
        .sum::<f32>()
        .sqrt()
}

#[pyfunction]
fn create_db() -> LogitDatabase {
    LogitDatabase::new()
}

#[pymodule]
fn logit(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<LogitDatabase>()?;
    m.add_function(wrap_pyfunction!(create_db, m)?)?;
    Ok(())
}
