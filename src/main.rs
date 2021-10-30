use std::process::exit;

use clap::{Arg, App};

mod pool;
use pool::interpret;

use crate::graphical::graphical_interpret;

mod graphical;
mod util;



fn main() {
    let argument_matches = App::new("pool interpreter")
                    .version("0.1.0")
                    .author("hyde <hiddy.tiddey@gmail.com>")
                    .about("interpreter for the pool 2d language")
                    .arg(Arg::with_name("filename")
                        .takes_value(true)
                        .value_name("FILE")
                        .help("set filename")
                        .index(1)
                    )
                    .arg(Arg::with_name("graphical_mode")
                        .short("g")
                        .long("graphical")
                        .takes_value(false)
                        .help("run program in graphical mode")
                    )
                    .get_matches();

        
    let f = argument_matches.value_of("filename");
    
    let graphical_mode = argument_matches.occurrences_of("graphical_mode") >= 1;

    if let Some(filename) = f {
        let grid = util::read_file(filename).expect("file not found");
        if graphical_mode {
            let exit_code = graphical_interpret(grid);
            exit((exit_code.or::<i64>(Ok(-1)).expect(".") & 0xffffffff) as i32);
        } else {
            let exit_code = interpret(grid);
            exit((exit_code & 0xffffffff) as i32);
        }
    } else {
        println!("no file specified");
    }
}
