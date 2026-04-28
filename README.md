<a href="https://github.com/jwaiton/COLT">
    <img src="assets/COLT.png" alt="MULE" style="display: block; margin: 0;"/>
</a>

<div align="right" style="margin-top: 0;">
<h3 style="margin: 0;">CAEN Output Loader & Translator</h3>
    <sub>Modification of <a href="https://commons.wikimedia.org/wiki/File:Horse-racing-3.jpg">photograph</a> taken by Softeis, distributed under CC BY-SA 3.0 license.</sub>
</div>

<hr>

<div align="center">

  [![CI](https://github.com/jwaiton/COLT/actions/workflows/CI.yml/badge.svg)](https://github.com/jwaiton/COLT/actions/workflows/CI.yml)
  [![tests](https://github.com/jwaiton/COLT/actions/workflows/tests.yml/badge.svg)](https://github.com/jwaiton/COLT/actions/workflows/Tests.yml)
  ![GitHub Issues or Pull Requests](https://img.shields.io/github/issues/jwaiton/COLT)
  [![GitHub License](https://img.shields.io/github/license/nu-ZOO/MULE)](https://github.com/jwaiton/COLT/blob/main/LICENSE)
</div>

<p align="center">
  <a href="#what-is-colt">About</a> &#xa0; | &#xa0;
  <a href="#installation">Installation</a> &#xa0; | &#xa0;
  <a href="#quickstart">Quickstart</a> &#xa0; | &#xa0;
  <a href="#Contributing">Contributing</a> &#xa0; | &#xa0;
  <a href="#Citations">Citations</a> &#xa0; | &#xa0;
  <a href="#License">License</a> 
</p>

## What is COLT?
COLT is a simple python package for extracting WaveDump events into iterators from binary files, written in Rust.

COLT currently supports Wavedump 1 \& 2 files, and has been tested with [these digitisers](https://github.com/nu-ZOO/COLT/wiki/tested_digitisers)

## Installation

### Using pip

```
pip install colt
```

### From scratch
```bash
git clone https://github.com/nu-ZOO/COLT.git
cd COLT
source setup.sh
```
At this point, the wheel files should be generated locally:
```bash
cd target/wheels
pip install colt_version.whl
```
This will install COLT into the local virtual environment, which through `setup.sh` should be `.venv`.


## Quickstart
With COLT initialised, write a python script like so:
```python
import numpy as np
from colt import EventReader                                              #  Wavedump 1
                                                                          # \/
for evt in EventReader('/home/e78368jw/Documents/COLT/scripts/wave1.dat', "WD1"):
    print(evt)
```

This will output:
```python
more events...
{'event_size': 284, 'board_id': 0, 'pattern': 0, 'board_channel': 1, 'event_counter': 9862, 'timestamp': 1232799195, 'data': array([7703, 7708, 7693, 7706, 7709, 7699, 7707, 7699, 7708, 7707, 7699,
       7706, 7703, 7710, 7699, 7702, 7704, 7702, 7707, 7704, 7702, 7702,
       7708, 7701, 7703, 7708, 7701, 7699, 7704, 7703, 7704, 7706, 7707,
       7695, 7711, 7702, 7706, 7702, 7701, 7704, 7707, 7707, 7697, 7707,
       7703, 7700, 7705, 7704, 7697, 7711, 7704, 7702, 7701, 7714, 7697,
       7705, 7705, 7706, 7708, 7702, 7711, 7701, 7711, 7699, 7706, 7702,
       7700, 7702, 7710, 7703, 7697, 7704, 7712, 7698, 7707, 7699, 7705,
       7702, 7709, 7703, 7711, 7707, 7699, 7707, 7707, 7704, 7700, 7703,
       7710, 7704, 7704, 7708, 7703, 7709, 7696, 7707, 7713, 7703, 7702,
       7706, 7708, 7698, 7707, 7704, 7703, 7709, 7707, 7707, 7698, 7709,
       7709, 7700, 7706, 7708, 7704, 7702, 7711, 7702, 7705, 7705, 7703,
       7706, 7705, 7710, 7699, 7702, 7713, 7700, 7708, 7698], dtype=uint16)}
more events...
```

which includes all relevant information regarding the data within each waveform.

For WaveDump 2, each iterator produces an array containing the waveforms of all channels, which needs to be separated by the user. A template for
this can be seen in [`test_colt_WD2.py`](https://github.com/nu-ZOO/COLT/blob/main/python/scripts/test_colt_WD2.py). General templates of the usage are seen in [`python/scripts/`](https://github.com/nu-ZOO/COLT/tree/main/python/scripts)

To get a *wheel* file, there are currently two options:
- within the **artifacts** section of the most recently run `CI.yml` github workflow, found [here](https://github.com/jwaiton/COLT/actions/workflows/CI.yml).
- run `maturin build`, and a `.whl` file will be placed within `COLT/target/wheels`


## Contributing

Contributions follow the standard MULE guidelines as found [here](https://github.com/nu-ZOO/MULE/wiki/Contribution-Guidelines)

## Citations

Citations have yet to be generated, please use the MULE citation found [here](https://github.com/nu-ZOO/MULE/blob/main/CITATION.cff)

## License

COLT uses a GPL-3.0 license. The full license is provided in [LICENCE](https://github.com/nu-ZOO/COLT/blob/main/LICENSE).
