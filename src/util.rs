#[derive(Clone, Debug)]
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

pub struct Input<R: std::io::Read> {
    pub type_: Type,
    pub reader: R,
}

impl<R: std::io::Read> Input<R> {
    pub fn read(&mut self) -> Result<String, std::io::Error> {
        let mut contents = String::new();
        self.reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

pub struct Output<W: std::io::Write> {
    pub type_: Type,
    pub writer: W,
}

impl<W: std::io::Write> Output<W> {
    pub fn write(&mut self, contents: &str) -> Result<(), std::io::Error> {
        self.writer.write_all(contents.as_bytes())?;
        Ok(())
    }
}
