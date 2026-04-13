import numpy as np
from colt import EventReader

for evt in EventReader('/home/e78368jw/Documents/COLT/scripts/wave1.dat', "WD1"):
    print(evt)
