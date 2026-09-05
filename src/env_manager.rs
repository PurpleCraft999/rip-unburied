use std::{
    collections::HashMap,
    env::{self, VarError},
    path::{Path, PathBuf},
};
#[derive(Debug, Clone)]
pub struct EnvManager {
    current_dir: PathBuf,
    var_map: HashMap<String, String>,
}
impl EnvManager {
    pub fn current_dir(&self) -> PathBuf {
        self.current_dir.clone()
    }
    pub fn var(&self, key: &str) -> Result<&String, VarError> {
        self.var_map.get(key).ok_or(VarError::NotPresent)
    }
    pub fn set_var(&mut self, key: &str, value: &str) {
        self.var_map.insert(key.to_owned(), value.to_owned());
    }
    pub fn envs(&self) -> HashMap<String, String> {
        self.var_map.clone()
    }

    pub fn set_current_dir(&mut self, path: &Path) {
        self.current_dir = path.to_path_buf()
    }
    pub fn remove_var(&mut self, key: &str) {
        self.var_map.remove(key);
    }
}
impl Default for EnvManager {
    fn default() -> Self {
        let mut var_map = HashMap::new();
        if let Ok(var) = env::var("RIP_GRAVEYARD") {
            var_map.insert("RIP_GRAVEYARD".to_owned(), var);
        }
        if let Ok(var) = env::var("XDG_DATA_HOME") {
            var_map.insert("XDG_DATA_HOME".to_owned(), var);
        }
        if let Ok(var) = env::var("__RIP_ALLOW_RENAME") {
            var_map.insert("__RIP_ALLOW_RENAME".to_owned(), var);
        }

        Self {
            current_dir: env::current_dir().expect("Failed to get current directory"),
            var_map, // var_map: HashMap::from([
                     //     ("XDG_DATA_HOME".to_owned(), env::var("XDG_DATA_HOME")),
                     //     (
                     //         "__RIP_ALLOW_RENAME".to_owned(),
                     //         env::var("__RIP_ALLOW_RENAME"),
                     //     ),
                     // ]),
        }
    }
}
