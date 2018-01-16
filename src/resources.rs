pub mod shaders {
	pub static BASIC_TRANSFORM2_VS: &'static str = include_str!("../assets/basic_transform2.vs");
	pub static BASIC_TRANSFORM_VS: &'static str = include_str!("../assets/basic_transform.vs");
	pub static BASIC_COLOR_FS: &'static str = include_str!("../assets/basic_color.fs");

	pub static KALEIDOSCOPE_FS: &'static str = include_str!("../assets/kaleidoscope.fs");

	pub static PAPER_VS: &'static str = include_str!("../assets/paper.vs");
	pub static PAPER_FS: &'static str = include_str!("../assets/paper.fs");
}