
# Fortress

A work in progress Dwarf Fortress clone.

Note this repo is a mono repo containing multiple projects within.

## Projects

This is organised as a monorepo. The code base is split into multiple smaller
projects to help organise where and how they can be used.

    ├─ world     How data is represented. Holds all the core data and
    │            definitions that everything else uses. For example creatures,
    │            tiles, maps, and colours. Basically everything depends on
    │            this.
    │
    ├─ main      Holds the main binary that runs the application.
    │
    ├─ generate  The map generation logic is here.
    │
    ├─ game      This is how you run the game play. It is headless however.
    │            It has no graphics, and no way of drawing or displaying items.
    │            It does have a UI however. It just doesn't know how to draw it.
    │
    ├─ head      This is what will draw the game to the screen, and offers
    │            events to hook into. Currently this only offers SDL2.
    │
    └─ util      Data structures which are in no way specific to anything.
                 i.e. points, areas, sizes, and things like that.

