import numpy as np
import time
from typing import BinaryIO
from colt import EventReader
# THIS SHOULD BE MOVED ELSEWHERE
class MalformedHeaderError(Exception):
    '''
    Exception created for when two headers don't match up consecutively.
    Created initially for WD1 processing, but should be back-ported for WD2
    '''

    def __init__(self, header1, header2):
        self.header1 = header1
        self.header2 = header2

    def __str__(self):
        return f"MalformedHeaderError: Headers don't output expected result. Ensure the .dat file provided is formatted correctly.\nFirst Header {self.header1}\nSecond Header {self.header2}"



def process_event_lazy_WD1(file_object  :  BinaryIO,
                           sample_size  :  int):

    '''
    WAVEDUMP 1: Generator that outputs each event iteratively from an opened binary file
    Parameters
    ----------
        file_object  (obj)  :  Opened file object
        sample_size  (int)  :  Time difference between each sample in waveform (2ns for V1730B digitiser)
    Returns
    -------
        data  (generator)  :  Generator object containing one event's worth of data
                              across each event
    '''

    # read first header
    header = np.fromfile(file_object, dtype = 'i', count = 6)

    # header to check against
    sanity_header = header.copy()

    # continue only if data exists
    while len(header) > 0:

        # alter header to match expected size
        header[0] = header[0] - 24
        event_size = header[0] // sample_size

        # collect waveform, no of samples and timestamp
        yield (np.fromfile(file_object, dtype = np.dtype('<H'), count = event_size), event_size, header[-1])

        # collect next header
        header = np.fromfile(file_object, dtype = 'i', count = 6)

        # check if header has correct number of elements and correct information ONCE.
        if sanity_header is not None:
            if len(header) == 6:
                if all([header[0] == sanity_header[0], # event size
                    header[4] == sanity_header[4] + 1,  # event number +1
                    header[5] > sanity_header[5]        # timestamp increases
                    ]):
                    sanity_header = None
                else:
                    raise MalformedHeaderError(sanity_header, header)
            else:
                raise MalformedHeaderError(sanity_header, header)
    print("Processing Finished!")


def rust_func(file):
    for evt in EventReader(file, "WD1"):
        # check how quickly it can run through them all
        pass

def py_func(file):
    with open(file, 'rb') as file:
        x = process_event_lazy_WD1(file, 2)
        # output x repeteadly until empty
        for output in x:
            pass


# benchmark warm up to deal with caching

file = '/home/e78368jw/Documents/COLT/scripts/wave1.dat'


rust_func(file)
py_func(file)


t0 = time.perf_counter()
rust_func(file)
t1 = time.perf_counter()

t2 = time.perf_counter()
py_func(file)
t3 = time.perf_counter()

print(f"Rust:   {t1 - t0:.3f}s")
print(f"Python: {t3 - t2:.3f}s")
print(f"Speedup: {(t3 - t2)/(t1 - t0):.2f}x")
