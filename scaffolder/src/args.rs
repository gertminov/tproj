use getopts::Options;

pub struct InputArgs {
    pub open_latest: bool,
    pub open_explorer: bool,
    pub open_both: bool,
    pub clean: bool,
    pub init: bool,
    pub help: bool,
    pub project_name: Option<String>
}
pub fn parse_args(args: Vec<String>) -> InputArgs {

    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("e", "explorer", "opens explorer in newly created folder");
    opts.optflag(
        "b",
        "both",
        "opens both new terminal window and explrer in created folder",
    );
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("l", "latest", "open latest project");
    opts.optflag("c", "clean", "clean up old projects");
    opts.optflag("i", "init", "initialize config file");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let help =  matches.opt_present("h");

    if help {
        print_usage(&program, opts)
    }

    let project_name = matches.free.first().map(|s| s.clone());

    let open_latest = matches.opt_present("l");
    let open_explorer = matches.opt_present("e");
    let open_both = matches.opt_present("b");
    let clean = matches.opt_present("c");
    let init = matches.opt_present("i");

    return InputArgs{open_latest, open_explorer, open_both, clean, help, project_name, init};
}
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [name] [options]", program);
    print!("{}", opts.usage(&brief))
}
