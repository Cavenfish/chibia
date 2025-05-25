# Chibia

This is a Rust CLI for keeping track of Tibia hunting data. This project mostly serves as a way for me to practice making CLI apps in Rust. It is partly inspired by hunt-analyser.

**Why the name Chibia?**

This is the English spelling of how Brazilian's pronounce Tibia. Since they are the largest player base in Tibia it felt fitting to name the CLI Chibia.

### Roadmap to First Release

- [x] Parse session times and lengths
- [ ] Improve UX for adding hunt logs
- [x] Add import/export for hunts
- [ ] Finish readme documentation
- [ ] Add more information to --helps
- [ ] Add mob_kill/h and looted_item/h stats for hunts

### Installation

Building from source with cargo.

```bash
git clone https://github.com/Cavenfish/chibia.git
cd chibia
cargo build
```

### Usage

```bash
chibia --help
Usage: chibia <COMMAND>

Commands:
  chars  Create, update, delete or list characters
  hunts  Add, remove, list, or query hunt logs
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```bash
chibia chars --help
Create, update, delete or list characters

Usage: chibia chars <COMMAND>

Commands:
  add       Create a new character
  level-up  Update level of a character
  skill-up  Update skill of a character
  delete    Delete a character
  import    Import characters
  export    Export characters
  show      List all characters
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```bash
chibia hunts --help
Add, remove, list, or query hunt logs

Usage: chibia hunts <COMMAND>

Commands:
  add     Add new hunt logs
  delete  Delete a hunt log
  update  Update a hunt log
  top     Get top hunting spots
  show    List all hunt logs
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Legal statement

Tibia is a registered trademark of CipSoft GmbH. Tibia and all products related to Tibia are copyright by CipSoft GmbH.
