# `cic` Change Interactive CSV

A command line utility for viewing and editing csvs.  Work in progress.

Goal is to have somethat that appears like excel, taking up the entire terminal window and uses vim-bindings to navigate to different cells to edit them.  This will NOT have any "excel" computational functionality, it will only store/edit static data (that is, csvs).

## Keybinds

* `hjkl` for movement of the cursor by one cell
* `g` go to top of column
* `G` go to bottom of column
* `0` go to first cell of row
* `$` go to last cell of row
* `a` to append value of current cell
* `o` to add new row after current row
* `O` to add new row before current row
* `D` to delete row
* `:` to enter prompt

## Commands

* `:addcol` to add column to the right
* `:delcol` to delete column
* `:w` to save
* `:q` to quit

# Ideas and next steps

* "insert" mode where I can use tab and enter to walk through a list of cells, like you would in excel
* suggest commands when in prompt
* add `c` to overwrite value in cell
* prompt user to save changes when exiting without saving
* allow `:wq`
* add help menu
* redraw on terminal size change
* graphics options to draw lines between columns
* search
