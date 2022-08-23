use std::{path::PathBuf, time::Duration};

// note that we can simply auto-derive Default here.
#[derive(Default, Debug, PartialEq)]
pub struct MyConfiguration {
    // Option defaults to None
    pub output: Option<PathBuf>,
    // Vecs defaults to empty vector
    pub search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    pub timeout: Duration,
    // bool defaults to false
    pub check: bool,
}

impl MyConfiguration {
    // add setters here
}
