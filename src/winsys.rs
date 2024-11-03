use std::env;
use std::fs;
use tracing::level_filters::LevelFilter;

/// Cleans out the temporary directory and its contents.
pub fn tempclean() {
    let dir = env::temp_dir();
    info!("Temporary directory: {}", dir.display());


    // Reads items in the directory with a for loop
    match fs::read_dir(dir) {

        Ok(entries) => {


            for entry in entries {
                //gets the whole file path
                match entry {
                    Ok(file) => {
                        let file_path = file.path();

                        //gets the file type and checks whether is a file or a directory
                        match file.file_type() {
                            Ok(file_type) => if file_type.is_file(){

                                //File logic
                                match fs::remove_file(&file_path) {
                                    Ok(_) => info!("Deleted file: {}", file_path.display()),
                                    Err(e) => {
                                        error!(
                                            "Failed to delete file: {} Error: {}",
                                            file_path.display(),
                                            e
                                        )
                                    }
                                }                                
                            } 

                            //directory logic
                            else if file_path.is_dir(){
                                match fs::remove_dir_all(&file_path) {
                                    Ok(_) => info!("Deleted directory: {}", file_path.display()),
                                    Err(e) => {
                                        error!(
                                            "Failed to delete directory: {} Error: {}",
                                            file_path.display(),
                                            e
                                        )
                                    }
                                }
                            } 
                            //other files handle
                            else{
                                info!("Other type of file found: {}", file_path.display())
                            }


                            Err(e) => error!("Error reading file type: {}", e),
                        }
                      
                    }
                    Err(e) => error!("Error reading entry: {}", e),

                    
                }
            }
        }
        Err(e) => error!("Error reading temporary directory: {}", e),
    }


}


//TODO() : Ask for user validation to delete files after ennumerating all files
//TODO() : Display total amount of files and how many of them were able to be deleted