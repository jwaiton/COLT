import sys, os, time
import numpy as np
import time
from colt import EventReader


COLT_DIR = str(os.environ['COLT_DIR'])
sys.path.append(f'{COLT_DIR}/python/scripts')
import funcs.WD_funcs as WD


# THIS SHOULD BE MOVED ELSEWHERE

def rust_func(file):
    for evt in EventReader(file, "WD2"):
        # check how quickly it can run through them all
        pass

def py_func(file):
    with open(file, 'rb') as f:

        # collect header info
        wdtype, samples, sampling_period, channels = WD.process_header(file)

        x = WD.read_binary_lazy(f, wdtype)
        # output x repeteadly until empty
        for output in x:
            pass


# benchmark warm up to deal with caching

file = f'{COLT_DIR}/python/scripts/three_channels_WD2.bin'


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
