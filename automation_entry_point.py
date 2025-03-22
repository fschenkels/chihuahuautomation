import os
import sys
import glob
import inspect
import logging
import importlib

USER_FOLDER = "automation"

logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s",
    level=logging.DEBUG
)

# add the user's folder to the path, this is a dependency to everything else
sys.path.insert(0, USER_FOLDER)

# imports the ABC meant as interface with the user's code
from framework.pr_automation_template import PRAutomationTemplate

class UsersCodeParser:
    @staticmethod
    def parse(abcs: list):
        """Returns a list of touples containing the user's concrete classes and
        their respective modules"""
        classes_and_modules = list()

        logging.info(f"Importing concrete implementations from '{USER_FOLDER}'...")
        for module in glob.iglob("*.py", root_dir=USER_FOLDER):
            logging.debug(f"Found module: {module}")
            imported_module = importlib.import_module(
                module.replace(".py", "")
            )

            for abc in abcs:
                logging.debug(f"reading implementations of {abc}...")
                implementations_found = [
                    (cls, imported_module) for _, cls in inspect.getmembers(
                        imported_module, inspect.isclass
                    ) if issubclass(cls, abc) and not cls is abc
                ]

                logging.debug(f"implementations found: {str(implementations_found)}")
                classes_and_modules.extend(implementations_found)

        return classes_and_modules


if __name__ == "__main__":
    logging.debug(f"current dir is: {os.getcwd()}")

    for concrete_class, _ in UsersCodeParser.parse(
        [PRAutomationTemplate]
    ):
        logging.info(
            f"{concrete_class} says: {concrete_class().are_you_alive()}"
        )
