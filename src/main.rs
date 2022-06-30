mod behavior;

use behavior::{LoadingBehavior, LoopBehavior};
use clap::Parser;
use rand::random;
use simplelog::{Config, LevelFilter, SimpleLogger};
use std::{
	thread::{self, JoinHandle},
	time,
};

#[derive(Parser)]
#[clap(name = "Puffin-Test-App")]
#[clap(author, version, about, long_about = None)]
struct Args {
	/// Set the level of logging in console
	#[clap(short, long, default_value_t = LevelFilter::Info)]
	pub log_level: LevelFilter,

	/// Indicate the number of loop wanted. -1 is for unlimited.
	#[clap(short, long, default_value_t = -1)]
	nb_loop: i32,

	/// Choose loading behavior of the simulation
	#[clap(long, value_enum, default_value_t = LoadingBehavior::None)]
	loading: LoadingBehavior,
}

fn main() {
	let args = Args::parse();

	let filter_level = args.log_level;
	SimpleLogger::init(filter_level, Config::default()).unwrap();

	let server_addr = format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT);
	log::info!("Serving demo profile data on {}", server_addr);
	let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();

	puffin::set_scopes_on(true); // need this to enable capture

	simulate_loading(args.loading, LoadingBehavior::PreLoop);

	let loading_thread_handle = if args.loading == LoadingBehavior::Threaded {
		let thread_handle = thread::Builder::new()
			.name("Loading".to_string())
			.spawn(loading)
			.unwrap();
		Some(thread_handle)
	} else {
		None
	};

	let loop_behavior = behavior::compute_loop_behavior(args.nb_loop);

	let mut loop_count = 0;
	while continue_loop(loop_behavior, loop_count) {
		loop_count += 1;

		puffin::profile_scope!("main_loop", format!("loop num : {}", loop_count));
		puffin::GlobalProfiler::lock().new_frame();

		let start_time = time::Instant::now();
		log::info!("loop {} ... start", loop_count);

		if loop_count == 0 {
			// Big sleep to simulate loading
			simulate_loading(args.loading, LoadingBehavior::FirstLoop);
		}

		let loop_duration = compute_loop_duration();
		{
			puffin::profile_scope!("sleep");
			let sleep_duration = time::Duration::from_millis(loop_duration);
			thread::sleep(sleep_duration);
		}

		let loop_duration = time::Instant::now() - start_time;
		log::info!(
			"loop {} duration {}ms",
			loop_count,
			loop_duration.as_millis()
		);
	}

	wait_end_loading(loading_thread_handle);

	puffin::GlobalProfiler::lock().new_frame(); // Needed to finalise last loop frame
}

fn loading() {
	puffin::profile_scope!("Threaded loading");
	println!("loading started");
	let loading_duration = 7;
	let sleep_duration = time::Duration::from_secs(loading_duration);
	thread::sleep(sleep_duration);
	println!("loading duration {}s", loading_duration);
}

fn wait_end_loading(loading_thread_handle: Option<JoinHandle<()>>) {
	// wait end of loading if needed
	if let Some(thread_handle) = loading_thread_handle {
		let start_time = time::Instant::now();
		match thread_handle.join() {
			Ok(()) => {}
			Err(err) => {
				eprintln!("Loading thread error : {:?}", err);
			}
		}
		let wait_duration = time::Instant::now() - start_time;
		println!("loading thread waited {}ms", wait_duration.as_millis());
	}
}

fn simulate_loading(behavior: LoadingBehavior, step: LoadingBehavior) {
	if behavior == step {
		puffin::profile_scope!("loading");
		let loading_duration = 5;
		let sleep_duration = time::Duration::from_secs(loading_duration);
		println!("loading duration {}s", loading_duration);
		thread::sleep(sleep_duration);
	}
}

fn compute_loop_duration() -> u64 {
	const MIN_DURATION: u64 = 10;
	const MAX_DURATION: u64 = 55;
	let rand_value: u64 = random::<u64>() % (MAX_DURATION - MIN_DURATION);
	MIN_DURATION + rand_value
}

fn continue_loop(loop_behavior: LoopBehavior, loop_count: u32) -> bool {
	match loop_behavior {
		LoopBehavior::Unlimited => true,
		LoopBehavior::Limited(limit) => loop_count < limit,
	}
}

#[test]
fn verify_cli() {
	use clap::CommandFactory;
	Args::command().debug_assert()
}
