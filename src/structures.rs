use serde::{Deserialize, Serialize};
use pyo3::prelude::*;

/// STDF record types and structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdfRecord {
    pub record_type: u8,
    pub record_subtype: u8,
    pub data: Vec<u8>,
}

/// Wafer information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaferInfo {
    pub wafer_id: String,
    pub lot_id: String,
    pub start_time: u32,
    pub finish_time: u32,
}

/// Die coordinate and test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DieResult {
    pub x_coord: i16,
    pub y_coord: i16,
    pub bin_number: u16,
    pub pass_fail: bool,
    pub test_time: f32,
}

/// Test parameter result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterResult {
    pub test_number: u32,
    pub test_name: String,
    pub result: f64,
    pub units: String,
    pub low_limit: Option<f64>,
    pub high_limit: Option<f64>,
}

/// Complete wafer data with all test results
#[pyclass]
#[derive(Debug, Clone)]
pub struct WaferData {
    #[pyo3(get, set)]
    pub wafer_info: String,
    #[pyo3(get, set)]
    pub die_count: usize,
    #[pyo3(get, set)]
    pub pass_count: usize,
    #[pyo3(get, set)]
    pub fail_count: usize,
}

#[pymethods]
impl WaferData {
    #[new]
    pub fn new() -> Self {
        WaferData {
            wafer_info: String::new(),
            die_count: 0,
            pass_count: 0,
            fail_count: 0,
        }
    }
    
    pub fn yield_percentage(&self) -> f64 {
        if self.die_count == 0 {
            0.0
        } else {
            (self.pass_count as f64 / self.die_count as f64) * 100.0
        }
    }
}
