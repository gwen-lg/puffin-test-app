use rand::random;
use std::{thread, time};

fn main() {
	let mut loop_count = 0;
	loop {
		loop_count += 1;

		let loop_duration = compute_loop_duration();
		let sleep_duration = time::Duration::from_millis(loop_duration);
		thread::sleep(sleep_duration);
		println!("loop {} duration {}ms", loop_count, loop_duration);
	}
}

fn compute_loop_duration() -> u64 {
	const MIN_DURATION: u64 = 10;
	const MAX_DURATION: u64 = 55;
	let rand_value: u64 = random::<u64>() % (MAX_DURATION - MIN_DURATION);
	MIN_DURATION + rand_value
}
