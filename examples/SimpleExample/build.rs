use std::{env, fs, path::PathBuf};

pub fn setup_build()
{
    // ==== Source & destination paths ====
    let source_dir = PathBuf::from("assets/image_example");
    let home_dir = env::home_dir().unwrap().display().to_string();
    let target_dir = PathBuf::from(format!("{}/.cache/page_system/assets", home_dir));

    // ==== Create the destination directory ====
    fs::create_dir_all(&target_dir).unwrap();

    // ==== Copy all image files recursively ====
    if source_dir.exists()
    {
        for entry in fs::read_dir(&source_dir).unwrap()
        {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file()
            {
                if let Some(ext) = path.extension()
                {
                    if matches!(ext.to_str(), Some("bmp") | Some("png") | Some("svg") | Some("ico") | Some("jpg") | Some("jpeg")) && !fs::exists(target_dir.join(entry.file_name())).unwrap()
                    {
                        let file_name = entry.file_name();
                        let dest_path = target_dir.join(file_name);
                        fs::copy(&path, &dest_path).unwrap();
                        println!("cargo:warning=📦 Copied {:?} → {:?}", path, dest_path);
                    }
                }
            }
        }
    }
    else
    {
        println!("cargo:warning=⚠️ No icons folder found at {:?}", source_dir);
    }
}
