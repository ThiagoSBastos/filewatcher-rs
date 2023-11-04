use std::fs;
use std::collections::{HashMap, HashSet};
use std::io;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
pub struct FileWatcher {
    dirname: PathBuf,
    filemap: HashMap<String, (FileEvent, SystemTime)>,
}

#[derive(Debug)]
pub enum FileEvent {
    Created,
    Updated,
    Deleted,
}

impl FileWatcher {
    fn get_files (pathname: &PathBuf) -> io::Result<Vec<(String, SystemTime)>>{
        let mut filenames = vec![];
        let entries = fs::read_dir(&pathname)?;
        for entry in entries {
            let entry = entry?;
            let meta = entry.metadata()?;
            if meta.is_dir() {
                let mut subdir = Self::get_files(&entry.path())?;
                filenames.append(&mut subdir);
            }

            if meta.is_file() {
                filenames.push((entry.path().display().to_string(), meta.modified()?));
            }
        }


        return Ok(filenames);
    }

    // Registers all files of a directory as Created for the FileWatcher
    pub fn new (pathname: PathBuf) -> io::Result<Self> {
        let filenames = Self::get_files(&pathname)?;

        let mut fmap = HashMap::new();
        for (el, time) in filenames {
            fmap.insert(el, (FileEvent::Created, time));
        }

        Ok(FileWatcher{dirname: pathname, filemap: fmap})
    }

    pub fn watch(&mut self, action: impl Fn(FileEvent)) -> io::Result<()>{

        println!("Action performed on {}", self.dirname.as_path().display().to_string());
        for (key,value) in &self.filemap { println!("key: {}, value: {:?}", key, value); }

        loop {
            let curr_list = Self::get_files(&self.dirname)?;
            let (curr_file_list, _): (Vec<String>, Vec<SystemTime>) = curr_list.clone().into_iter().unzip();

            // Case 0: The file no longer exists and should be DELETED
            let old_set: HashSet<String> = self.filemap.keys().cloned().collect();
            let curr_set: HashSet<String> = curr_file_list.iter().cloned().collect();
            let sym_difference: Vec<String> = (&old_set - &curr_set).iter().cloned().collect();
            for removed_file in sym_difference {
                action(FileEvent::Deleted);
                self.filemap.remove(&removed_file);
            }

            for (file, curr_time) in &curr_list {
                // Case 1: The file was already mapped and may have been UPDATED
                if self.filemap.contains_key(file) {
                    // Check if file has been modified, if it is, mark as UPDATED
                    let metadata = fs::metadata(&file)?;
                    match metadata.modified() { // FIXME: Figure out why this doesn't work
                        Ok(curr_time) => {
                            let sys_time = SystemTime::now();
                            let curr_duration = curr_time.duration_since(sys_time);
                            let old_duration = self.filemap.get(file).unwrap().1.duration_since(sys_time);

                            match (curr_duration, old_duration) {
                                (Ok(curr_duration), Ok(old_duration)) => {
                                    if (curr_duration.as_secs() - old_duration.as_secs()) > 0 {
                                        println!("Modified")
                                    }
                                },
                                _ => (),
                            }
                        },
                        Err(e) => return Err(e),
                    }
                }
                // Case 2: The file was CREATED
                else {
                    action(FileEvent::Created);
                    self.filemap.insert(file.clone(), (FileEvent::Created, curr_time.clone()));
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    //use std::{assert_eq, io::stdout};

    use super::*;

    #[test]
    fn it_works() -> io::Result<()> {
        let pathname: PathBuf = PathBuf::from("test_data");
        FileWatcher::new(pathname)?.watch(|file_event| {
            match file_event {
                FileEvent::Created => println!("Action for created"),
                FileEvent::Updated => println!("Action for updated"),
                FileEvent::Deleted => println!("Action for deleted")
            }
        })

        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
