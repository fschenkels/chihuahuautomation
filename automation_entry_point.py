import os
import sys
import glob
import inspect
import importlib

sys.path.insert(0, "automation")
from framework.pr_automation_template import PRAutomationTemplate

if __name__ == "__main__":
    print(f"current dir is: {os.getcwd()}")

    pr_subclasses = list()
    imported_modules = list()
    for module in glob.iglob("*.py", root_dir="automation"):
        print(f"Found module: {module}")
        imported_module = importlib.import_module(
            module.replace(".py", "")
        )
        imported_modules.append(
            imported_module
        )

        pr_subclasses.extend([
            cls for _, cls in inspect.getmembers(
                imported_module, inspect.isclass
            ) if issubclass(cls, PRAutomationTemplate)
            and not cls is PRAutomationTemplate
        ])

    print(f"subclasses are: {str(pr_subclasses)}")

    for subclass in pr_subclasses:
        print(
            f"{type(subclass)} says: {subclass().are_you_alive()}"
        )
