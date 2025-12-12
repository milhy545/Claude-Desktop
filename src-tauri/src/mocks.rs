#[cfg(test)]
use crate::system::SystemOps;
#[cfg(test)]
use crate::error::AppError;
#[cfg(test)]
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::process::{Output, ExitStatus};
#[cfg(test)]
use std::sync::Mutex;
#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::os::unix::process::ExitStatusExt;

#[cfg(test)]
pub struct MockSystemOps {
    pub files: Mutex<HashMap<PathBuf, String>>,
    pub commands: Mutex<Vec<(String, Vec<String>)>>, // Zaznamenané příkazy
    pub command_outputs: Mutex<HashMap<String, (bool, String, String)>>, // (success, stdout, stderr)
}

#[cfg(test)]
impl MockSystemOps {
    pub fn new() -> Self {
        Self {
            files: Mutex::new(HashMap::new()),
            commands: Mutex::new(Vec::new()),
            command_outputs: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_file(self, path: &str, content: &str) -> Self {
        self.files.lock().unwrap().insert(PathBuf::from(path), content.to_string());
        self
    }

    pub fn with_command_output(self, command: &str, success: bool, stdout: &str, stderr: &str) -> Self {
        self.command_outputs.lock().unwrap().insert(command.to_string(), (success, stdout.to_string(), stderr.to_string()));
        self
    }
}

#[cfg(test)]
#[async_trait::async_trait]
impl SystemOps for MockSystemOps {
    async fn read_to_string(&self, path: &Path) -> Result<String, AppError> {
        let files = self.files.lock().unwrap();
        files.get(path)
            .cloned()
            .ok_or_else(|| AppError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")))
    }

    async fn write(&self, path: &Path, content: &str) -> Result<(), AppError> {
        let mut files = self.files.lock().unwrap();
        files.insert(path.to_path_buf(), content.to_string());
        Ok(())
    }

    async fn create_dir_all(&self, _path: &Path) -> Result<(), AppError> {
        Ok(())
    }

    async fn exists(&self, path: &Path) -> bool {
        let files = self.files.lock().unwrap();
        files.contains_key(path)
    }

    async fn remove_file(&self, path: &Path) -> Result<(), AppError> {
        let mut files = self.files.lock().unwrap();
        files.remove(path);
        Ok(())
    }

    async fn remove_dir_all(&self, path: &Path) -> Result<(), AppError> {
        // Zjednodušená implementace: smaže všechny soubory začínající touto cestou
        let mut files = self.files.lock().unwrap();
        files.retain(|k, _| !k.starts_with(path));
        Ok(())
    }

    fn home_dir(&self) -> Option<PathBuf> {
        Some(PathBuf::from("/home/mockuser"))
    }

    fn config_dir(&self) -> Option<PathBuf> {
        Some(PathBuf::from("/home/mockuser/.config"))
    }

    async fn run_command(&self, command: &str, args: &[&str]) -> Result<Output, AppError> {
        // Zaznamenat volání
        self.commands.lock().unwrap().push((command.to_string(), args.iter().map(|s| s.to_string()).collect()));

        let outputs = self.command_outputs.lock().unwrap();
        if let Some((success, stdout, stderr)) = outputs.get(command) {
            // Vytvořit ExitStatus (hacky pro Unix)
            let status = ExitStatus::from_raw(if *success { 0 } else { 1 } << 8);
            Ok(Output {
                status,
                stdout: stdout.as_bytes().to_vec(),
                stderr: stderr.as_bytes().to_vec(),
            })
        } else {
            // Default mock response
             let status = ExitStatus::from_raw(0);
             Ok(Output {
                status,
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        }
    }
}
