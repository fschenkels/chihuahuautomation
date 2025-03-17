import os
import sys
import inspect
import importlib

sys.path.insert(0, "automation")
automation_module = importlib.import_module("automation")
#framework_module = importlib.import_module("framework.pr_automation_template")

#from framework_module import PRAutomationTemplate
from framework.pr_automation_template import PRAutomationTemplate

if __name__ == "__main__":
    print(f"current dir is: {os.getcwd()}")

    #subcls = [cls for _, cls in inspect.getmembers(automation_module, inspect.isclass) if issubclass(cls, framework_module.PRAutomationTemplate)]
    subcls = [cls for _, cls in inspect.getmembers(automation_module, inspect.isclass) if issubclass(cls, PRAutomationTemplate)]

    print(f"subclasses are: {str(subcls)}")