use std::fs;
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileWatcher {
    dirname: PathBuf,
    filemap: HashMap<String, FileEvent>,
}

#[derive(Debug)]
pub enum FileEvent {
    Created,
    Updated,
    Deleted,
}

impl FileWatcher {
    fn get_files (pathname: &PathBuf) -> io::Result<Vec<PathBuf>>{
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
                filenames.push(entry.path());
            }
        }


        return Ok(filenames);
    }

    // Registers all files of a directory as Created for the FileWatcher
    pub fn new (pathname: PathBuf) -> io::Result<Self> {
        let filenames = Self::get_files(&pathname)?;

        let mut fmap = HashMap::new();
        for el in filenames {
            fmap.insert(el.into_os_string().into_string().unwrap(), FileEvent::Created);
        }

        Ok(FileWatcher{dirname: pathname, filemap: fmap})
    }

    pub fn watch(&self, _action: impl FnOnce(FileEvent)) -> io::Result<()>{

        println!("Action performed on {}", self.dirname.as_path().display().to_string());
        for (key,value) in &self.filemap { println!("key: {}, value: {:?}", key, value); }

        // loop {
        // make a state machine before any action has been done to check the current state of the
        // file
        // action();
        // }

        Ok(())
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
