#!/bin/bash

echo "COLT - CAEN Output Loader & Translator"

# set directory path to variable
export COLT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"


# initialise virtual environment
python -m venv .venv
source .venv/bin/activate

pip install maturin
pip install numpy
pip install pytest

maturin develop --release

echo "Initialisation complete, please run:"
echo "source .venv/bin/activate"
