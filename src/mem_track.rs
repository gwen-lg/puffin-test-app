use alloc_track::{BacktraceMetric, BacktraceMode};
use chrono::{DateTime, Local};
use std::{
	fs::File,
	io::{self, Write},
};

pub struct MemTrack {
	time: DateTime<Local>,
}

impl MemTrack {
	pub fn init() -> Self {
		let time = Local::now();
		Self { time }
	}

	pub fn report(&self) -> Result<(), io::Error> {
		let now = self.time.format("%Y-%m-%d-%T").to_string();
		let filename = format!("alloc_backtrace_{now}.txt");
		let mut file = File::create(filename)?;

		// Summary
		let backtrace_report = alloc_track::backtrace_report(|_, _| true);
		let summary =
			backtrace_report
				.0
				.iter()
				.fold(BacktraceMetric::default(), |val, (_, cur_metric)| {
					BacktraceMetric {
						allocated: val.allocated + cur_metric.allocated,
						freed: val.freed + cur_metric.freed,
						allocations: val.allocations + cur_metric.allocations,
						mode: BacktraceMode::None,
					}
				});
		writeln!(&mut file, "Summary : \n{summary}")?;

		// with Filter and Sort
		let mut backtrace_report =
			alloc_track::backtrace_report(|_, metrics| metrics.allocations > 10);
		backtrace_report.0.sort_unstable_by(|(_, a), (_, b)| {
			a.allocations.partial_cmp(&b.allocations).unwrap().reverse()
		});
		writeln!(&file, "Details : \n{backtrace_report}")?;

		Ok(())
	}
}
