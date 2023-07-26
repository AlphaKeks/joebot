use {
	std::io::stderr,
	time::macros::format_description,
	tracing::info,
	tracing_subscriber::{
		fmt::{format::FmtSpan, time::UtcTime},
		EnvFilter,
	},
};

pub fn setup() {
	let timer = UtcTime::new(format_description!(
		"[[[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]]"
	));

	let filter = EnvFilter::from_default_env();

	tracing_subscriber::fmt()
		.pretty()
		.with_timer(timer)
		.with_file(true)
		.with_line_number(true)
		.with_span_events(FmtSpan::ACTIVE)
		.with_env_filter(filter)
		.with_writer(stderr)
		.init();

	info!("Initialized logging");
}
