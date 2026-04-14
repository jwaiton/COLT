from colt import EventReader
import sys, os, time

COLT_DIR = str(os.environ['COLT_DIR'])
sys.path.append(f'{COLT_DIR}/python/scripts')
import funcs.WD_funcs as WD

def rust_func(file):
    for evt in EventReader(file, "WD1"):
        # check how quickly it can run through them all
        pass

def py_func(file):
    with open(file, 'rb') as file:
        x = WD.process_event_lazy_WD1(file, 2)
        # output x repeteadly until empty
        for output in x:
            pass


# benchmark warm up to deal with caching

file = f'{COLT_DIR}/python/scripts/wave1.dat'


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
