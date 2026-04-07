# COLT - CAEN Acquisition Loader & Translator

Python package for extracting WaveDump events into iterators from binary files.

Currently only supports WaveDump 1 .dat files.

Use:

- run `source setup.sh`, this will create a local version of the python package in the virtual environment
- create a python script, like so:

```python
import numpy as np
from colt import EventReader

for evt in EventReader('/home/e78368jw/Documents/COLT/scripts/wave1.dat'):
    print(evt)
```

This will output:
```
...
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
...
```

which includes all relevant information regarding the data within the waveform.
