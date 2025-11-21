# mlog

A command for logging the music I listen to.

It's sort of meant to be used with the Nushell wrapper, `mlog`, but can be used with with the normal CLI, `musiklog`.
The difference is you will lose the pretty printing and argument completition so I don't recommend it.

Data is saved to a SQLite database (here bundled with the CLI).

Not yet battle tested.

## Installation

Install the CLI!
```nushell
cargo install --path .
```

Source the script somehow!
```nushell
source mlog.nu  
```

## Usage

Nushell will help you with the how-to with `help mlog`, or look at the exported commands in `mlog.nu`.
But otherwise you can basically do this:

- List artists
- Add artist
- List releases
- Add release
- Log a listen (of a release)
- List logs
