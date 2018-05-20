
# Unreleased (2018-05-20):
- [Added] deadend and braid algorithms for creating loops in mazes
    Changed dijkstra to avoid issues with loops
    Added additional link helper methods to MazeGrid
- [Changed] aldous\_broder, wilsons, and hunt\_and\_kill to work with MazeGrids
- [Changed] polar grid drawing to include colorization
    Added furthest\_on_rim to get point furthest on the rim of a polar grid
    from another point
- [Added] polar\_png for drawing polar grids
    Changed backtracker algorithms to be generalized for MazeGrid
    Added circle example
- [Added] PolarGrid and implementations for PolarGrid and PolarCell
    Additionally refactored traits to have more default implementations
- [Changed] organization of data structs to make development easier
    Created data::grid, data::cell, and data::pos. Refactored where needed
- [Changed] dijkstra and solver interfaces to use traits
    - [Fixed] trait associated types to use PositionType dictated by the cells
- [Added] MazeCell, MazeGrid, MazePosition traits
    Refactored the existing Cell, Grid, and Position to implement these
    traits
- [Changed] dijkstra and solver to be iterative
- [Added] recursive and iterative backtracker implementations
    The inclusion of the iterative one is to avoid blowing up the stack for
    large inputs
- [Added] furthest\_corners function
    This will find two corners that are the furthest distance apart in the
    maze
- [Added] hunt and kill algorithm and example
    Also some minor fixes
    - [Changed] binary algorithm to not use links list and instead just modify
    - [Fixed] test that still had old definition of color fn
    - [Improved] ran cargo fmt
- [Fixed] default color function and color functions in general
    The color function now takes max weight into consideration
    - [Changed] Dijkstra algorithm to require starting position
    - [Changed] solve to apply Dijkstra by itself and to require start, target
    - [Changed] examples
- [Added] implementation for Wilson's + example
    - [Changed] code that was using HashMaps to use HashSets where appropriate
- [Added] stubs for wilson
- [Fixed] actually added solver
- [Changed] style to use constants for default cell and wall measurements
- [Added] style options to render solutions and weight map
    - [Added] in\_solution to Cell
    - [Changed] weight of Cell to u32
    - [Added] color\_fn, draw\_solution, and solution\_color to Style
    - [Changed] png export to render weight map and solution
    - [Changed] aldous broder example to draw full png
- [Added] implementation of png rendering for grids
    - [Added] Style and StyleBuilder for setting image rendering styles
    - [Added] is\_linked\_pos to Cell for determining linked status from a Position
- [Added] image and imageproc dependencies
- [Added] benchmarks
- [Added] algorithms module, refactor examples
- [Added] aldous broder example and neighbors methods for grid and cell
- [Added] sidewinder implementation and cell direction references
- [Added] readme
- [Added] sidewinder stub
- [Added] binary tree search example
- [Changed] grid.to\_string to allow for toggling labels
- [Added] tests for contains and remaining getters
- [Added] initial commit
