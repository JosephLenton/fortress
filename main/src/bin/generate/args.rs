use structopt::StructOpt;

///
/// The structure of the commands.
///
/// A large amount of this is generated by `StructOpt`.
/// See that project for how to write large amounts of this.
///
/// The gist however is that we make a struct that will hold
/// all of our arguments. Commands are then parsed, and then
/// turned into this struct.
///
#[derive(StructOpt, Debug)]
#[structopt(name = "Map Generator", about = "Generates maps.")]
pub struct Args {
    /// A seed for the map.
    /// By seeding the map you can generate the same map again and again,
    /// using the same seed value.
    #[structopt(short = "s", long = "seed", help = "Seed for the random generator.")]
    pub seed: Option<usize>,

    /// The width of the map we are building.
    #[structopt(short = "w", default_value = "50", long = "width",
                help = "Set width for the map.")]
    pub width: u32,

    /// The height of the map we are building.
    #[structopt(short = "h", default_value = "50", long = "height",
                help = "Set height for the map.")]
    pub height: u32,

    /// Should colour be on or off.
    #[structopt(subcommand)]
    pub colour: Option<ArgsColour>,
}

///
/// For turning colour on or off.
///
#[derive(StructOpt, Debug, PartialEq, Eq)]
pub enum ArgsColour {
    colour,
    no_colour,
}

impl Args {
    /// Parses the command line arguments, and returns a new Args struct which
    /// holds the arguments given.
    ///
    /// If the arguments fail to parse then this will automatically end the
    /// application.
    #[allow(unreachable_pub)]
    pub fn new_from_args() -> Args {
        Args::from_args()
    }
}
