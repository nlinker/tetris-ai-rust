# RL for Tetris powered by Rust

RL stands for Reinforcement Learning.

## Installation and run

## Useful stuff

```bash
cargo install cargo-tree
cargo tree  # to see the tree
 
# for ncurses dependency
sudo apt install libncurses5-dev
```

https://tetris.fandom.com/wiki/Tetris_Guideline

https://tetris.fandom.com/wiki/SRS, especially
[Wall kicks](https://tetris.fandom.com/wiki/SRS?section=3)

Left
```
^[[D
You pressed byte 27
You pressed byte 91
You pressed byte 68
```

Right
```
^[[C
You pressed byte 27
You pressed byte 91
You pressed byte 67
```

Up
```
^[[A
You pressed byte 27
You pressed byte 91
You pressed byte 65
```

Down
```
^[[B
You pressed byte 27
You pressed byte 91
You pressed byte 66
```