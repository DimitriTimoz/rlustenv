from rlustenv_api import *

def start(controller: Controller):
    """create an instance of Gadget, configure it and return to Rust"""
    return controller

positions = []
def loop(controller: Controller):
    """loop function, called every frame"""
    print(controller.position)
    print(controller.name)
    return controller
