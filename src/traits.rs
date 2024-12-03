pub trait EOSDetector: Iterator {
	fn reached_eos(&mut self) -> bool;
}
