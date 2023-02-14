# Méliès

A Rust built, GUI focused, remake or Ryukbot v2.

## Goals

*Ryukbot v2* was designed to make recording Team Fortress 2 footage as automated as possible while still allowing some freedom in how that automation worked, however, it's entirely command line interface made it confusing or even hard to use for some people. *Méliès* looks to change that by creating a faster and easier to understand system. With [Rust](https://www.rust-lang.org/) and [Tauri](https://tauri.app/) doing all of the heavy lifting, we won't need to rely on pythons relatively slow speeds to process everything. The GUI built with [Svelte-Kit](https://kit.svelte.dev/) also has the goal of making the program as easy to understand and customizable as possible, allowing users to see and edit their settings without needing a text editor to edit a JSON file.

### A Trip to the Moon

[Georges Méliès](https://en.wikipedia.org/wiki/Georges_M%C3%A9li%C3%A8s) was a French illusionist and film that took audiences from the Earth to the Moon back in 1902. I'm hoping this little program helps people unleash their own imaginations, just like Georges Méliès did all those years ago.

## To-Do

Currently, pretty much everything is to-do. I'll keep this page updated with the completed tasks as they are done and new tasks as they arrive.

### Functionality

- [x] Set up project
- [x] Open settings.json
- [ ] Find tf folder
- [ ] Open events_.txt file
- [ ] Read lines in file
- [ ] Parse events from each line

### Style

- [x] Decide default color scheme
- [ ] Decide Font
- [ ] Build basic reusable component set

### Customization

- [ ] Themes/Color choices
- [ ] Easily editable settings (within the app)

### Mods

- [ ] Open custom mod files
- [ ] Decide on syntax (can keep old syntax from v2)

### Events editor

- [ ] Open events file
- [ ] Add new events to the end
- [ ] View current events
- [ ] Edit already existing events
- [ ] Save the txt file

### VDM editor

- [ ] Open VDM file
- [ ] Parse VDM file

### Logs.tf integration

- [ ] Open log using logs.tf API

## License

This project uses the [MIT License](https://mit-license.org/).