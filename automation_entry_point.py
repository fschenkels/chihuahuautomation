import os
import sys
import glob
import inspect
import importlib

sys.path.insert(0, "automation")
from framework.pr_automation_template import PRAutomationTemplate

if __name__ == "__main__":
    print(f"current dir is: {os.getcwd()}")

    imported_modules = list()
    for module in glob.iglob("*.py", root_dir="automation"):
        print(f"Found module: {module}")
        imported_modules.append(
            importlib.import_module(module.replace(".py", ""))
        )

    pr_subclasses = [
        cls for _, cls in inspect.getmembers(
            automation_module, inspect.isclass
        ) if issubclass(cls, PRAutomationTemplate)
    ]

    print(f"subclasses are: {str(pr_subclasses)}")
