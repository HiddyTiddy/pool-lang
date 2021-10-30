use clap::{Arg, App};

mod pool;
use pool::interpret;

mod util;



fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() == 1 {
    //     eprintln!("missing arguments");
    //     return;
    // }
    // let filename = &args[1];
    let argument_matches = App::new("pool interpreter")
                    .version("0.1.0")
                    .author("hyde <hiddy.tiddey@gmail.com>")
                    .about("interpreter for the pool 2d language")
                    .arg(Arg::with_name("filename")
                        .takes_value(true)
                        .value_name("FILE")
                        .help("set filename")
                        .index(1)
                    ).get_matches();

        
    let f = argument_matches.value_of("filename");
    
    if let Some(filename) = f {
        let grid = util::read_file(filename).expect("file not found");
        interpret(grid);   
    } else {
        println!("no file specified");
    }
}
