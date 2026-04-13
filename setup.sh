#!/bin/bash

echo "COLT - CAEN Output Loader & Translator"


# initialise virtual environment
python -m venv .venv
source .venv/bin/activate

pip install maturin
pip install numpy

maturin develop --release

echo "Initialisation complete, please run:"
echo "source .venv/bin/activate"
