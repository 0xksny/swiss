pub enum Type {
    JSON,
    YAML,
}

pub fn get_type_from_path(path: &str) -> Option<Type> {
    println!("{}", path);

    let path = std::path::Path::new(path);

    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("json") => Some(Type::JSON),
            Some("yaml") | Some("yml") => Some(Type::YAML),
            _ => None,
        },
        None => None,
    }
}
