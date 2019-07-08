use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde_json as json;

use iscc_rs::{content_id_image, content_id_mixed, content_id_text, data_id, instance_id, meta_id};

/// Returns the path of a given file in the test_data directory.
fn get_path(filename: &str) -> String {
    let path: PathBuf = ["tests", "test_data", filename].iter().collect();
    path.to_string_lossy().into_owned()
}

// TODO: Maybe refactor with macros.

fn test_meta_id(test_name: &str, inputs: &json::Value, outputs: &json::Value) {
    let title = inputs[0].as_str().unwrap();
    let extra = inputs[1].as_str().unwrap();
    let mid_expected = outputs[0].as_str().unwrap();
    let title_expected = outputs[1].as_str().unwrap();
    let extra_expected = outputs[2].as_str().unwrap();

    let (mid, title, extra) = meta_id(title, extra);
    assert_eq!(
        (mid.as_ref(), title.as_ref(), extra.as_ref()),
        (mid_expected, title_expected, extra_expected),
        "test {} failed",
        test_name
    );
}

fn test_content_id_text(test_name: &str, inputs: &json::Value, outputs: &json::Value) {
    let text = inputs[0].as_str().unwrap();
    let partial = inputs[1].as_bool().unwrap();
    let expected_result = outputs.as_str().unwrap();

    let result = content_id_text(text, partial);
    assert_eq!(result, expected_result, "test {} failed", test_name);
}

fn test_content_id_image(test_name: &str, inputs: &json::Value, outputs: &json::Value) {
    let filename = inputs[0].as_str().unwrap();
    let partial = inputs[1].as_bool().unwrap();
    let expected_result = outputs.as_str().unwrap();

    let result = content_id_image(&get_path(filename), partial).unwrap();
    assert_eq!(result, expected_result, "test {} failed", test_name);
}

fn test_content_id_mixed(test_name: &str, inputs: &json::Value, outputs: &json::Value) {
    let cids: Vec<&str> = inputs[0]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    let partial = inputs[1].as_bool().unwrap();
    let expected_result = outputs.as_str().unwrap();

    let result = content_id_mixed(&cids, partial);
    assert_eq!(result, expected_result, "test {} failed", test_name);
}

fn test_data_id(test_name: &str, inputs: &json::Value, outputs: &json::Value) {
    let filename = inputs[0].as_str().unwrap();
    let expected_result = outputs.as_str().unwrap();

    let result = data_id(&get_path(filename)).unwrap();
    assert_eq!(result, expected_result, "test {} failed", test_name);
}

fn test_instance_id(test_name: &str, inputs: &json::Value, outputs: &json::Value) {
    let filename = inputs[0].as_str().unwrap();
    let code_expected = outputs[0].as_str().unwrap();
    let hex_hash_expected = outputs[1].as_str().unwrap();

    let (code, hex_hash) = instance_id(&get_path(filename)).unwrap();
    assert_eq!(
        (code.as_ref(), hex_hash.as_ref()),
        (code_expected, hex_hash_expected),
        "test {} failed",
        test_name
    );
}

#[test]
fn test_conformance_with_test_data() {
    let file_content = fs::read_to_string(&get_path("test_data.json")).unwrap();
    let test_data: HashMap<String, HashMap<String, json::Value>> =
        json::from_str(&file_content).unwrap();
    for (function_name, mut value) in test_data {
        let required = value.remove("required").unwrap();
        if required == false {
            continue;
        }
        for (test_name, test_data) in value {
            let inputs = &test_data["inputs"];
            let outputs = &test_data["outputs"];
            match function_name.as_ref() {
                "meta_id" => test_meta_id(&test_name, inputs, outputs),
                "content_id_text" => test_content_id_text(&test_name, inputs, outputs),
                "content_id_image" => test_content_id_image(&test_name, inputs, outputs),
                "content_id_mixed" => test_content_id_mixed(&test_name, inputs, outputs),
                "data_id" => test_data_id(&test_name, inputs, outputs),
                "instance_id" => test_instance_id(&test_name, inputs, outputs),
                _ => panic!("Unexpected function {}", function_name),
            };
        }
    }
}
