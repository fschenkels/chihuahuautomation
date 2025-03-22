import os
import sys
import glob
import inspect
import logging
import importlib

from constants import USER_FOLDER

# imports the ABC meant as interface with the user's code
from pr_automation_template import PRAutomationTemplate

class UsersCodeParser:
    ABCs = [
        PRAutomationTemplate
    ]

    def __init__(self):
        self.classes_and_modules = self.__parse_classes_and_modules()

    def get_implementations_of(abc):
        return [
            cls for cls, _ in self.classes_and_modules.get(
                abc, []
            )
        ]

    def __parse_classes_and_modules(self):
        """Parses the user's concrete classes and their respective modules"""
        
        classes_and_modules = dict()
        logging.info(f"Importing concrete implementations from '{USER_FOLDER}'...")
        for module in glob.iglob("*.py", root_dir=USER_FOLDER):
            logging.debug(f"Found module: {module}")
            imported_module = importlib.import_module(
                module.replace(".py", "")
            )

            for abc in self.ABCs:
                logging.debug(f"reading implementations of {abc}...")
                implementations_found = [
                    (cls, imported_module) for _, cls in inspect.getmembers(
                        imported_module, inspect.isclass
                    ) if issubclass(cls, abc) and not cls is abc
                ]

                logging.debug(f"implementations found: {str(implementations_found)}")
                classes_and_modules[abc] = implementations_found
        return classes_and_modules

