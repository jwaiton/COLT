import pytest
import numpy as np
from colt import EventReader
import sys, os, time

COLT_DIR = str(os.environ['COLT_DIR'])
sys.path.append(f'{COLT_DIR}/python/scripts')
import funcs.WD_funcs as WD

def test_WD1reader():
    '''
    check the reader matches the expected shape, with the relevant data
    '''
    file = f'{COLT_DIR}/python/scripts/wave1.dat'

    colt_data = []
    for evt in EventReader(file, "WD1"):
        colt_data.append(evt)

    with open(file, 'rb') as f:
        x = WD.process_event_lazy_WD1(f, 2)

        for i, output in enumerate(x):
            # ensure data is the same in both
            assert np.array_equal(np.array(output[0]), np.array(colt_data[i]['data']))


def test_WD2reader():
    '''
    check the reader matches the expected shape, with the relevant data
    '''
    file = f'{COLT_DIR}/python/scripts/three_channels_WD2.bin'

    colt_data = []
    for evt in EventReader(file, "WD2"):
        colt_data.append(evt)

    with open(file, 'rb') as f:

        # collect header info
        wdtype, samples, sampling_period, channels = WD.process_header(file)

        x = WD.read_binary_lazy(f, wdtype)
        # output x repeteadly until empty

        for i, output in enumerate(x):
            # avoid end of file component
            if repr(output[1][0]) != 'np.float64(0.0)':
                assert np.array_equal(np.concatenate([output[1][0][5], output[1][0][6], output[1][0][7]]), np.array(colt_data[i]['data']))
            # ensure data is the same in both
