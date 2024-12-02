use std::fs;
use std::path::PathBuf;


pub struct BrowserCache {
    pub browser_name: String,
    pub cache_path: PathBuf,
}

impl BrowserCache {
    pub fn new(browser_name: &str) -> Self {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let appdata_local = home_dir.join("AppData").join("Local");

        let cache_paths = match browser_name.to_lowercase().as_str() {
            "chrome" => vec![
                appdata_local.join("Google//Chrome//User Data//Default//Cache"),
                appdata_local.join("Google//Chrome SxS//User Data//Default//Code Cache"),
            ],

            "firefox" => vec![
                appdata_local.join("Mozilla//Firefox//Profiles"),
            ],
            "edge" => vec![
                appdata_local.join("Microsoft//Edge//User Data//Default//Cache"),
                appdata_local.join("Microsoft//Edge//User Data//Default//Code Cache"),
            ],
            _ => return Err(format!("Unsupported browser: {}", browser_name)),
        };

        BrowserCache {
            browser_name: browser_name.to_string(),
            cache_paths,
        }
    }

    pub fn clear(&self) -> Result<(), std::io::Error> {

        for path in &self.cache_path {
            if path.exists() {
                //remove directory contents
                for entry in fs::read_dir(path)? {
                    let entry = entry?;
                    let path = entry.path();

                    //checks to remove directory or file
                    if path.is_dir() {
                        fs::remove_dir_all(path)?;
                    } else if path.is_file() {
                        fs::remove_file(path)?;
                    }
                }
            }
        }
        Ok(())


    }
}
