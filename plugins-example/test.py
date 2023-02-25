from rlustenv_api import *

def start(controller: DroneController):
    """create an instance of Gadget, configure it and return to Rust"""
    return controller
a = 0
positions = []
def loop(controller: DroneController):
    """loop function, called every frame"""
    global a
    print(controller.position)
    print(controller.velocity)
    return controller
