# Basic Features

A program to index, search, and rank various items on the computer written in Rust, 2024 edition.

## Basics

- Runs on NixOS under GNOME or wlroots.
- Uses the `eframe` and `egui` libraries for GUI rendering.
- The window should respond to common interactions:
  - `Ctrl-w` and `Ctrl-q` should close the window and quit the app.

## Functionality

- The window should contain a main search field at the top and a scrollable list of matched items below the text box.

## Interaction

- Typing into the text box should update the matched items after a debounce of 100ms.
- When pressing the up and down arrow keys, the selected item should change respectively.
- Page Up and Page Down should move the selection to the top or bottom of the visible items. The second press should move the list of items up or down by the number of items currently being displayed.
- The home and end keys should move to the first last items in the list respectively.
- Pressing Return or Enter should open the selected item
- Clicking on an item should launch it as well.
- If shift is also held down, it should reveal the item in a platform appropriate file browser.
- When the text box is empty, a list of recent queries should be shown.
  - If the items are clicked or if Return/Enter is pressed when one is selected, it should fill the search field with that item.
