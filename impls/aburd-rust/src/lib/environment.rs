use std::collections::HashMap;

#[derive(Debug)]
pub struct MalEnvironment {
    reader_macros: HashMap<String, String>,
}

impl MalEnvironment {
    pub fn new() -> Self {
        Self {
            reader_macros: get_reader_macros(),
        }
    }
}

fn get_reader_macros() -> HashMap<String, String> {
    let mut reader_macros = HashMap::new();

    reader_macros.insert("@".to_string(), "deref".to_string());
    reader_macros.insert("'".to_string(), "quote".to_string());

    reader_macros
}
