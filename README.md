# Chibia

This is a Rust CLI for keeping track of Tibia hunting data. This project mostly serves as a way for me to practice making CLI apps in Rust. It is partly inspired by hunt-analyser.

**Why the name Chibia?**

This is the English spelling of how Brazilian's pronounce Tibia. Since they are the largest player base in Tibia it felt fitting to name the CLI Chibia.

### Features to be added
- [ ] Add mob_kill/h and looted_item/h stats for hunts

### Installation

Building from source with cargo.

```bash
git clone https://github.com/Cavenfish/chibia.git
cd chibia
cargo build --release
cp ./target/release/chibia ~/.local/bin
```

### Basic Usage Example

To add hunting logs to the database first export the hunting session from Tibia as a json file. Then you can add the hunt log to the database using `chibia hunts add`. 

```bash
chibia hunts add --help
Add new hunt logs

Usage: chibia hunts add [OPTIONS]

Options:
      --id <ID>                ID of character used on hunt [default: 0]
      --spawn <SPAWN>          Location of hunt [default: Unknown]
      --loot-mult <LOOT_MULT>  Loot multiplier during hunt [default: 0]
      --no-skip                Disable the skip prompt
  -h, --help                   Print help
```

If you don't pass id, spawn or loot-mult you will be prompted for it after a preview of the hunt log is shown. This is useful when you have several hunt logs that need to be added but they all are for different characters or spawns. If you pass any of those three parameters it will apply to all hunt logs (unless you agree to skip a log) to be added.

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

You can also restrict to a specific spawn location.

```bash
chibia hunts top --name "Homem Um Soco" --spawn "Upper Spike -3" --xp
ID    Character       Balance    Raw XP/h   Total XP  
-------------------------------------------------------
10    Homem Um Soco   10.7k      60.9k      21.8k     
20    Homem Um Soco   12.6k      55.0k      25.5k     
18    Homem Um Soco   5.5k       52.4k      29.2k 
```

You can also view the full hunt log data with the show command. The character info at the time of adding the hunt is saved so you know what the stats of your character was at the time of the hunt.

```bash
chibia hunts show --id 18

Homem Um Soco (Monk):
  Level:		35
  Magic Level:		18
  Fist Fighting:	80
  Shielding Level:	55

Hunt Info:
   ID:			18
   Start Date:		Unknown
   End Date:		Unknown
   Duration:		00:09h
   Spawn:		Upper Spike -3
   Loot Multiplier:	1
   Loot:		8.1k
   Supplies:		2.6k
   Balance:		5.5k
   Raw XP:		8.5k (52.4k/h)
   XP:			29.2k (181.1k/h)
   Damage:		13.8k (13.8k/h)
   Healing:		3.7k (3.7k/h)
Looted Items:
   -- 1 a small stone
   -- 1 a wolf tooth chain
   -- 3 a silver brooch
   -- 493 a gold coin
   -- 2 a strange talisman
   -- 2 a throwing star
   -- 3 cheese
   -- 1 a shadow herb
   -- 1 a yellow piece of cloth
   -- 1 a potato
   -- 4 a gauze bandage
   -- 2 a stone wing
   -- 2 a flask of embalming fluid
   -- 1 a cheese cutter
   -- 2 an earflap
   -- 2 soft cheese
   -- 1 rat cheese
Monsters Killed:
   -- 8 corym charlatan
   -- 6 corym skirmisher
   -- 1 crypt shambler
   -- 2 demon skeleton
   -- 10 gargoyle
   -- 5 ghost
   -- 20 mummy
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
