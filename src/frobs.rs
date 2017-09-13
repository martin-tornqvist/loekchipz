// *** THIS MODULE IS ONLY FOR DEMO PUROSES - TO BE REMOVED ***

use io;
use json;
use std::path::PathBuf;

pub fn test_load_frobs()
{
    let path: PathBuf = ["data", "frobs.json"].iter().collect();

    let content = io::file_to_str(&path);

    log!("Content:\n{}", content);

    let data = json::parse(&content).unwrap();

    // Accessing data is nice and easy!
    for frob in data["frobs"].members()
    {
        log!(
            "Hi my name is {}, I have {} hit points and do {} damage.",
            frob["name"],
            frob["hp"],
            frob["damage"]
        );
    }
}
