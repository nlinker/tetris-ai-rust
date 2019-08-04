# RL for Tetris powered by Rust

RL stands for Reinforcement Learning.

## Installation and run

```bash
conda create -n py37t python=3.7
conda activate py37t
conda install -c pytorch pytorch

# this is needed for the cuda support in tch-rs
export TORCH_CUDA_VERSION=cu90
export LIBTORCH=~/anaconda3/envs/py37t/lib/python3.7/site-packages/torch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:${LD_LIBRARY_PATH}

cargo clean # force `tch-rs` to be compiled with the CUDA support
cargo run -- -t
```

## Verify torch and cuda

```
❯❯❯ ldd target/debug/tetris-app
    linux-vdso.so.1 (0x00007fff071b8000)
    libstdc++.so.6 => /usr/lib/x86_64-linux-gnu/libstdc++.so.6 (0x00007fdcfee9b000)
    libc10.so => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/libc10.so (0x00007fdcfec57000)
    libcaffe2.so => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/libcaffe2.so (0x00007fdcfba21000)
    libtorch.so.1 => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/libtorch.so.1 (0x00007fdcfa736000)
    libdl.so.2 => /lib/x86_64-linux-gnu/libdl.so.2 (0x00007fdcfa532000)
    librt.so.1 => /lib/x86_64-linux-gnu/librt.so.1 (0x00007fdcfa32a000)
    libpthread.so.0 => /lib/x86_64-linux-gnu/libpthread.so.0 (0x00007fdcfa10b000)
    libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fdcf9ef3000)
    libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fdcf9b02000)
    /lib64/ld-linux-x86-64.so.2 (0x00007fdcff8b7000)
    libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fdcf9764000)
    libgomp.so.1 => /usr/lib/x86_64-linux-gnu/libgomp.so.1 (0x00007fdcf9535000)
    libmkl_intel_lp64.so => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/../../../../libmkl_intel_lp64.so (0x00007fdcf89bd000)
    libmkl_gnu_thread.so => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/../../../../libmkl_gnu_thread.so (0x00007fdcf716a000)
    libmkl_core.so => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/../../../../libmkl_core.so (0x00007fdcf2e95000)
    libcudart.so.10.0 => /usr/local/cuda/lib64/libcudart.so.10.0 (0x00007fdcf2c1b000)
    libnvToolsExt.so.1 => /usr/local/cuda/lib64/libnvToolsExt.so.1 (0x00007fdcf2a12000)
    libcaffe2_gpu.so => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/libcaffe2_gpu.so (0x00007fdcc6f85000)
    libc10_cuda.so => /home/nick/anaconda3/envs/py37t/lib/python3.7/site-packages/torch/lib/libc10_cuda.so (0x00007fdcc6d61000)
    libcusparse.so.10.0 => /usr/local/cuda/lib64/libcusparse.so.10.0 (0x00007fdcc32f9000)
    libcurand.so.10.0 => /usr/local/cuda/lib64/libcurand.so.10.0 (0x00007fdcbf192000)
    libcufft.so.10.0 => /usr/local/cuda/lib64/libcufft.so.10.0 (0x00007fdcb8cde000)
    libcublas.so.10.0 => /usr/local/cuda/lib64/libcublas.so.10.0 (0x00007fdcb4748000)
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

