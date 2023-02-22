from rlustenv_api import *

def start(controller: Controller):
    """create an instance of Gadget, configure it and return to Rust"""
    controller.id += 15
    return controller

def loop(controller: Controller):
    """loop function, called every frame"""
    controller.id += 1
    return controller