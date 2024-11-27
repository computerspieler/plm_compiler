#[macro_export]
macro_rules! parsing_error {
	($pos: ident, $msg: tt) => { {
		dbg!("TRACE: file: {}, line: {}", file!(), line!());
		println!("Parsing error at {}: {}", $pos, $msg);
		return None;
	} };
	($start: ident, $end: ident, $msg: tt) => { {
		dbg!("TRACE: file: {}, line: {}", file!(), line!());
		println!("Parsing error between {} and {}: {}",
			$start, $end, $msg);
		return None;
	} };
	($msg: tt) => { {
		dbg!("TRACE: file: {}, line: {}", file!(), line!());
		// Is it supposed to be the end ? We'll suppose that
		// for now, but it could be otherwise
		println!("Parsing error at the end: {}", $msg);
		return None;
	} };
}

#[macro_export]
macro_rules! check_token {
	($tok: expr, $goal: pat) => {
		match ($tok) {
		Some(($goal, _)) => { }
		Some((_, pos)) => {
			parsing_error!(pos, "Expected another token type")
		}
		None => {
			parsing_error!("Missing token")
		}
		}
	};
}

pub use parsing_error;
pub use check_token;
