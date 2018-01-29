/// Represents the various types of ground.
#[derive(Copy, Clone)]
pub enum Tile {
    /// An empty tile.
    /// There is nothing here, and you cannot stand on it.
    Empty,

    /// Normal ground.
    Ground,

    /// Grassy ground.
    /// Like grass in a park.
    Grass,

    /// Thick grass.
    /// Like when you get grass that goes up to your knees.
    GrassThick,

    /// A hill.
    Hill,

    /// Rocky ground.
    /// You can walk over this.
    Rocks,

    /// Water.
    Water,

    /// A wall.
    Wall,

    /// Treestump.
    TreeStump,

    /// Ice.
    Ice,
}
