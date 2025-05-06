#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::collections::HashMap;
    use crate::{try_run, IncludeDetails};

    fn get_project_root() -> PathBuf {
        let current_dir = std::env::current_dir().unwrap();
        
        // Navigate up from transpiler directory if needed
        if current_dir.ends_with("transpiler") {
            return current_dir.parent().unwrap().to_path_buf();
        }
        
        current_dir
    }

    fn resolve_lib_modules() -> HashMap<String, String> {
        let project_root = get_project_root();
        let lib_dir = project_root.join("plonk/lib");

        if !lib_dir.exists() {
            panic!("The 'plonk/lib' directory does not exist at path: {}", lib_dir.display());
        }
        
        let mut modules = HashMap::new();
        for entry in fs::read_dir(lib_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("plonk") {
                let code = fs::read_to_string(path.clone()).expect("cannot read library");
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                modules.insert(name, code);
            }
        }
        modules
    }

    fn get_plonk_files() -> Vec<PathBuf> {
        let project_root = get_project_root();
        let src_dir = project_root.join("plonk/src");
        
        if !src_dir.exists() {
            panic!("The 'plonk/src' directory does not exist at path: {}", src_dir.display());
        }
        
        let mut files = Vec::new();
        for entry in fs::read_dir(src_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("plonk") {
                files.push(path);
            }
        }
        
        files
    }

    fn create_snapshot_dir() -> PathBuf {
        let project_root = get_project_root();
        let snapshot_dir = project_root.join("transpiler/snapshots");
        
        if !snapshot_dir.exists() {
            fs::create_dir_all(&snapshot_dir).expect("Failed to create snapshots directory");
        }
        
        snapshot_dir
    }

    fn save_snapshot(file_path: &Path, content: &str) {
        let snapshot_dir = create_snapshot_dir();
        
        let base_name = file_path
            .file_stem()
            .unwrap()
            .to_string_lossy();
            
        let snapshot_path = snapshot_dir.join(format!("{}.snapshot", base_name));
        fs::write(snapshot_path, content).expect("Failed to write snapshot");
    }

    fn compare_with_snapshot(file_path: &Path, content: &str) -> bool {
        let snapshot_dir = create_snapshot_dir();
        
        let base_name = file_path
            .file_stem()
            .unwrap()
            .to_string_lossy();
            
        let snapshot_path = snapshot_dir.join(format!("{}.snapshot", base_name));
        
        if !snapshot_path.exists() {
            return false;
        }
        
        let snapshot_content = fs::read_to_string(snapshot_path)
            .expect("Failed to read snapshot");
            
        snapshot_content == content
    }

    #[test]
    fn test_all_plonk_files() {
        // Create snapshots directory if it doesn't exist
        create_snapshot_dir();
        
        // Get all plonk files
        let files = get_plonk_files();
        let modules = resolve_lib_modules();
        
        // Test each file
        for file_path in files {
            println!("Testing file: {}", file_path.display());
            
            // Read the file
            let code = fs::read_to_string(&file_path).expect("Failed to read plonk file");
            
            // Run the transpiler
            let result = try_run(
                code, 
                modules.clone(), 
                Some(IncludeDetails::TranspiledScript)
            );
            
            // Check if the transpiler succeeded
            match result {
                Ok(output) => {
                    let file_name = file_path.file_name().unwrap().to_string_lossy();
                    println!("  - Transpilation successful for {}", file_name);
                    
                    // Create a summary of the output for snapshot testing
                    let summary = format!(
                        "File: {}\nTranspiled Script Length: {} characters\nFirst 100 chars: {}\n",
                        file_name,
                        output.transpiled_script.len(),
                        &output.transpiled_script.chars().take(100).collect::<String>()
                    );
                    
                    // Check if we need to update the snapshot
                    let update_snapshots = std::env::var("UPDATE_SNAPSHOTS").unwrap_or_default() == "1";
                    
                    if update_snapshots {
                        save_snapshot(&file_path, &summary);
                        println!("  - Updated snapshot for {}", file_name);
                    } else {
                        // Compare with existing snapshot
                        let matches = compare_with_snapshot(&file_path, &summary);
                        assert!(
                            matches, 
                            "Output for {} doesn't match snapshot. Run with UPDATE_SNAPSHOTS=1 to update.", 
                            file_name
                        );
                        println!("  - Output matches snapshot for {}", file_name);
                    }
                },
                Err(e) => {
                    panic!("Transpilation failed for {}: {:?}", file_path.display(), e);
                }
            }
        }
    }
    
    // Individual tests for each plonk file
    macro_rules! generate_test {
        ($test_name:ident, $file_name:expr) => {
            #[test]
            fn $test_name() {
                let project_root = get_project_root();
                let file_path = project_root.join(format!("plonk/src/{}", $file_name));
                
                if !file_path.exists() {
                    println!("Skipping test for {} as file doesn't exist", $file_name);
                    return;
                }
                
                let code = fs::read_to_string(&file_path).expect("Failed to read plonk file");
                let modules = resolve_lib_modules();
                
                let result = try_run(
                    code, 
                    modules, 
                    Some(IncludeDetails::TranspiledScript)
                );
                
                assert!(result.is_ok(), "Transpilation failed for {}: {:?}", $file_name, result.err());
                println!("Transpilation successful for {}", $file_name);
            }
        };
    }

    // Generate individual tests for each known file
    generate_test!(test_fibonacci, "fibonacci.plonk");
    generate_test!(test_mimc5_feistel, "mimc5-feistel.plonk");
    generate_test!(test_mimc5, "mimc5.plonk");
    generate_test!(test_poseidon, "poseidon.plonk");
    generate_test!(test_simple_arith, "simple_arith.plonk");
    generate_test!(test_simple_demo, "simple_demo.plonk");
    generate_test!(test_table_simple, "table_simple.plonk");
}
