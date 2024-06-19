use crate::disk::list_files::list_files_at_root;

use std::path::Path;
use std::prelude::v1::Result as V1Result;
use serde_json::Value;
use std::time::SystemTime;
use serde_json::json;


pub fn search_keyword_in_files(keyword: &str, filepath: &Path) -> Result<Value, String> {
    let keyword: String = keyword.to_lowercase();


    let start_time = SystemTime::now();    
        

    let files: Vec<Value> = match list_files_at_root(filepath) {
        Ok(files) => files,
        Err(e) => return Err(e.to_string()),
    };

    println!("Searching for keyword: {}", keyword);
    println!("Files: {:?}", files);



    let mut matching_files = Vec::new();

    for file in files {
        let filename: String = match file["filename"].as_str() {
            Some(name) => name.to_lowercase(),
            None => continue,
        };

        if filename.contains(&keyword) {
            println!("Found matching file: {:?}", file);
            matching_files.push(file);
        }
    }
    let duration = SystemTime::now().duration_since(start_time).expect("Time went backwards");
    let amount_results = matching_files.len();
    let items_per_second = amount_results as f64 / duration.as_secs_f64();

    // append it to the Value list
    let search_results: Vec<Value> = matching_files.iter().map(|x| x.clone()).collect();
    
    // make a new obj value and return that including the amount of results
    let search_results: Value = json!({
        "results": search_results,
        "amount_results": amount_results,
        "duration": duration.as_millis(),
        "items_per_second": items_per_second,
    });


    Ok(search_results)
}
