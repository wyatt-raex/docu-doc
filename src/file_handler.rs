use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::*;
use chrono::*;
use substring::*;

// Stands for Docu-Doc File. Holds info on full file path, parent folder, file name, & extension type
struct DdFile {
    full_path: PathBuf,
    parent_directory: String,
    filename: String,
    extension: String,
    full_filename: String,
    date_mod: String,
    year_mod: String,
    month_mod: String,
    ym_combo: String,
}

/*impl DdFile {
    fn print(&self) {
        println!("Path: {:?}", self.full_path);
        println!("Parent Directory: {:?}", self.parent_directory);
        println!("Filename: {:?}", self.filename);
        println!("Type: {:?}", self.extension);
        println!("Full Filename: {:?}", self.full_filename);
        println!("Year Modified: {:?}", self.year_mod);
        println!("Month Modified {:?}", self.month_mod);
        println!("Year/Month Modified {:?}", self.ym_combo);
    }
}*/

pub fn algorithm_select(original_dir: &str, dest_dir: String, algorithm_code: String){
    // TODO: If only original_dir is specified make dest_dir = original_dir

    if algorithm_code == "1"{
        // TODO: have front-end specify if keep_folder_hierarchy should be true or not
        extension_sort(original_dir, dest_dir).ok();
    }
    else if algorithm_code == "2"{
        date_sort(original_dir, dest_dir).ok();
    }
    else{
        println!("You fool! that's not valid.");
    }
}

fn get_string_after_last_backslash(input_string: String) -> String {
    match input_string.rfind('\\') {
        Some(index) => input_string[index+1..].to_string(),
        None => input_string,
    }
}

//TODO: Program crashes if directory has a "." in the name
fn get_entry_info(entry: &DirEntry) -> DdFile {
    let mut ret: DdFile = DdFile {
        full_path: entry.path(),
        parent_directory: String::from(""),
        filename: String::from(""),
        extension: String::from(""),
        full_filename: String::from(""), // Filename + extension (e.g. test.txt)
        date_mod: String::from(""), //Unformatted Date Info
        year_mod: String::from(""),
        month_mod: String::from(""),
        ym_combo: String::from(""),
    };

    let path = Path::new(ret.full_path.as_os_str());

    // Get parent_directory. Example: a file "./test/file.txt" parent directory is "./test"
    ret.parent_directory = get_string_after_last_backslash(path.parent().unwrap().to_str().unwrap_or_default().to_string());

    // Get the name of the file with no extensions. Is originally an &OsStr, we want a regular String instead.
    ret.filename = path.file_stem().map(|x| x.to_str().unwrap_or_default().to_string()).unwrap();

    // Get extension type
    if path.is_dir() { 
        ret.extension = String::from("directory"); 

        //println!("{:?}", ret.filename);
        //println!("{:?}", path.extension());
        if path.extension() != None {
            let test = path.extension().map(|x| x.to_str().unwrap_or_default().to_string()).unwrap();
            //println!("Found period at end of directory name: {:?}", test);
            ret.filename = ret.filename + "." + &test.clone();
        }
    }
    else if path.is_dir() == false{
        // Get the extension of the file. Originally an &OsStr we want a regular String instead.
        if path.extension() != None {
            ret.extension = path.extension().map(|x| x.to_str().unwrap_or_default().to_string()).unwrap();
        }
        else {
            ret.extension = String::from("");
        }
    }

    //NEW 4/5: Get date modified + format that information
    let file_metadata = fs::metadata(path).ok().unwrap();
    let date_mod = file_metadata.modified().ok();
    let date_mod: chrono::DateTime<Utc> = date_mod.unwrap().into();
    ret.date_mod = date_mod.to_rfc3339();

    ret.year_mod = ret.date_mod.clone().substring(0, 4).to_string();
    ret.month_mod = ret.date_mod.clone().substring(5, 7).to_string();
    let mut ym = ret.ym_combo.clone();
    ym.push_str(&ret.year_mod);
    ym.push_str("/");
    ym.push_str(&ret.month_mod);
    ret.ym_combo = ym;

    // Set full filename
    // TODO handle if Directory
    ret.full_filename = ret.filename.clone() + "." + ret.extension.as_str();

    ret
}

//Method to scan through specified directory
fn extension_sort(original_dir: &str, dest_dir: String) -> std::io::Result<()> {
    //Creation of a found extension list. <Extension Type, Where extension is found>
    //TODO: Switch key pair from <Extension, Location> to <Location, Extension>
    /*Issue Detail: When recursing into new directory, won't sort extensions/dates found in original directory ie. already in the hashmap.
    So if you have .doc files in root case folder, it won't sort doc files in root > subfolder */
    let mut ext_list: HashMap<String, Vec<String>> = HashMap::new();
    extension_sort_recursive(original_dir, dest_dir, &mut ext_list)
}

