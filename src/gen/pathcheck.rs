
pub fn generate_pathcheckp(urls: &[String], paths: &[String]) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for i in urls {
        let mut t: Vec<String> = paths.iter().map(
            |path| format!("{}/{}", i, path)
        ).collect();
        vec.append(&mut t);
    }
    vec
}

