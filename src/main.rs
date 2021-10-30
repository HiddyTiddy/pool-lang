use std::env;

mod pool;
use pool::interpret;

mod util;



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("missing arguments");
        return;
    }
    let filename = &args[1];
    let grid = util::read_file(filename).expect("file not found");
    interpret(grid);
}
