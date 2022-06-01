use std::{thread, time};

fn main() {
	let mut loop_count = 0;
	loop {
		loop_count += 1;

		let loop_duration = 33;
		let sleep_duration = time::Duration::from_millis(loop_duration);
		thread::sleep(sleep_duration);
		println!("loop {} duration {}ms", loop_count, loop_duration);
	}
}
