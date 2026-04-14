import numpy as np
import os
from colt import EventReader

COLT_DIR = str(os.environ['COLT_DIR'])

for evt in EventReader(f'{COLT_DIR}/python/scripts/wave1.dat', "WD1"):
    print(evt)
