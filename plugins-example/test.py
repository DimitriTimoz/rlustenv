from rlustenv_api import *
import math
import random
import numpy as np
import tensorflow as tf
from collections import deque
from tf_agents.agents.dqn import dqn_agent
import matplotlib.pyplot as plt
import tf_agents
import time

num_actions = 2 + 2 
num_input = 2 + 2 + 1 + 1
learning_rate = 1e-3
gamma = 0.9
epsilon = 0.05
decision_rate = 5.
n_episodes = 0

plt.switch_backend('agg')

reward_history = deque(maxlen=1000)

def model_build():
    return tf.keras.Sequential([
        tf.keras.layers.Input(shape=(num_input), dtype='float32'),
        tf.keras.layers.Dense(64, activation='relu'),
        tf.keras.layers.Dense(128, activation='relu'),
        tf.keras.layers.Dense(128, activation='relu'),
        tf.keras.layers.Dense(num_actions * 20),
        tf.keras.layers.Reshape((num_actions, 20)),
    ])

   

model = model_build()
model.compile(optimizer=tf.keras.optimizers.Adam(learning_rate=learning_rate), loss=tf.keras.losses.MeanSquaredError())
model.load_weights("model.h5")

history_q = deque(maxlen=5000)
history_s = deque(maxlen=5000)

min_action = np.array([0.0, 0.0, -math.pi/3., -math.pi/3.])
max_action = np.array([1.0, 1.0, math.pi/3., math.pi/3.])
num_split = np.array([20, 10, 20, 20], dtype=np.int32)


def train_step(reward, action, observation, next_observation, done):
    global gamma, prev_action, history_q, history_s
    if prev_action is not None:
        observation = np.array(observation, dtype=np.float32)
        next_Q_values=model.predict([next_observation])[0]
        best_next_actions=tf.math.argmax(next_Q_values, axis=1)
        next_mask=tf.one_hot(best_next_actions, 20)
        next_best_Q_values=tf.reduce_sum(next_Q_values*next_mask, axis=1)
        target_Q_values=reward+(1-done)*gamma*next_best_Q_values
        target_Q_values = np.array([target_Q_values] * 20).T
        mask=tf.one_hot(action, 20)
        Q_values = (target_Q_values * mask)
        Q_values = np.array(Q_values, dtype=np.float32)
        history_s.insert(random.randint(0, len(history_s)), observation)
        history_q.insert(random.randint(0, len(history_q)), Q_values)
        return [next_Q_values]
    return model.predict([next_observation])
  
def compute_reward(controller: DroneController):
    distance = math.sqrt(controller.relative_position[0]**2 + controller.relative_position[1]**2)
    return -(distance**2) 

def get_state(controller: DroneController):
    return [*(np.array(controller.velocity)/10.), (np.array(controller.angular_velocity)), *np.array(controller.relative_position)/100., math.cos(controller.angle)]

def continue_actions(action):
    global min_action, max_action, num_split
    return np.array(((action/num_split) * (max_action - min_action)) + min_action, dtype=np.float32)
    
prev_observation= None
prev_action = None
prev_controller = None


def start(controller: DroneController):
    """create an instance of Gadget, configure it and return to Rust"""
    global prev_observation, prev_controller, n_episodes, prev_action
    prev_action = None
    n_episodes += 1
    prev_controller = controller
    rewards.clear()
    actions_disc = train_step(0, prev_action, prev_observation, get_state(controller), False)[0]
    prev_action = np.argmax(actions_disc, axis=1)
    actions = continue_actions(prev_action)
        
    controller.thrust_left = actions[0]
    controller.thrust_right = actions[1]
    controller.thrust_left_angle = actions[2]
    controller.thrust_right_angle = actions[3]

    prev_observation = get_state(controller)
    return controller

def random_action():
    return np.array([random.random(), random.random(), random.random()*(2./3.)*math.pi - (math.pi/3.), random.random()*(2./3.)*math.pi - (math.pi/3.)])

debug = True
start_time = 0
start_time=time.time()
last_time =time.time() - 1.

rewards = []

def loop(controller: DroneController):
    """loop function, called every frame"""
    global epsilon, last_time, prev_observation, model, prev_action, train_loss, optimizer
    if time.time() - last_time < 1./decision_rate:
        return controller
    reward = compute_reward(controller)
    rewards.append(reward)
    actions_disc = train_step(reward, prev_action, prev_observation, get_state(controller), False)[0]
    # Apply action
    prev_action = np.argmax(actions_disc, axis=1)
    actions = continue_actions(prev_action)
    
    if random.random() < epsilon - (n_episodes/10*10000.):
        actions = random_action()
    
    controller.thrust_left = actions[0]
    controller.thrust_right = actions[1]
    controller.thrust_left_angle = actions[2]
    controller.thrust_right_angle = actions[3]
    if debug:
        print("Entrainement {:5.3f} seconde(s)".format(float(time.time()-start_time)))
    prev_observation = get_state(controller)
    last_time = time.time()
    return controller


def end(controller: DroneController, reason: str):
    """called when the simulation ends"""
    print("Simulation ended because of: " + reason)
    reward = compute_reward(controller)
    if reason == "reached_target":
        reward = 100
    elif reason == "crashed":
        reward = -1000.
    rewards.append(reward)
    actions_disc = train_step(reward, prev_action, prev_observation, get_state(controller), True)[0]
    # add to file
    reward_history.append(np.mean(rewards))
    model.fit(np.array(history_s), np.array(history_q), batch_size=min(100, len(history_q)), epochs=max(1, min(len(history_q)//100, 20)))
    plt.clf()
    plt.plot(reward_history)
    print("len hist", len(history_q))
    plt.savefig("reward_history.png")
    model.save_weights("model.h5")

