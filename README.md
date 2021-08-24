# `cic` Change Interactive CSV

A command line utility for viewing and editing csvs.  Work in progress.

Goal is to have somethat that appears like excel, taking up the entire terminal window and uses vim-bindings to navigate to different cells to edit them.  This will NOT have any "excel" computational functionality, it will only store/edit static data (that is, csvs).

## Notes

### Keybinds

hjkl movement
c update cell with new value (prompt for overwrite)
a edit cell value (prompt with update)
dd delete row
o new row

### Structure

// a simple dataframe
struct Table
    data: Vec<Vec<String>>

    from_csv(stream) -> Table
    to_csv

    get_at(x, y) -> String
    get_row(x) -> Vec<String>
    get_col(x) -> Vec<String>

    new_row(x)
    new_col(y)

    update(x, y, val)

    clear(x, y)

    // Maybe these get cached? yes
    dims -> (i32, i32)
    col_width(i) -> i32
    col_widths -> Vec<i32>


// Location in the table
// could be tuple struct
struct Cursor
    pub x: i32  // includes header, contrary to table struct
    pub y: i32


// abstraction from keyboard events
enum Command
    MoveUp ...
    Insert
    Escape
    CommandLine (colon)


// functions for reading user input
mod input
    read -> Option<Command>
    command                 // read from command line (more like normal mode)


// Everything involving visuals and terminal stuff
// Only struct for drop functionality, no state needed here
struct Renderer
    setup                       // raw mode, enter alternate screen
    draw_table(Table, Cursor, x, y)     // draw table at x, y column as top
    draw_command_line           // > at bottom
    draw_status
    drop                        // exit raw mode, exit alternative screen
