use std::fs;

pub fn create_config() -> std::io::Result<()> {
    fs::create_dir("~/.config/felis/")?;
    let _make_file = fs::File::create("~/.config/felis/config");
    Ok(())
}
