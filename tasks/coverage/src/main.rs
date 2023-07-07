use hdx_coverage::AppArgs;
use pico_args::Arguments;

fn main() {
    let mut args = Arguments::from_env();
    let command = args.subcommand().expect("subcommands");

    let args = AppArgs {
        filter: args.opt_value_from_str("--filter").unwrap(),
        update: args.contains("--update"),
        diff: args.contains("--diff"),
    };

    let task = command.as_deref().unwrap_or("default");

    match task {
        "parser" => args.run_parser(),
        "lexer" => args.run_lexer(),
        _ => args.run_all(),
    };
}
