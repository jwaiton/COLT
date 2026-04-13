import numpy as np
from colt import EventReader

for evt in EventReader('/home/e78368jw/Documents/COLT/scripts/three_channels_WD2.bin', "WD2"):
    print(f"event_counter: {evt['event_counter']}, timestamp: {evt['timestamp']}, samples: {evt['samples']}, sampling_period: {evt['sampling_period']}, channels: {evt['channels']}" )
    evt_data = np.array_split(evt['data'], evt['channels'])
    for i in range(0,evt['channels']):
        print(f'ch{i} data:')
        print(evt_data[i])



