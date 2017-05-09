
extern crate tiles;
extern crate getopts;

mod map;

use tiles::print;
use std::env;
use std::io;

struct MainOptions {
    map   : map::MapOptions,
    print : print::PrintOptions,
}

fn main() {
    let options = parse_opts( env::args().collect() );

    let     map = map::new_map( options.map );
    let mut out = io::stdout();

    out.lock();
    print::print_map( options.print, & map, &mut out );
}

fn parse_opts(
    args : Vec<String>,
) -> MainOptions {
    let program = args[0].clone();
    let opts    = new_opts();

    let mut width  = 50;
    let mut height = 50;
    let mut colour = print::PrintColourOptions::Off;
    let mut seed   = Some( 0 as usize );

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(_) => {
            println!("Unknown argument given");
            print_help_and_exit( &program, opts );
        },
    };

    if matches.opt_present("?") {
        print_help_and_exit( &program, opts );
    }

    if matches.opt_present("c") {
        colour = print::PrintColourOptions::On;
    }

    match matches.opt_str("w") {
        Some( width_str ) => width = width_str.parse::<u32>().unwrap(),
        None              => { /* do nothing */ },
    };

    match matches.opt_str("h") {
        Some( height_str ) => height = height_str.parse::<u32>().unwrap(),
        None               => { /* do nothing */ },
    };

    seed = match matches.opt_str("s") {
        Some( seed_str ) => Some( seed_str.parse::<usize>().unwrap() ),
        None             => None,
    };

    return MainOptions {
        map : map::MapOptions {
            width   : width,
            height  : height,
            seed    : seed,
        },

        print : print::PrintOptions {
            colour  : colour,
        },
    }
}

fn new_opts() -> getopts::Options {
    let mut opts = getopts::Options::new();

    opts.optopt( "s", "seed"  , "set seed for random generator", "SEED" );
    opts.optopt( "w", "width" , "set width for map", "WIDTH"            );
    opts.optopt( "h", "height", "set height for map", "HEIGHT"          );
    opts.optflag( "c", "colour", "set to use colours on output"         );
    opts.optflag( "?", "help"  , "print this help menu"                 );

    return opts;
}

fn print_missing_value_and_exit( name : &'static str ) -> ! {
    print!("value missing for property {}", name);
    std::process::exit(0);
}

fn print_help_and_exit( program: &str, opts: getopts::Options ) -> ! {
    print_help( program, opts );
    std::process::exit(0);
}

fn print_help( program: &str, opts: getopts::Options ) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

