#!/usr/bin/env bash

export TORCH_CUDA_VERSION=cu90
export LIBTORCH=~/anaconda3/envs/py37t/lib/python3.7/site-packages/torch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:${LD_LIBRARY_PATH}

echo ${LD_LIBRARY_PATH}