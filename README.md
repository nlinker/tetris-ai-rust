# RL for Tetris powered by Rust

RL stands for Reinforcement Learning.

## Installation and run

```bash
conda create -n py37t python=3.7
conda activate py37t
conda install -c pytorch pytorch

export LIBTORCH=~/anaconda3/envs/py37t/lib/python3.7/site-packages/torch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:${LD_LIBRARY_PATH}

cargo run -- -t
```

## Useful stuff

### Super Rotation System
![srs](_img/srs-pieces.png)


### Install dependencies
```bash
cargo install cargo-tree
cargo tree  # to see the tree
 
# for ncurses dependency
sudo apt install libncurses5-dev
```

https://tetris.fandom.com/wiki/Tetris_Guideline

https://tetris.fandom.com/wiki/SRS, especially
[Wall kicks](https://tetris.fandom.com/wiki/SRS?section=3)


