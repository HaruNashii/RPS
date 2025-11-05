use crate::sdl::sdl3_generators::ASSETS;

pub fn list_embedded()
{
    println!("📦 Embedded entries:");
    for entry in ASSETS.entries()
    {
        match entry
        {
            include_dir::DirEntry::File(file) =>
            {
                println!("📄 {}", file.path().display());
            }
            include_dir::DirEntry::Dir(dir) =>
            {
                println!("📁 {}", dir.path().display());
                for sub_entry in dir.entries()
                {
                    match sub_entry
                    {
                        include_dir::DirEntry::File(sub_file) =>
                        {
                            println!("  📄 {}", sub_file.path().display());
                        }
                        include_dir::DirEntry::Dir(sub_dir) =>
                        {
                            println!("  📁 {}", sub_dir.path().display());
                            for sub_sub in sub_dir.entries()
                            {
                                if let include_dir::DirEntry::File(f) = sub_sub
                                {
                                    println!("    📄 {}", f.path().display());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
