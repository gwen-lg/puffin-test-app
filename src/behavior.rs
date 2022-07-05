#[derive(Clone, Copy)]
pub enum LoopBehavior {
	Unlimited,
	Limited(u32),
}

pub fn compute_loop_behavior(nb_loop: i32) -> LoopBehavior {
	if nb_loop < 0 {
		LoopBehavior::Unlimited
	} else {
		LoopBehavior::Limited(nb_loop as u32)
	}
}
