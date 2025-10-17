use polars::prelude::*;
use anyhow::Result;

use crate::structures::*;

/// Convert STDF records to a Polars DataFrame
pub fn records_to_dataframe(records: Vec<StdfRecord>) -> Result<DataFrame> {
    let mut record_types = Vec::new();
    let mut record_subtypes = Vec::new();
    let mut data_lengths = Vec::new();
    
    for record in records {
        record_types.push(record.record_type);
        record_subtypes.push(record.record_subtype);
        data_lengths.push(record.data.len() as u32);
    }
    
    let df = DataFrame::new(vec![
        Series::new("record_type", record_types),
        Series::new("record_subtype", record_subtypes),
        Series::new("data_length", data_lengths),
    ])?;
    
    Ok(df)
}

/// Convert die results to a Polars DataFrame
pub fn die_results_to_dataframe(results: Vec<DieResult>) -> Result<DataFrame> {
    let mut x_coords = Vec::new();
    let mut y_coords = Vec::new();
    let mut bin_numbers = Vec::new();
    let mut pass_fails = Vec::new();
    let mut test_times = Vec::new();
    
    for result in results {
        x_coords.push(result.x_coord);
        y_coords.push(result.y_coord);
        bin_numbers.push(result.bin_number);
        pass_fails.push(result.pass_fail);
        test_times.push(result.test_time);
    }
    
    let df = DataFrame::new(vec![
        Series::new("x_coord", x_coords),
        Series::new("y_coord", y_coords),
        Series::new("bin_number", bin_numbers),
        Series::new("pass_fail", pass_fails),
        Series::new("test_time", test_times),
    ])?;
    
    Ok(df)
}

/// Convert parameter results to a Polars DataFrame
pub fn parameter_results_to_dataframe(results: Vec<ParameterResult>) -> Result<DataFrame> {
    let mut test_numbers = Vec::new();
    let mut test_names = Vec::new();
    let mut values = Vec::new();
    let mut units = Vec::new();
    
    for result in results {
        test_numbers.push(result.test_number);
        test_names.push(result.test_name);
        values.push(result.result);
        units.push(result.units);
    }
    
    let df = DataFrame::new(vec![
        Series::new("test_number", test_numbers),
        Series::new("test_name", test_names),
        Series::new("result", values),
        Series::new("units", units),
    ])?;
    
    Ok(df)
}