fn extension_sort_recursive (original_dir: &str, dest_dir: String, ext_list: &mut HashMap<String, Vec<String>>) -> std::io::Result<()> {
    /* Start by getting the information of the files */

    // Get the entries of the directory and extract it from the Result<ReadDir, Error>
    let entries = fs::read_dir(original_dir).unwrap_or_else(|error| {
        panic!("Problem reading directory path to sort: {:?}. {:?}", original_dir, error);    
    });    

    for entry in entries {
        // Get file path by extracting it from entry's Result<DirEntry, Error>
        let entry = entry.unwrap_or_else(|error| {
            panic!("Could not read specific file path: {:?}", error);
        });
        let entry_info: DdFile = get_entry_info(&entry);
        //entry_info.print();
        //println!();

        // Recurse into directory if one exists
        if entry_info.extension == "directory" {
            // creates new path: "original_directory/new_directory_to_recurse_into"
            let new_dir: String = original_dir.clone().to_string() + "\\" + entry_info.filename.as_str();

            let new_dest_dir: String = dest_dir.clone().to_string() + "\\" + entry_info.filename.as_str();

            // Check to make sure this isn't a directory we've created as a result of sorting. If so we don't want to go into it.
            let current_folder_extensions= ext_list.get(original_dir);
            match current_folder_extensions {
                Some(extension_vector) => {
                    let mut ext_exists: bool = false;

                    for ext in extension_vector {
                        //println!("Checking extensions: {:?} == {:?}", ext, entry_info.filename);
                        if *ext == entry_info.filename {
                            ext_exists = true;
                            println!("Folder {:?} already exists!", ext);
                            break;
                        }
                    }

                    if !ext_exists {
                        // Recurse into folder -> it's not a folder the sort algo created
                        match extension_sort_recursive(&new_dir, new_dest_dir, ext_list) { _ => {}, }
                    }
                }
                None => { 
                    // No vector of folders created was return for this path. We can just go ahead and recurse
                    match extension_sort_recursive(&new_dir, new_dest_dir, ext_list) { _ => {}, }
                }
            }

        }
        else {
            // If the file is of a new extension type add it to our unique extensions list
            match ext_list.get(&entry_info.extension) {
                Some(_ext) => {},
                None => {
                    // If the extension doesn't exist yet add it, and create the folder for it
                    let mut new_ext_dir: String = dest_dir.clone().to_string() + "\\";
                    new_ext_dir += entry_info.extension.clone().as_str();

                    //println!("Currently at {:?}", original_dir);
                    //println!("Destination directory {:?}", dest_dir); 

                    let created_extension_folders = ext_list.get_mut(&dest_dir);
                    match created_extension_folders {
                        // If there's already some extensions listed for this path just push it to the vector
                        Some(vector) => {
                            let mut ext_already_exists: bool = false;
                            for ext in vector.clone() {
                                if ext == entry_info.extension {
                                    ext_already_exists = true;
                                    break;
                                } 
                            }
                            if !ext_already_exists {
                                vector.push(entry_info.extension.clone());
                            }
                        },

                        // Else make a new Vector for this path and insert it into the hashmap
                        None => {
                            ext_list.insert(original_dir.to_string().clone(), vec![entry_info.extension.clone()]);
                        }
                    }
                    if get_string_after_last_backslash(entry_info.parent_directory) == entry_info.extension{
                        break;
                    }
                    match fs::create_dir_all(&new_ext_dir) {
                        Ok(_test) => { /*println!("New Directory Created: {:?}\nResult: {:?}", new_ext_dir, test);*/ },
                        Err(err) => { panic!("Could not create directory: {:?}", err); }
                    }
                }
            }

            //println!("Extension List: {:?}", ext_list);

            /* Now prepare to move the files to the destination directory */
            let mut destination = PathBuf::from(&dest_dir);
            destination.push(&entry_info.extension); // Sorting subfolder
            destination.push(&entry_info.full_filename); // filename

            match fs::rename(&entry.path(), destination.clone()) {
                Ok(_) => {},
                Err(err) => { panic!("Could not move file.\n\n Extension list: {:?},\n\n File: {:?},\n\n Destination: {:?},\n\n  Err: {:?}", ext_list, entry_info.full_filename, destination, err); }
            }
        }
    }
    Ok(())
}

