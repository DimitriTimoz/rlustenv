import math
import numpy as np
import pandas as pd


def discretize_actions(action):
    global min_action, max_action, num_split
    return pd.Series(np.array((action/(max_action - min_action)) * num_split, dtype=np.int32))
def continue_actions(action):
    global min_action, max_action, num_split
    return pd.Series(np.array(((action/num_split) * (max_action - min_action)) + min_action, dtype=np.float32))
min_action = np.array([0.0, 0.0, -math.pi/3., -math.pi/3.])
max_action = np.array([1.0, 1.0, math.pi/3., math.pi/3.])
num_split = np.array([20, 20, 20, 20], dtype=np.int32)

print(discretize_actions(np.array([0.5, 0.5, math.pi/6., math.pi/6.])))
print(continue_actions(np.array([10, 10, 5, 5])))