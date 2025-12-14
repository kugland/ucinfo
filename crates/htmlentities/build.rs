use anyhow::anyhow;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::path::Path;
use std::{env, fs, path};

fn output_file_path<P: AsRef<Path>>(filename: P) -> anyhow::Result<path::PathBuf> {
    let binding = env::var_os("OUT_DIR").ok_or_else(|| anyhow!("OUT_DIR not set"))?;
    Ok(path::Path::new(&binding).join(filename))
}

fn load_data(data_path: &Path) -> anyhow::Result<Vec<(u32, Vec<String>)>> {
    let mut map = HashMap::<u32, HashSet<String>>::new();
    let contents = fs::read_to_string(data_path)?;
    let metadata: serde_json::Value = serde_json::from_str(&contents)?;
    let metadata = metadata
        .as_object()
        .ok_or_else(|| anyhow!("Invalid JSON format"))?;

    for (entity, value) in metadata {
        let Some(codepoints) = value.get("codepoints").and_then(|cp| cp.as_array()) else {
            anyhow::bail!("Missing codepoints for entity {}", entity);
        };
        if codepoints.len() != 1 {
            continue; // Skip entities with multiple codepoints
        }
        let Some(codepoint) = codepoints[0].as_u64() else {
            anyhow::bail!("Invalid codepoint for entity {}", entity);
        };
        let codepoint = codepoint as u32;

        let entity = entity.trim_start_matches('&').trim_end_matches(';');

        map.entry(codepoint).or_insert_with(|| HashSet::new());
        map.entry(codepoint).and_modify(|m| {
            m.insert(entity.to_string());
        });
    }

    let mut result: Vec<(u32, Vec<String>)> = map
        .into_iter()
        .map(|(cp, set)| {
            let mut set: HashSet<String> = set;
            for entity in set.clone() {
                // Exclude all-caps variants to avoid duplication
                let uppercase = entity.to_uppercase();
                if uppercase != entity && set.contains(&uppercase) {
                    set.remove(&uppercase);
                }
            }
            for entity in set.clone() {
                // Exclude all-lowercase variants to avoid duplication
                let lowercase = entity.to_lowercase();
                if lowercase != entity && set.contains(&lowercase) {
                    set.remove(&lowercase);
                }
            }
            let mut entities: Vec<String> = set.into_iter().collect();
            entities.sort_by_key(|s| s.to_lowercase());
            (cp, entities)
        })
        .collect();
    result.sort_by_key(|&(cp, _)| cp);

    Ok(result)
}

/// Save all glyphs with a given width to a binary file.
fn save_entities_data<P: AsRef<Path>>(
    entities: &[(u32, Vec<String>)],
    filename: P,
) -> anyhow::Result<String> {
    let output_file = output_file_path(filename)?;

    let mut output = fs::File::create(&output_file)?;
    let encoded: Vec<u8> = bincode::encode_to_vec(entities, bincode::config::standard()).unwrap();

    output.write_all(&encoded)?;

    Ok(output_file.to_string_lossy().to_string())
}

fn main() -> anyhow::Result<()> {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("htmlentities");

    if !data_dir.exists() {
        return Err(anyhow::anyhow!(
            "Data directory does not exist: {}",
            data_dir.display()
        ));
    }

    let json = data_dir.join("entities.json");
    let entities = load_data(&json)?;
    let bin = save_entities_data(&entities, "htmlentities.bin")?;

    println!("cargo:rerun-if-changed={}", json.display());
    println!("cargo:rustc-env=HTMLENTITIES_BIN_FILE={}", bin);

    Ok(())
}
