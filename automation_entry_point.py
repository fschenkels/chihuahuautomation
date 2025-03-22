import os
import sys
import glob
import inspect
import logging
import importlib

from constants import USER_FOLDER
from event_handler import GithubEventHandler

logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s",
    level=logging.DEBUG
)

if __name__ == "__main__":
    logging.debug(f"current dir is: {os.getcwd()}")

    GithubEventHandler().process_event()
