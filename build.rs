use std::{env, fs};
use std::path::{Path, PathBuf};


fn get_output_path() -> PathBuf {
	let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
	let build_type = env::var("PROFILE").unwrap();
	let path = Path::new(&manifest_dir_string).join("target").join(build_type);
	return PathBuf::from(path);
}

fn main() {
	let target_dir = get_output_path();
	fs::copy(Path::join(&env::current_dir().unwrap(), "SDL2.dll"), Path::join(Path::new(&target_dir), Path::new("SDL2.dll"))).unwrap();
	fs::copy(Path::join(&env::current_dir().unwrap(), "SDL2_ttf.dll"), Path::join(Path::new(&target_dir), Path::new("SDL2_ttf.dll"))).unwrap();
	let _ = fs::DirBuilder::new().create(Path::join(&target_dir, "fonts"));
	fs::copy(Path::join(&env::current_dir().unwrap(), "fonts/EnvyCodeR_regular.ttf"), Path::join(&target_dir, "fonts/EnvyCodeR_regular.ttf")).unwrap();
	fs::copy(Path::join(&env::current_dir().unwrap(), "fonts/EnvyCodeR_bold.ttf"), Path::join(&target_dir, "fonts/EnvyCodeR_bold.ttf")).unwrap();
	fs::copy(Path::join(&env::current_dir().unwrap(), "fonts/EnvyCodeR_italic.ttf"), Path::join(&target_dir, "fonts/EnvyCodeR_italic.ttf")).unwrap();
}