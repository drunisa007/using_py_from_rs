import numpy as np
import pandas as pd


def bar():
    npversion = np.__version__
    pdversion = pd.__version__
     
    return npversion+" "+ pdversion

