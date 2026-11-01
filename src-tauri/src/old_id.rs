//! Up to 0.2.3 the desk's identifier was `dev.softov.ahpdesk`, and the OS
//! folders the desk keeps its data in are named after the identifier. On its
//! first start as `com.ahpio.desk`, a desk that updated finds its old hosts
//! and its old ahpd through here.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The identifier up to 0.2.3.
const OLD_ID: &str = "dev.softov.ahpdesk";

/// Brings the 0.2.3 folders beside `config` and `local` over, before anything
/// reads them. A failure is logged and the desk starts without them: the host
/// list starts empty, and Add host offers to install ahpd again.
pub fn take_old_folders(config: &Path, local: &Path) {
    if let Err(err) = copy_config(config) {
        eprintln!("ahp-desk: could not copy the host list from {OLD_ID}: {err}");
    }
    if let Err(err) = move_local_data(local) {
        eprintln!("ahp-desk: could not move the local data from {OLD_ID}: {err}");
    }
}

/// Copies the old config folder (the host list, the settings) to `new`. The
/// old one stays, so going back to 0.2.3 still finds its hosts. The copy is
/// made beside `new` and renamed into place, so a stop halfway never leaves
/// half a host list.
fn copy_config(new: &Path) -> io::Result<bool> {
    let Some(old) = old_beside(new)? else {
        return Ok(false);
    };
    let part = new.with_file_name(format!("{OLD_ID}.copying"));
    remove_all(&part)?;
    copy_dir(&old, &part)?;
    remove_if_empty(new)?;
    fs::rename(&part, new)?;
    Ok(true)
}

/// Moves the old local data folder (the ahpd the desk installed, the app
/// windows' web storage) to `new`: one rename, nothing copied.
fn move_local_data(new: &Path) -> io::Result<bool> {
    let Some(old) = old_beside(new)? else {
        return Ok(false);
    };
    remove_if_empty(new)?;
    fs::rename(&old, new)?;
    Ok(true)
}

/// The old folder beside `new`, when there is one and `new` holds nothing yet.
fn old_beside(new: &Path) -> io::Result<Option<PathBuf>> {
    let old = new.with_file_name(OLD_ID);
    if !old.is_dir() || holds_anything(new)? {
        return Ok(None);
    }
    Ok(Some(old))
}

fn holds_anything(dir: &Path) -> io::Result<bool> {
    match fs::read_dir(dir) {
        Ok(mut entries) => Ok(entries.next().is_some()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err),
    }
}

fn remove_if_empty(dir: &Path) -> io::Result<()> {
    match fs::remove_dir(dir) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

fn remove_all(dir: &Path) -> io::Result<()> {
    match fs::remove_dir_all(dir) {
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

fn copy_dir(from: &Path, to: &Path) -> io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A folder that stands for `%APPDATA%` or `~/.config`, gone at the end of the test.
    struct Scratch(PathBuf);
    impl Scratch {
        fn new() -> Self {
            static NEXT: AtomicUsize = AtomicUsize::new(0);
            let dir = std::env::temp_dir().join(format!(
                "ahp-desk-old-id-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(&dir).unwrap();
            Scratch(dir)
        }
        fn old(&self) -> PathBuf {
            self.0.join(OLD_ID)
        }
        fn new_dir(&self) -> PathBuf {
            self.0.join("com.ahpio.desk")
        }
        /// The old folder with a host list and a file in a subfolder.
        fn with_old(self) -> Self {
            fs::create_dir_all(self.old().join("ahpd")).unwrap();
            fs::write(self.old().join("hosts.json"), "[old]").unwrap();
            fs::write(self.old().join("ahpd").join("package.json"), "{}").unwrap();
            self
        }
    }
    impl Drop for Scratch {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn the_config_is_copied_and_the_old_one_stays() {
        let s = Scratch::new().with_old();
        assert!(copy_config(&s.new_dir()).unwrap());
        assert_eq!(
            fs::read_to_string(s.new_dir().join("hosts.json")).unwrap(),
            "[old]"
        );
        assert!(s.new_dir().join("ahpd").join("package.json").is_file());
        assert!(s.old().join("hosts.json").is_file());
        assert!(!s.0.join(format!("{OLD_ID}.copying")).exists());
    }

    #[test]
    fn the_local_data_is_moved_and_the_old_one_goes() {
        let s = Scratch::new().with_old();
        assert!(move_local_data(&s.new_dir()).unwrap());
        assert!(s.new_dir().join("ahpd").join("package.json").is_file());
        assert!(!s.old().exists());
    }

    #[test]
    fn an_empty_new_folder_is_taken_over() {
        let s = Scratch::new().with_old();
        fs::create_dir_all(s.new_dir()).unwrap();
        assert!(copy_config(&s.new_dir()).unwrap());
        assert!(s.new_dir().join("hosts.json").is_file());
    }

    #[test]
    fn a_new_folder_with_anything_in_it_is_left_alone() {
        let s = Scratch::new().with_old();
        fs::create_dir_all(s.new_dir()).unwrap();
        fs::write(s.new_dir().join("hosts.json"), "[new]").unwrap();
        assert!(!copy_config(&s.new_dir()).unwrap());
        assert!(!move_local_data(&s.new_dir()).unwrap());
        assert_eq!(
            fs::read_to_string(s.new_dir().join("hosts.json")).unwrap(),
            "[new]"
        );
        assert!(s.old().is_dir());
    }

    #[test]
    fn with_no_old_folder_nothing_is_made() {
        let s = Scratch::new();
        assert!(!copy_config(&s.new_dir()).unwrap());
        assert!(!move_local_data(&s.new_dir()).unwrap());
        assert!(!s.new_dir().exists());
    }

    #[test]
    fn a_copy_left_halfway_is_made_again() {
        let s = Scratch::new().with_old();
        let part = s.0.join(format!("{OLD_ID}.copying"));
        fs::create_dir_all(&part).unwrap();
        fs::write(part.join("hosts.json"), "[half]").unwrap();
        assert!(copy_config(&s.new_dir()).unwrap());
        assert_eq!(
            fs::read_to_string(s.new_dir().join("hosts.json")).unwrap(),
            "[old]"
        );
    }
}
