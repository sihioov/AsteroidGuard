pub(crate) trait Env {
    fn file_exists(&self, path: &str) -> bool;
    fn getprop(&self, key: &str) -> Option<String>;
    fn which(&self, bin: &str) -> bool;
}

pub(crate) struct DefaultEnv {}

impl Env for DefaultEnv {
    fn file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn getprop(&self, key: &str) -> Option<String> {
        let output = std::process::Command::new("getprop").arg(key).output();
        match output {
            Ok(out) => {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    let trimmed = s.trim().to_string();
                    if trimmed.is_empty() { None } else { Some(trimmed) }
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    fn which(&self, bin: &str) -> bool {
        if let Ok(path_var) = std::env::var("PATH") {
            for dir in path_var.split(if cfg!(windows) { ';' } else { ':' }) {
                let candidate = std::path::Path::new(dir).join(bin);
                if candidate.exists() {
                    return true;
                }
                if cfg!(windows) {
                    let candidate_exe = std::path::Path::new(dir).join(format!("{bin}.exe"));
                    if candidate_exe.exists() {
                        return true;
                    }
                }
            }
        }
        false
    }
}


