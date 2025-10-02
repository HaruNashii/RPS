use std::env;
use std::fs;

fn main() 
{

    //Move The Image To An Tmp Folder
    let home_path = env::home_dir().unwrap().display().to_string();
    let target_image_example_dir = format!("{}/.cache/page_system", home_path).replace(" ", "");

    let image_example_path = ["assets/image_example/example_1.jpg", "assets/image_example/example_2.jpg"];
    let image_example_dir = [format!("{}/example_1.jpg", target_image_example_dir).replace(" ", ""), format!("{}/example_2.jpg", target_image_example_dir).replace(" ", "")];



    if !fs::exists(&target_image_example_dir).unwrap() 
    {
        fs::create_dir_all(&target_image_example_dir).expect("Failed to create image directory");
    };

    for (index, currently_image_example_path) in image_example_path.iter().enumerate()
    {
        if fs::exists(currently_image_example_path).unwrap() && !fs::exists(image_example_dir[index].clone()).unwrap() 
        {
            fs::copy(currently_image_example_path, image_example_dir[index].clone()).unwrap();
        };
    }
}
