// A pair of a directory path and a vector of directory or file paths within the first directory
// path.
#[derive(Clone, Debug)]
pub struct SubPaths(String, Vec<String>);

#[derive(Clone, Debug)]
pub struct GitIgnore {
    pub exclude_all: Vec<String>,
    pub exclude_paths: Vec<SubPaths>,
    pub include_paths: Vec<SubPaths>,
}
