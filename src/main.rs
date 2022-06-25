use clap::{Args, Parser, Subcommand, ValueEnum};
use rand::random;
use std::{thread, time};

#[derive(Clone, ValueEnum)]
//#[clap(ARG ENUM ATTRIBUTE)]
enum EnumValues {
	/// Doc comment
	//#[clap(POSSIBLE VALUE ATTRIBUTE)]
	Limited,
}

#[derive(Args, Clone, Copy, Debug)]
//#[clap(PARENT APP ATTRIBUTE)]
struct Struct {
	/// Doc comment
	//#[clap(ARG ATTRIBUTE)]
	field: u32,
}

#[derive(Clone, Copy, Subcommand)]
enum LoopBehavior {
	Unlimited,
	Limited(Struct),
}
impl Default for LoopBehavior {
	fn default() -> Self {
		LoopBehavior::Unlimited
	}
}

#[derive(PartialEq, Clone, Copy, clap::ValueEnum)]
enum LoadingBehavior {
	None,
	PreLoop,
	FirstLoop,
	//Threaded, TODO: add thread loading management
}
impl Default for LoadingBehavior {
	fn default() -> Self {
		LoadingBehavior::None
	}
}


#[derive(Parser)]
#[clap(name = "Puffin-Test-App")]
#[clap(author, version, about, long_about = None)]
struct AppBehavior {
	/// Indicate to wait profiler connection before continue app execution
	#[clap(short, long, value_parser, default_value_t = false)]
	pub wait_profiler: bool,

	/// Choose loading behavior of the test app
	#[clap(short, long, value_enum, default_value_t = LoadingBehavior::None)]
	pub loading: LoadingBehavior,

	#[clap(subcommand)] //TODO: enable loop behavior management
	pub loop_behavior: LoopBehavior,
}

fn main() {
	let app_behavior = AppBehavior::parse();

	let server_addr = format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT);
	eprintln!("Serving demo profile data on {}", server_addr);
	let puffin_server = puffin_http::Server::new(&server_addr).unwrap();

	puffin::set_scopes_on(true); // need this to enable capture

	// wait client(s) connection
	if app_behavior.wait_profiler {
		while puffin_server.num_clients() == 0 {}
	}

	simulate_loading(app_behavior.loading, LoadingBehavior::PreLoop);

	let mut loop_count = 0;
	while continue_loop(app_behavior.loop_behavior, loop_count) {
		puffin::profile_scope!("main_loop", format!("loop num : {}", loop_count));
		puffin::GlobalProfiler::lock().new_frame();

		if loop_count == 0 {
			// Big sleep to simulate loading
			simulate_loading(app_behavior.loading, LoadingBehavior::FirstLoop);
		}

		loop_count += 1;

		let loop_duration = compute_loop_duration();
		{
			puffin::profile_scope!("sleep");
			let sleep_duration = time::Duration::from_millis(loop_duration);
			thread::sleep(sleep_duration);
		}
		println!("loop {} duration {}ms", loop_count, loop_duration);
	}
}

fn simulate_loading(behavior: LoadingBehavior, step: LoadingBehavior) {
	if behavior == step {
		puffin::profile_scope!("loading");
		let sleep_duration = time::Duration::from_secs(5);
		thread::sleep(sleep_duration);
	}
}

fn compute_loop_duration() -> u64 {
	const MIN_DURATION: u64 = 25;
	const MAX_DURATION: u64 = 105;
	let rand_value: u64 = random::<u64>() % (MAX_DURATION - MIN_DURATION);
	MIN_DURATION + rand_value
}

fn continue_loop(loop_behavior: LoopBehavior, loop_count: u32) -> bool {
	let continue_loop = match loop_behavior {
		LoopBehavior::Unlimited => true,
		LoopBehavior::Limited(limit) => loop_count < limit.field,
	};
	continue_loop
}
