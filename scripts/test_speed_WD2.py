import sys
import os
import numpy as np
import time
import warnings
from typing import Generator
from typing import BinaryIO
from typing import Optional
from colt import EventReader

# THIS SHOULD BE MOVED ELSEWHERE

def generate_wfdtype(channels, samples):
    '''
    generates the dtype for collecting the binary data based on samples and number of
    channels
    '''
    if channels >1:
        wdtype = np.dtype([
                ('event_number', np.uint32),
                ('timestamp', np.uint64),
                ('samples', np.uint32),
                ('sampling_period', np.uint64),
                ('channels', np.int32),
                ] +
                [(f'chan_{i+1}', np.float32, (samples,)) for i in range(0,channels)]
        )
    else:
        wdtype = np.dtype([
            ('event_number', np.uint32),
            ('timestamp', np.uint64),
            ('samples', np.uint32),
            ('sampling_period', np.uint64),
            ('chan_1', np.float32, (samples,))
        ])

    return wdtype


def read_defaults_WD2(file        :  BinaryIO,
                      byte_order  :  str) -> (int, int, int, int):
    '''
    Provided with an open WD2 binary file, will provide the header information.

    Parameters
    ----------

        file        (BufferedReader)  :  Opened file
        byte_order  (str)             :  Byte order

    Returns
    -------

        event_number     (int)  :  First event number extracted from file
        timestamp        (int)  :  Timestamp of first event
        samples          (int)  :  Number of samples
        sampling_period  (int)  :  The time value of 1 sample in ns
    '''

    event_number    = int.from_bytes(file.read(4), byteorder=byte_order)
    timestamp       = int.from_bytes(file.read(8), byteorder=byte_order)
    samples         = int.from_bytes(file.read(4), byteorder=byte_order)
    sampling_period = int.from_bytes(file.read(8), byteorder=byte_order)

    return (event_number, timestamp, samples, sampling_period)

def process_header(file_path  :  str,
                   byte_order :  Optional[str] = None) -> (np.dtype, int, int, int):
    '''
    Collect the relevant information from the file's header, and determine if its valid

    Header is formatted for WD2 as shown:
        Event number    -> uint32 (4 bytes)
        Timestamp       -> uint64 (8 bytes)
        Samples         -> uint32 (4 bytes)
        Sampling Period -> uint64 (8 bytes)
        (OPTIONAL)
        Channels        -> int32 (8 bytes)

    Waveform data is 4-byte float (float32).

    This extra optional channel poses problems, so need to consider it.
    The rest are all as expected.

    The `byte_order` should generally be left alone, but I have left it as an optional argument
    as there may be situations in which the data is recorded as little-endian and the computer you're
    processing it on is big-endian.

    Parameters
    ----------

        file_path  (str)  :  Path to binary file
        byte_order (str)  :  Byte order

    Returns
    -------

        wdtype           (ndtype)  :  Custom data type for extracting information from
                                      binary files
        samples          (int)     :  Number of samples per event
        sampling_period  (int)     :  The time value of 1 sample in ns
        channels         (int)     :  Number of channels in the data
    '''

    # ensure you're using the right byteorder defined by your machine.
    # If you take the data from one machine to another of differing endianness,
    # you may have issues here!
    if byte_order == None:
        warnings.warn("Warning: No byte order provided. This may cause issues if transferring data between machines.")
        byte_order = sys.byteorder
    elif (byte_order != 'little') and (byte_order != 'big'):
        raise NameError(f'Invalid byte order provided: {byte_order}. Please provide the correct byte order for your machine.')

    # open file
    if not os.path.exists(file_path):
        raise FileNotFoundError(2, 'Path or file not found', file_path)

    with open(file_path, 'rb') as file:

        event_number, timestamp, samples, sampling_period = read_defaults_WD2(file, byte_order)
        # attempt to read channels
        channels        = int.from_bytes(file.read(4), byteorder=byte_order)

        # then read in a full collection of data, and see if the following header makes sense.
        # if it explicitly breaks, assume 1 channel, raise a warning and continue.
        try:
            dataset         = file.read(4*samples*channels)
            event_number_1, timestamp_1, samples_1, sampling_period_1 = read_defaults_WD2(file, byte_order)
        except MemoryError as e:
            warnings.warn("process_header() unable to read file, defaulting to 1-channel description.\nIf this is not what you expect, please ensure your data was collected correctly.")
            event_number_1 = -1
            samples_1 = -1
            sampling_period_1 = -1

        # check that event header is as expected
        if (event_number_1 -1 == event_number) and (samples_1 == samples) and sampling_period_1 == (sampling_period):
            print(f"{channels} channels detected. Processing accordingly...")
        else:
            print(f"Single channel detected. If you're expecting more channels, something has gone wrong.\nProcessing accordingly...")
            channels = 1

    # this is a check to ensure that if you've screwed up the acquisition, it warns you adequately
    if samples == 0:
        raise RuntimeError(r"Unable to decode raw waveforms that have sample size zero. In wavedump 2, when collecting data from a single channel make sure that 'multiple channels per file' isn't checked.")

    # collect data types
    wdtype = generate_wfdtype(channels, samples)
    return wdtype, samples, sampling_period, channels


def read_binary_lazy(file    :  BinaryIO,
                     wdtype  :  np.dtype) -> Generator:
    '''
    Reads the binary in with the expected format/offset, lazily,
    depending on counts to break the data up.

    NOTE:
    The counts are hardset to 1, making this function relatively inefficient.
    In the future, the logic should be revised to allow `np.fromfile`'s count
    value to be set based on optimal read-in speed. The logic of the WD2 function
    will have to accomodate this when indexing the files.

    Parameters
    ----------

        file    (BufferedReader)  :  Opened file
        wdtype  (ndtype)         :  Custom data type for extracting information from
                                     binary files
        counts  (int)             :  How many events you want to read in. -1 sets it to take all events.
        offset  (int)             :  Offset at which to start reading the data. Used for chunking purposes
                                     and so should by default be set to zero if not chunking.

    Returns
    -------
        data  (ndarray)  :  Unformatted data from binary file

    '''
    # initialise data to start the loop
    data = (np.fromfile(file, dtype=wdtype, count = 1))
    while len(data) != 0:
        yield (True, data)
        # ensure data is loaded in after the yield, so the while check is done
        data = (np.fromfile(file, dtype=wdtype, count = 1))
    # yield 1 when finished
    print('Processing Finished!')
    yield (False, np.zeros(shape = (1,)))


def rust_func(file):
    for evt in EventReader(file, "WD2"):
        # check how quickly it can run through them all
        pass

def py_func(file):
    with open(file, 'rb') as f:

        # collect header info
        wdtype, samples, sampling_period, channels = process_header(file)

        x = read_binary_lazy(f, wdtype)
        # output x repeteadly until empty
        for output in x:
            pass


# benchmark warm up to deal with caching

file = '/home/e78368jw/Documents/COLT/scripts/three_channels_WD2.bin'


rust_func(file)
py_func(file)


t0 = time.perf_counter()
rust_func(file)
t1 = time.perf_counter()

t2 = time.perf_counter()
py_func(file)
t3 = time.perf_counter()

print(f"Rust:   {t1 - t0:.6f}s")
print(f"Python: {t3 - t2:.6f}s")
print(f"Speedup: {(t3 - t2)/(t1 - t0):.6f}x")
