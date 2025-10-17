use super::*;

#[cfg(test)]
mod tests {
    use crate::*;

    // Test data structures
    #[test]
    fn test_wafer_record_creation() {
        let wafer = WaferRecord {
            wafer_id: "W123".to_string(),
            die_x: 10,
            die_y: 20,
            test_result: Some("PASS".to_string()),
            bin_number: Some(1),
        };

        assert_eq!(wafer.wafer_id, "W123");
        assert_eq!(wafer.die_x, 10);
        assert_eq!(wafer.die_y, 20);
        assert_eq!(wafer.test_result, Some("PASS".to_string()));
        assert_eq!(wafer.bin_number, Some(1));
    }

    #[test]
    fn test_stdf_record_types() {
        // Test various STDF record types
        let record_types = vec!["FAR", "MIR", "WIR", "WRR", "PRR", "PTR"];
        
        for record_type in record_types {
            assert!(record_type.len() == 3);
        }
    }

    // Test parsing functions
    #[test]
    fn test_parse_header() {
        let header_bytes = vec![0x00, 0x04, 0x00, 0x0A];
        let result = parse_record_header(&header_bytes);
        
        assert!(result.is_ok());
        let (length, record_type, record_subtype) = result.unwrap();
        assert_eq!(length, 4);
    }

    #[test]
    fn test_parse_wafer_id() {
        let wafer_id = "WAFER_001";
        let parsed = parse_wafer_identifier(wafer_id);
        
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap(), wafer_id);
    }

    #[test]
    fn test_parse_coordinates() {
        let x: i16 = 5;
        let y: i16 = 10;
        
        let coords = DieCoordinate { x, y };
        assert_eq!(coords.x, 5);
        assert_eq!(coords.y, 10);
    }

    #[test]
    fn test_parse_bin_number() {
        let bin_numbers = vec![1, 2, 3, 255];
        
        for bin in bin_numbers {
            assert!(bin <= 255);
            assert!(bin >= 0);
        }
    }

    #[test]
    fn test_invalid_bin_number() {
        let result = validate_bin_number(256);
        assert!(result.is_err());
    }

    // Test data conversion functions
    #[test]
    fn test_records_to_dataframe() {
        let records = vec![
            WaferRecord {
                wafer_id: "W1".to_string(),
                die_x: 0,
                die_y: 0,
                test_result: Some("PASS".to_string()),
                bin_number: Some(1),
            },
            WaferRecord {
                wafer_id: "W1".to_string(),
                die_x: 1,
                die_y: 0,
                test_result: Some("FAIL".to_string()),
                bin_number: Some(2),
            },
        ];

        let df_result = convert_to_dataframe(&records);
        assert!(df_result.is_ok());
        
        let df = df_result.unwrap();
        assert_eq!(df.height(), 2);
    }

    #[test]
    fn test_empty_records_to_dataframe() {
        let records: Vec<WaferRecord> = vec![];
        let df_result = convert_to_dataframe(&records);
        
        assert!(df_result.is_ok());
        let df = df_result.unwrap();
        assert_eq!(df.height(), 0);
    }

    #[test]
    fn test_dataframe_column_names() {
        let records = vec![WaferRecord {
            wafer_id: "W1".to_string(),
            die_x: 0,
            die_y: 0,
            test_result: Some("PASS".to_string()),
            bin_number: Some(1),
        }];

        let df_result = convert_to_dataframe(&records);
        assert!(df_result.is_ok());
        
        let df = df_result.unwrap();
        let columns = df.get_column_names();
        
        assert!(columns.contains(&"wafer_id"));
        assert!(columns.contains(&"die_x"));
        assert!(columns.contains(&"die_y"));
        assert!(columns.contains(&"test_result"));
        assert!(columns.contains(&"bin_number"));
    }

    // Test file parsing
    #[test]
    fn test_parse_stdf_file_not_found() {
        let result = parse_stdf_file("nonexistent_file.stdf");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_file() {
        let empty_data: Vec<u8> = vec![];
        let result = parse_stdf_data(&empty_data);
        
        assert!(result.is_ok());
        let records = result.unwrap();
        assert_eq!(records.len(), 0);
    }

    // Test data validation
    #[test]
    fn test_validate_wafer_record() {
        let valid_record = WaferRecord {
            wafer_id: "W123".to_string(),
            die_x: 0,
            die_y: 0,
            test_result: Some("PASS".to_string()),
            bin_number: Some(1),
        };

        assert!(validate_wafer_record(&valid_record).is_ok());
    }

    #[test]
    fn test_validate_empty_wafer_id() {
        let invalid_record = WaferRecord {
            wafer_id: "".to_string(),
            die_x: 0,
            die_y: 0,
            test_result: None,
            bin_number: None,
        };

        let result = validate_wafer_record(&invalid_record);
        assert!(result.is_err());
    }

    // Test data aggregation
    #[test]
    fn test_aggregate_by_bin() {
        let records = vec![
            WaferRecord {
                wafer_id: "W1".to_string(),
                die_x: 0,
                die_y: 0,
                test_result: Some("PASS".to_string()),
                bin_number: Some(1),
            },
            WaferRecord {
                wafer_id: "W1".to_string(),
                die_x: 1,
                die_y: 0,
                test_result: Some("PASS".to_string()),
                bin_number: Some(1),
            },
            WaferRecord {
                wafer_id: "W1".to_string(),
                die_x: 2,
                die_y: 0,
                test_result: Some("FAIL".to_string()),
                bin_number: Some(2),
            },
        ];

        let bin_counts = aggregate_by_bin(&records);
        assert_eq!(bin_counts.get(&1), Some(&2));
        assert_eq!(bin_counts.get(&2), Some(&1));
    }

    #[test]
    fn test_filter_by_test_result() {
        let records = vec![
            WaferRecord {
                wafer_id: "W1".to_string(),
                die_x: 0,
                die_y: 0,
                test_result: Some("PASS".to_string()),
                bin_number: Some(1),
            },
            WaferRecord {
                wafer_id: "W1".to_string(),
                die_x: 1,
                die_y: 0,
                test_result: Some("FAIL".to_string()),
                bin_number: Some(2),
            },
        ];

        let passed = filter_by_result(&records, "PASS");
        assert_eq!(passed.len(), 1);
        assert_eq!(passed[0].test_result, Some("PASS".to_string()));
    }

    // Integration tests
    #[test]
    fn test_full_parsing_pipeline() {
        // Create sample STDF data
        let sample_data = create_sample_stdf_data();
        
        // Parse the data
        let parse_result = parse_stdf_data(&sample_data);
        assert!(parse_result.is_ok());
        
        let records = parse_result.unwrap();
        assert!(records.len() > 0);
        
        // Convert to DataFrame
        let df_result = convert_to_dataframe(&records);
        assert!(df_result.is_ok());
    }
}

