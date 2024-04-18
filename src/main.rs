use anyhow::Result;
use std::fs;
use walkdir::WalkDir;

mod commons;

fn main() {
    let subfix = ".txt";
    // add_file_subfix("/tmp/a", subfix);
    add_folder_file_subfix("/Users/jiashiwen/Downloads/jdcloud-docs", subfix).unwrap();
}

fn add_folder_file_subfix(folder: &str, subfix: &str) -> Result<()> {
    for entry in WalkDir::new(folder)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
    {
        if let Some(p) = entry.path().to_str() {
            if p.eq(folder) {
                continue;
            }

            if p.ends_with(subfix) {
                continue;
            }
            add_file_subfix(p, subfix)?;
            // if let Some(f) = &regex_filter {
            //     if !f.filter(p) {
            //         continue;
            //     }
            // }

            // if let Some(f) = &last_modify_filter {
            //     let modified_time = entry
            //         .metadata()?
            //         .modified()?
            //         .duration_since(UNIX_EPOCH)?
            //         .as_secs();
            //     if !f.filter(i128::from(modified_time)) {
            //         continue;
            //     }
            // }

            // let obj_size = i128::from(entry.metadata()?.len());
            // let key = size_distributed(obj_size);
            // let mut size = match size_map.get(&key) {
            //     Some(m) => *m.value(),
            //     None => 0,
            // };
            // size += 1;
            // size_map.insert(key, size);
        };
    }
    Ok(())
}

//Todo 掉过后缀已为subfix的文件
fn add_file_subfix(file_path: &str, subfix: &str) -> Result<()> {
    let mut new_file_path = file_path.to_string();
    new_file_path.push_str(subfix);
    fs::rename(file_path, new_file_path)?;
    Ok(())
}

fn copy_file_and_add_subfix(file_path: &str, target_file_path: &str) {}
