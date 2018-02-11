
# Fortress

A work in progress Dwarf Fortress clone.

Note this repo is a mono repo containing multiple projects within.

## Projects

This is organised as a monorepo. The code base is split into multiple smaller
projects to help organise where and how they can be used.

    │
    ├─ main      Entry points. These are the binaries you can run.
    │   │        Currently there are only two.      
    │   │
    │   ├─ fortress     Runs the game.
    │   └─ generate     For map generation.
    │
    ├─ game      This is the engine for running the game. It is headless.
    │            It has no graphics, and no way of drawing or displaying items.
    │            It does have a UI however. It just doesn't know how to draw it.
    │
    ├─ generate  Map generation logic.
    │
    ├─ head      High Level Renderer. This is what draws the map to the screen,
    │            decides the players colour (and draws them), and so on.
    │            It manages a lot of drawing items which don't need to care 
    │            about how items are drawn. Like the camera position, and 
    │            offsetting the items when drawn.
    │
    │            For the actual drawing this uses the low level renderer.
    │
    ├─ llr       Low Level Renderer. This is the drawing implementation.   
    │   │        It uses a generic implementation, and under that it is 
    │   │        implemented. The workings of the implementations are hidden.
    │   │        This is to allow the implementations to be swapped in and out.
    │   │        Whilst this is a drawing backend, it is unconventional.
    │   │        It has a very precise API catered to the style of the game.
    │   │
    │   ├─ llr_sdl2     Uses SDL2 for the running and rendering.
    │   │               This is probably the best way to run the game for release.
    │   │               release builds.
    │   │
    │   └─ llr_terminal Runs the game in the terminal. It's a bit slow, and 
    │                   it's far from perfect, but it works! Useful for local
    │                   development.
    │
    ├─ world     How data is represented. Holds all the core data and
    │            definitions that everything else uses. For example creatures,
    │            tiles, maps, and colours. Basically everything depends on
    │            this.
    │
    └─ util      Data structures which are in no way specific to anything.
                 i.e. points, areas, sizes, and things like that.

