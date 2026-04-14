import numpy as np
import os
from colt import EventReader

COLT_DIR = str(os.environ['COLT_DIR'])

for evt in EventReader(f'{COLT_DIR}/python/scripts/three_channels_WD2.bin', "WD2"):
    print(f"event_counter: {evt['event_counter']}, timestamp: {evt['timestamp']}, samples: {evt['samples']}, sampling_period: {evt['sampling_period']}, channels: {evt['channels']}" )
    evt_data = np.array_split(evt['data'], evt['channels'])
    for i in range(0,evt['channels']):
        print(f'ch{i} data:')
        print(evt_data[i])