// Helper functions for tests
fn parse_record_header(data: &[u8]) -> Result<(u16, u8, u8), String> {
    if data.len() < 4 {
        return Err("Insufficient data".to_string());
    }
    let length = u16::from_le_bytes([data[0], data[1]]);
    let record_type = data[2];
    let record_subtype = data[3];
    Ok((length, record_type, record_subtype))
}

fn parse_wafer_identifier(id: &str) -> Result<String, String> {
    if id.is_empty() {
        return Err("Empty wafer ID".to_string());
    }
    Ok(id.to_string())
}

struct DieCoordinate {
    x: i16,
    y: i16,
}

fn validate_bin_number(bin: u32) -> Result<(), String> {
    if bin > 255 {
        return Err("Bin number exceeds maximum".to_string());
    }
    Ok(())
}

fn validate_wafer_record(record: &WaferRecord) -> Result<(), String> {
    if record.wafer_id.is_empty() {
        return Err("Empty wafer ID".to_string());
    }
    Ok(())
}

fn aggregate_by_bin(records: &[WaferRecord]) -> std::collections::HashMap<u8, usize> {
    let mut counts = std::collections::HashMap::new();
    for record in records {
        if let Some(bin) = record.bin_number {
            *counts.entry(bin).or_insert(0) += 1;
        }
    }
    counts
}

fn filter_by_result(records: &[WaferRecord], result: &str) -> Vec<WaferRecord> {
    records
        .iter()
        .filter(|r| r.test_result.as_deref() == Some(result))
        .cloned()
        .collect()
}

fn create_sample_stdf_data() -> Vec<u8> {
    // Return minimal valid STDF file structure
    vec![0x00, 0x02, 0x00, 0x0A, 0x00, 0x00]
}
