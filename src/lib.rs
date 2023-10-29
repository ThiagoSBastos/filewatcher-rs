use std::fmt;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FileWatcher {
    dirname: String,
    filemap: HashMap<String, FileEvent>,
}

#[derive(Debug)]
pub enum FileEvent {
    Created,
    Updated,
    Deleted,
}

impl fmt::Display for FileEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileEvent::Created => write!(f, "FileEvent::Created"),
            FileEvent::Updated => write!(f, "FileEvent::Updated"),
            FileEvent::Deleted => write!(f, "FileEvent::Deleted"),
        }
    }
}

impl FileWatcher {
    pub fn new (pathname: String) -> std::io::Result<Self> {  

        let _ = fs::metadata(&pathname)?;

        let mut fmap = HashMap::new();
        let filenames: Vec<PathBuf>= fs::read_dir(&pathname)?
            .into_iter()
            .map(|r| r.unwrap().path())
            .filter(|r| r.is_file())
            .collect();

        for el in filenames {
            fmap.insert(el.into_os_string().into_string().unwrap(), FileEvent::Created);       
        }

        Ok(FileWatcher{dirname: pathname, filemap: fmap})
    }

    pub fn watch(&self, _action: impl FnOnce(FileEvent)) -> std::io::Result<()>{

        println!("Action performed on {}", self.dirname);
        for (key,value) in &self.filemap { println!("key: {}, value: {}", key, value); }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    //use std::{assert_eq, io::stdout};

    use super::*;

    #[test]
    fn it_works() -> std::io::Result<()> {
        let pathname = String::from("/home/thiago/Documents/Code/file-watcher-rs/file-watcher/test_data");
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
