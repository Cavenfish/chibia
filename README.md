# Chibia

This is a Rust CLI for keeping track of Tibia hunting data. This project mostly serves as a way for me to practice making CLI apps in Rust. It is partly inspired by hunt-analyser.

**Why the name Chibia?**

This is the English spelling of how Brazilian's pronounce Tibia. Since they are the largest player base in Tibia it felt fitting to name the CLI Chibia.

### Roadmap to First Release

- [x] Parse session times and lengths
- [x] Improve UX for adding hunt logs
- [x] Add import/export for hunts
- [ ] Finish readme documentation
- [ ] Add more information to --helps

### Features to be added
- [ ] Add mob_kill/h and looted_item/h stats for hunts

### Installation

Building from source with cargo.

```bash
git clone https://github.com/Cavenfish/chibia.git
cd chibia
cargo build
```

### Basic Usage Example

You can preview all stored hunt logs with a simple show command.

```bash
chibia hunts show
ID    Character       Balance    Raw XP/h   Total XP  
-------------------------------------------------------
1     Homem Um Soco   11.6k      57.1k      65.7k     
2     Homem Um Soco   19.6k      53.8k      25.3k     
3     Cavenfish       12.2k      322.8k     278.3k    
4     Cavenfish       60.7k      353.3k     1.0kk     
5     Homem Um Soco   15.9k      67.1k      50.9k     
6     Binky Boy       -28.2k     144.8k     115.4k    
7     Cavenfish       16.0k      394.2k     274.5k    
8     Homem Um Soco   23.4k      72.8k      68.5k     
9     Homem Um Soco   41.2k      75.7k      38.6k     
10    Homem Um Soco   10.7k      60.9k      21.8k     
11    Binky Boy       -13.6k     119.8k     65.5k     
12    Binky Boy       -17.0k     142.6k     114.3k    
13    Cavenfish       79.5k      114.9k     99.6k     
14    Cavenfish       36.9k      368.1k     604.8k    
15    Cavenfish       7.6k       326.1k     265.9k    
16    Cavenfish       78.5k      274.9k     323.1k    
17    Cavenfish       34.5k      357.3k     434.2k    
18    Homem Um Soco   5.5k       52.4k      29.2k     
19    Cavenfish       30.7k      378.8k     387.4k    
20    Homem Um Soco   12.6k      55.0k      25.5k
```

You can also query the database for the top xp or loot hunts of specific characters. To sort by raw xp/h you use,

```bash
chibia hunts top --name Cavenfish --xp
ID    Character       Balance    Raw XP/h   Total XP  
-------------------------------------------------------
7     Cavenfish       16.0k      394.2k     274.5k    
19    Cavenfish       30.7k      378.8k     387.4k    
14    Cavenfish       36.9k      368.1k     604.8k    
17    Cavenfish       34.5k      357.3k     434.2k    
4     Cavenfish       60.7k      353.3k     1.0kk 
```

and by loot you can do the following.

```bash
chibia hunts top --name Cavenfish --loot
ID    Character       Balance    Raw XP/h   Total XP  
-------------------------------------------------------
13    Cavenfish       79.5k      114.9k     99.6k     
16    Cavenfish       78.5k      274.9k     323.1k    
4     Cavenfish       60.7k      353.3k     1.0kk     
14    Cavenfish       36.9k      368.1k     604.8k    
17    Cavenfish       34.5k      357.3k     434.2k 
```

### Help Pages

```bash
chibia --help
A CLI for keeping track of Tibia hunting data

Usage: chibia <COMMAND>

Commands:
  chars  Create, update, delete or list characters
  hunts  Add, remove, list, or query hunt logs
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
chibia chars --help
Manage character data

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
Manage hunt data

Usage: chibia hunts <COMMAND>

Commands:
  add     Add new hunt logs
  delete  Delete a hunt log
  update  Update a hunt log
  top     Get top hunting spots
  show    List all hunt logs
  export  Export hunt log
  import  Import hunt log
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Legal statement

Tibia is a registered trademark of CipSoft GmbH. Tibia and all products related to Tibia are copyright by CipSoft GmbH.
