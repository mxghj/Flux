mod core;
use crate::core::apps::{indexer::indexing};

// Main function that run parser
fn main()
{
    println!("This is example of parser it will be print all .desktop file and their icons!");
    let apps = indexing().unwrap_or_default();
    for entry in apps {
        println!("App {}, icon {:?}, comment {}, exec {}, type {}", entry.name, entry.icon_path, entry.description, entry.exec, entry.type_file);
    }
}
