extern crate walkdir;
extern crate dirs;
mod common;

fn main() {

  let home_dir = dirs::home_dir().unwrap();

  let search_file_name = "ssh-config";
  let config_path = home_dir.as_path().join(".ssh/config");
  let ssh_config_root_path = home_dir.as_path().join("clients");

  common::backup_file_with_date(config_path.to_str().unwrap());

  let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(config_path)
        .unwrap();

  for entry in walkdir::WalkDir::new(ssh_config_root_path.as_path())
  .follow_links(false)
  .into_iter()
  .filter_map(|e| e.ok())
  .filter(|e| !e.file_type().is_dir())
  .filter(|e| e.file_name().to_str() == Some(search_file_name))
  {
    println!("{}", entry.path().display());

    let path_str = entry.path().to_str().unwrap();
    common::append_to_file(&mut file, &path_str);
  }
}