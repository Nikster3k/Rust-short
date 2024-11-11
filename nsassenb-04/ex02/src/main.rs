use std::{env, os::unix::fs::MetadataExt, path::Path};


struct  FileSize {
	size: usize
}

impl std::fmt::Display for FileSize {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self.size {
			(0..1000) => f.write_fmt(format_args!("{} bytes", self.size)),
			(1000..1000000) => f.write_fmt(format_args!("{:.1} kilobytes", (self.size as f32 / 1000.0))),
			(1000000..1000000000) => f.write_fmt(format_args!("{:.1} megabytes", (self.size as f32 / 1000000.0))),
			_ => f.write_fmt(format_args!("{:.1} gigabytes", (self.size as f32 / 1000000000.0))),
		}
	}
}

fn calc_size_for_dir(path: &Path, size: &mut FileSize) -> usize {
	if let Ok(entries) = std::fs::read_dir(path) {
		for entry in entries.flatten() {
			if let Ok(metadata) = entry.metadata() {
				if metadata.is_dir() {
					// print!("GO INTO DIR");
					calc_size_for_dir(entry.path().as_path(), size);
				} else {
					size.size += metadata.size() as usize;
				}
			}
		}
	}
	size.size
}

fn main() {
	if env::args().len() != 2 {
		return;
	}
	let mut size = FileSize{size: 0};
	calc_size_for_dir(Path::new(&env::args().nth(1).unwrap()), &mut size);
	std::println!("{}", size);
}
