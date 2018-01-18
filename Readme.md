
# Fortress

A work in progress Dwarf Fortress clone.

Note this repo is a mono repo containing multiple projects within.

## Projects

This is organised as a monorepo. The code base is split into multiple smaller
projects to help organise where and how they can be used.

    ├─ main      Entry points. These are the binaries you can run.
    │   │        Currently there are only two.      
    │   │
    │   ├─ fortress     Runs the game.
    │   └─ generate     For map generation.
    │
    ├─ generate  The map generation logic is here.
    │
    ├─ game      This is the engine for running the game. It is headless.
    │            It has no graphics, and no way of drawing or displaying items.
    │            It does have a UI however. It just doesn't know how to draw it.
    │
    ├─ head      This is what will draw the game to the screen, and offers
    │            events to hook into. Currently this only offers SDL2.
    │
    ├─ world     How data is represented. Holds all the core data and
    │            definitions that everything else uses. For example creatures,
    │            tiles, maps, and colours. Basically everything depends on
    │            this.
    │
    └─ util      Data structures which are in no way specific to anything.
                 i.e. points, areas, sizes, and things like that.

