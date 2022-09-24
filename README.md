# `cic` Change Interactive CSV

A command line utility for viewing and editing csvs.  Work in progress.

Goal is to have somethat that appears like excel, taking up the entire terminal window and uses vim-bindings to navigate to different cells to edit them.  This will NOT have any "excel" computational functionality, it will only store/edit static data (that is, csvs).

## Keybinds (Table Mode)

* `hjkl` for movement of the cursor by one cell
* `g` go to top of column
* `G` go to bottom of column
* `0` go to first cell of row
* `$` go to last cell of row
* `a` to append value of current cell
* `c` to overwrite value of current cell
* `o` to add new row after current row
* `O` to add new row before current row
* `D` to delete row
* `:` to enter prompt
* `I` to go into insert mode (below)

## Insert Mode

In this mode, you'll be typing most of the type, with tab and enter as navigation, as you would in excel.

* `tab` moves cursor right
* `backtab` moves cursor left (shift+tab)
* `enter` moves cursor down and all the way to the left
* arrows keys let you move the cursor as expected
* `esc` goes back to table mode

## Commands

* `:addcol` to add column to the right
* `:delcol` to delete column
* `:w` to save
* `:q` to quit

# Ideas and next steps

* suggest commands when in prompt
* prompt user to save changes when exiting without saving
* allow `:wq`
* add help menu
* redraw on terminal size change
* graphics options to draw lines between columns
* search
