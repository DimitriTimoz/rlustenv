from rlustenv_api import *
import math
import random
import numpy as np
import tensorflow as tf
from collections import deque
from tf_agents.agents.dqn import dqn_agent
import tf_agents
import time

num_actions = 2 + 2 
num_input = 2 + 2 + 1 + 1
learning_rate = 1e-3
gamma = 0.99
epsilon = 0.4
decision_rate = 4.

def model_build():
    return tf.keras.Sequential([
        tf.keras.layers.Input(shape=(num_input), dtype='float32'),
        tf.keras.layers.Dense(16, activation='relu'),
        tf.keras.layers.Dense(32, activation='relu'),
        tf.keras.layers.Dense(32, activation='relu'),
        tf.keras.layers.Dense(num_actions * 10, activation='linear'),
        tf.keras.layers.Reshape((num_actions, 10)),
    ])

   

model = model_build()
model.compile(optimizer=tf.keras.optimizers.Adam(learning_rate=learning_rate), loss=tf.keras.losses.MeanSquaredError())
#model.load_weights("model.h5")

min_action = np.array([0.0, 0.0, 0.0, 0.0])
max_action = np.array([1.0, 1.0, 2.*math.pi, 2.*math.pi])
num_split = np.array([10, 10, 10, 10], dtype=np.int32)


def train_step(reward, action, observation, next_observation, done):
    observation = np.array([observation], dtype=np.float32)
    global gamma
    next_Q_values=model.predict([next_observation])[0]
    best_next_actions=tf.math.argmax(next_Q_values, axis=1)
    next_mask=tf.one_hot(best_next_actions, 10)
    next_best_Q_values=tf.reduce_sum(next_Q_values*next_mask, axis=1)
    target_Q_values=reward+(1-done)*gamma*next_best_Q_values
    target_Q_values = np.array([target_Q_values] * 10).T
    mask=tf.one_hot(action, 10)
    Q_values = (target_Q_values * mask)
    Q_values = np.array([Q_values], dtype=np.float32)
    model.fit(observation, Q_values, verbose=0)
    
    return model.predict([next_observation])
  
def compute_reward(controller: DroneController):
    return math.exp(-np.linalg.norm(controller.relative_position))

def get_state(controller: DroneController):
    return [*controller.velocity, controller.angular_velocity, *controller.relative_position, controller.angle]

def discretize_action(action):
    global min_action, max_action, num_split
    return np.array((action/(max_action - min_action)) * num_split, dtype=np.int32)

def undiscretize_action(action):
    global min_action, max_action, num_split
    return np.array(((action/num_split) * (max_action - min_action)) + min_action, dtype=np.float32)
    
prev_observation=None
prev_action = discretize_action([0., 0., 0., 0.])
prev_controller = None


def start(controller: DroneController):
    """create an instance of Gadget, configure it and return to Rust"""
    global prev_observation, prev_controller
    prev_controller = controller
    prev_observation = get_state(controller)
    print("precedent", prev_observation)
    return controller

def random_action():
    return np.array([random.random(), random.random(), random.random()*2.*math.pi, random.random()*2.*math.pi])

debug = True
start_time = 0
start_time=time.time()
last_time =time.time() - 1.

def loop(controller: DroneController):
    """loop function, called every frame"""
    global epsilon, last_time, prev_observation, model, prev_action, train_loss, optimizer
    if time.time() - last_time < 1./decision_rate:
        return controller
    reward = compute_reward(controller)
    actions_disc = train_step(reward, prev_action, prev_observation, get_state(controller), False)[0]
    # Apply action
    prev_action = np.argmax(actions_disc, axis=1)
    actions = undiscretize_action(prev_action)
    
    if random.random() < epsilon:
        actions = random_action()
    
    controller.thrust_left = actions[0]
    controller.thrust_right = actions[1]
    controller.thrust_left_angle = actions[2]
    controller.thrust_right_angle = actions[3]
    if debug:
        print("Entrainement {:5.3f} seconde(s)".format(float(time.time()-start_time)))
        #print("     loss: {:6.4f}".format(train_loss.result()))
    print("  here: ", actions)
    prev_observation = get_state(controller)
    
    last_time = time.time()
    return controller


def end(controller: DroneController, reason: str):
    """called when the simulation ends"""
    print("Simulation ended because of: " + reason)
    reward = 10 if reason == "reached_target" else -10
    actions_disc = train_step(-10., prev_action, prev_observation, get_state(controller), True)[0]

    model.save_weights("model.h5")

