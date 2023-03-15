use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Result, Write},
    path::Path,
};

/// Creates a `json` file from the `yaml` frontmatter
/// of a given `markdown` document.
fn jsonify_frontmatter<P: AsRef<Path>>(path: P) -> Result<()> {
    // Read given `.md` file
    let file = File::open(path)?;

    // Create mutable `BufReader` to read file contents
    let mut bufreader = BufReader::new(file);

    // Read file contents as string to `contents`
    let mut contents = String::new();
    bufreader.read_to_string(&mut contents)?;

    // Collect content string by line
    let lines = contents.lines().collect::<Vec<_>>();

    // Ignore first `---`
    let fm_slice = &lines[1..];

    // Get idx (index) of `---` for string splicing/slicing
    let last_idx = fm_slice
        .into_iter()
        .position(|line| *line == "---")
        .unwrap();

    // Required frontmatter &str slice
    let frontmatter = &lines[1..last_idx + 1];

    // Create HashMap<&str, &str> for storing yaml key value pairs
    let mut md_map: HashMap<&str, &str> = HashMap::new();

    for item in frontmatter.iter() {
        let split = item.split(": ").into_iter().collect::<Vec<_>>();
        md_map.insert(split[0], split[1]);
    }

    // Convert HashMap to pretty printed json string
    let json_fm = serde_json::to_string_pretty(&md_map)?;

    // Create `output.json` file
    let mut file = File::create("output.json")?;

    // Write json_fm as bytest to file buffer
    file.write_all(json_fm.as_bytes())?;

    Ok(())
}

fn main() -> Result<()> {
    jsonify_frontmatter("hello.md")
}
