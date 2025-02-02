use std::path::Path;

pub struct Configuration {
	program_base: u32,

	listing: Option<Box<Path>>,
	binary: Path,
}
