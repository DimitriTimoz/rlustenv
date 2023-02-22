from rlustenv_api import *

def start(controller: Controller):
    """create an instance of Gadget, configure it and return to Rust"""
    return controller

def loop(controller: Controller):
    """loop function, called every frame"""
    print("Hello from Python!")
    print(controller.position)
    return controller