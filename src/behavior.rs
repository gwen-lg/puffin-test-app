#[derive(Clone, Copy)]
pub enum LoopBehavior {
	Unlimited,
	Limited(u32),
}
