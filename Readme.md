
# Fortress

A work in progress Dwarf Fortress clone.

Note this repo is a mono repo containing multiple projects within.

## Projects

This is organised as a monorepo. The code base is split into multiple smaller
projects to help organise where and how they can be used.

 * *Fortress* Holds all the core data and definitions that everything else 
   hooks into. For example maps, tiles, and colours. Basically everything 
   depends on this.
 * *Main* Holds the main binary that runs the application.
 * *Generate* The map generation logic is here.
 * *Game* This is what can run the game. It is headless however. It has no
   graphics, and no way of drawing or displaying items.
   It does have a UI however. It just doesn't know how to draw it.
 * *Head* This is what will draw the game to the screen, and offer events to
   hook into. Currently this only offers SDL2.
 * *Util* Data structures which are in no way specific to anything. Like 
   points, areas, sizes, and things like that.

