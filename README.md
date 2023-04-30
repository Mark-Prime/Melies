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
- [x] Find tf folder
- [x] Open _events.txt file
- [x] Read lines in file
- [x] Parse events from each line
- [x] Check multiple file locations/names
- [x] Create event struct
- [x] Read ds and prec event file types
- [x] Create clip struct
- [x] Combine close events into a single clip
- [x] Name clips appropriately
- [x] Make VDM crate (whole different thing I'm doing)
- [x] Add auto-suffix option to settings
- [x] Make auto-suffix setting functional 
- [x] add FOV and view model FOV settings
- [x] Create default settings if first time use
- [x] Hide console on build
- [x] Display events.txt or killstreaks.txt file on home page
- [x] Clear events file after running (if that option is selected)
- [ ] Create backups
- [ ] Create logs
- [x] Beautify events display
- [x] Add edit button
- [x] Allow editing of events
- [x] Save the events after edited
- [x] Allow clip_start and clip_end to work
- [x] enabled native spec_modes

### Style

- [x] Decide default color scheme
- [ ] Build basic reusable component set
- [ ] Create logo/icon
- [x] Decide Font

### Customization

- [x] Open and display settings file
- [x] Easily editable settings (within the app)
- [ ] Themes/Color choices

### Mods

- [ ] Open custom mod files
- [ ] Decide on syntax (can keep old syntax from v2)
- [ ] Create mod struct

### Events editor

- [x] Open events file
- [x] Add new events to the end
- [x] View current events
- [x] Edit already existing events
- [x] Save the txt file
- [x] Edit demo name
- [x] Add new demos

### VDM editor

- [ ] Open VDM file
- [ ] Parse VDM file

### Logs.tf integration

- [x] Open log using logs.tf API
- [x] Parse data from the JSON and display it
- [x] Allow event selection for recording
- [ ] POV selection
- [x] Turn selected events into 
- [ ] make it stop crashing if the loading takes too long

### Demo scanning

- [x] Locate demo files in the tf folder
- [x] Parse a list of kills in the demo
- [x] Display list to user
- [x] Allow user to select what to record

## License

This project uses the [MIT License](https://mit-license.org/).