//NEW 4/5: date_sort method
//Method to scan through specified directory
fn date_sort(original_dir: &str, dest_dir: String) -> std::io::Result<()> {
    //Creation of a found extension list. <Extension Type, Where extension is found>
    //TODO: Switch key pair from <Date, Location> to <Location, Date>
    let mut date_list: HashMap<String, Vec<String>> = HashMap::new();
    date_sort_recursive(original_dir, dest_dir, &mut date_list)
}

//NEW 4/5: date_sort_recursive method
fn date_sort_recursive(original_dir: &str, dest_dir: String, date_list: &mut HashMap<String, Vec<String>>) -> std::io::Result<()> {
    /* Start by getting the information of the files */

    // Get the entries of the directory and extract it from the Result<ReadDir, Error>
    let entries = fs::read_dir(original_dir).unwrap_or_else(|error| {
        panic!("Problem reading directory path to sort: {:?}. {:?}", original_dir, error);    
    });    

    for entry in entries {
        // Get file path by extracting it from entry's Result<DirEntry, Error>
        
        let entry = entry.unwrap_or_else(|error| {
            panic!("Could not read specific file path: {:?}", error);
        });
        let entry_info: DdFile = get_entry_info(&entry);
        //entry_info.print();
        //println!();

        // Recurse into directory if one exists
        if entry_info.extension == "directory" {
            // creates new path: "original_directory/new_directory_to_recurse_into"
            let new_dir: String = original_dir.clone().to_string() + "\\" + entry_info.filename.as_str();

            let new_dest_dir: String = dest_dir.clone().to_string() + "\\" + entry_info.filename.as_str();

            // Check to make sure this isn't a directory we've created as a result of sorting. If so we don't want to go into it.
            let current_folder_dates= date_list.get(original_dir);
            match current_folder_dates {
                Some(date_vector) => {
                    let mut date_exists: bool = false;

                    for date in date_vector {
                        //println!("Checking extensions: {:?} == {:?}", ext, entry_info.filename);
                        if *date == entry_info.filename {
                            date_exists = true;
                            println!("Folder {:?} already exists!", date);
                            break;
                        }
                    }

                    if !date_exists {
                        // Recurse into folder -> it's not a folder the sort algo created
                        match date_sort_recursive(&new_dir, new_dest_dir, date_list) { _ => {}, }
                    }
                }
                None => { 
                    // No vector of folders created was return for this path. We can just go ahead and recurse
                    match date_sort_recursive(&new_dir, new_dest_dir, date_list) { _ => {}, }
                }
            }
        }
        else {
            // If the file is of a new extension type add it to our unique extensions list
            match date_list.get(&entry_info.ym_combo) {
                Some(_date) => {},
                None => {
                    // If the extension doesn't exist yet add it, and create the folder for it
                    let mut new_date_dir: String = dest_dir.clone().to_string() + "\\";
                    new_date_dir += entry_info.ym_combo.clone().as_str();
                      
                    let created_date_folders = date_list.get_mut(&dest_dir);
                    match created_date_folders {
                        // If there's already some extensions listed for this path just push it to the vector
                        Some(vector) => {
                            let mut date_already_exists: bool = false;
                            for date in vector.clone() {
                                if date == entry_info.ym_combo {
                                    date_already_exists = true;
                                    break;
                                } 
                            }
                            if !date_already_exists {
                                vector.push(entry_info.ym_combo.clone());
                            }
                        },

                        // Else make a new Vector for this path and insert it into the hashmap
                        None => {
                            date_list.insert(original_dir.to_string().clone(), vec![entry_info.ym_combo.clone()]);
                        }
                    }
                    if get_string_after_last_backslash(entry_info.parent_directory) == entry_info.month_mod{
                        break;
                    }
                    match fs::create_dir_all(&new_date_dir) {
                        Ok(_test) => { /*println!("New Directory Created: {:?}\nResult: {:?}", new_ext_dir, test);*/ },
                        Err(err) => { panic!("Could not create directory: {:?}", err); }
                    }
                }
            }

            //println!("Date List: {:?}", date_list);

            /* Now prepare to move the files to the destination directory */
            let mut destination = PathBuf::from(&dest_dir);
            destination.push(&entry_info.ym_combo); // Sorting subfolder
            destination.push(&entry_info.full_filename); // filename
            //println!("Destination: {:?}\n", destination);

            match fs::rename(&entry.path(), destination.clone()) {
                Ok(_) => {},
                Err(err) => { panic!("Could not move file.\n\n Date list: {:?},\n\n File: {:?},\n\n Destination: {:?},\n\n  Err: {:?}", date_list, entry_info.full_filename, destination, err); }
            }
        }
    }
    Ok(())
}
