mod file;
mod folder;

pub use file::File;
pub use folder::Folder;

pub trait Component {
    fn search(&self, keyword: &str);
}

#[cfg(test)]
mod test {
    use super::file::File;

    fn test_composite() {
        let file1 = File::new("File 1");
        let file2 = File::new("File 2");
    }
}
