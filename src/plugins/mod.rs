#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}

pub struct PluginEntry {
    pub name: String,
    pub author: String,
    pub version: String,
    pub alerts: Vec<String>,
    pub remediations: Vec<String>,
}