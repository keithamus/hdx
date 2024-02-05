use hdx_benchmark::AppArgs;
use pico_args::Arguments;

fn main() {
	let mut args = Arguments::from_env();
	let command = args.subcommand().expect("subcommands");

	let args = AppArgs {
		filter: args.opt_value_from_str("--filter").unwrap(),
		save: args.opt_value_from_str("--save").unwrap(),
	};

	let task = command.as_deref().unwrap_or("default");

	match task {
		// "parser" => args.run_parser(),
		"lexer" => args.run_lexer(),
		// "minifier" => args.run_minifier(),
		_ => args.run_all(),
	};
}
