use std::process::ExitCode;

pub enum CliError {
	ParseFailed,
	Checks(usize),
	FilesAndStdin,
	#[allow(dead_code)]
	Io(std::io::Error),
	Fmt(std::fmt::Error),
	SerdeJson(serde_json::Error),
}

impl From<std::io::Error> for CliError {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl From<std::fmt::Error> for CliError {
	fn from(err: std::fmt::Error) -> Self {
		Self::Fmt(err)
	}
}

impl From<serde_json::Error> for CliError {
	fn from(err: serde_json::Error) -> Self {
		Self::SerdeJson(err)
	}
}

impl std::fmt::Debug for CliError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ParseFailed => write!(f, "Parsing Failed!"),
			Self::FilesAndStdin => {
				write!(f, "Specified multiple files including stdin. Try passing just files, or use `-` for stdin.")
			}
			Self::Checks(i) => f.write_str(&format!("{i} files failed check!")),
			Self::Io(arg0) => f.debug_tuple("::io::Error").field(arg0).finish(),
			Self::Fmt(arg0) => f.debug_tuple("::fmt::Error").field(arg0).finish(),
			Self::SerdeJson(arg0) => f.debug_tuple("::serde_json::Error").field(arg0).finish(),
		}
	}
}

impl From<CliError> for ExitCode {
	fn from(val: CliError) -> Self {
		match val {
			CliError::Checks(i) => (i as u8).into(),
			_ => ExitCode::FAILURE,
		}
	}
}

pub type CliResult = Result<(), CliError>;
