# gol-rs: Conway's game of life simulated in rust

A very quick and potentially very badly written rust project. I wanted to get familiar with the language so I tried my signature language testing project, Conway's game of life. Here are the results!

## Building and running

Clone the repository, `cd` into it, then run: 

```bash
cargo build --release && cargo run --release
```

This has only been tested on linux: the ncurses library that was used ([this  one](https://github.com/ihalila/pancurses)) is supposedly cross platform but this hasn't been tested.

## To Do

I still want to add a couple of things, so i can't say it's done just yet. Namely: 

 - [ ] State management: add a `State` enumeration to allow for setup/pausing/a lot of fun stuff. 
 - [ ] Better printing and refreshing: works fine in my hyper terminal window but the default Chrome OS terminal has very noticeable and hideous "scanlines" so I'll see what I can do about that
 - [ ] If anyone has suggestions/issues, report them and I'll be glad to help!

