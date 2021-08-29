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

types of modes:
* movement
    * move direction
    * add row
    * delete row
* prompt/command
    * add column
    * delete column (with prompt?)
* visual
    * move rows around
    * move columns around?
* insert

Maybe separate user input and actual commands?

terminal event + mode -> action
