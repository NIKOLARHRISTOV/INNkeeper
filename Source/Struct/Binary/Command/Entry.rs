pub type Entry = Vec<Vec<&'static str>>;

pub struct Struct {
	pub Command: Command,
	pub Entry: Entry,
	pub Parallel: Parallel,
	pub Pattern: Pattern,
	pub Separator: Separator,
}

impl Struct {
	pub fn Fn(Option: Option) -> Self {
		Self {
			Command: Option.Command,
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern,
			Separator: Option.Separator,
		}
	}
}

use crate::Struct::Binary::Command::Option::{
	Command, Parallel, Pattern, Separator, Struct as Option,
};
