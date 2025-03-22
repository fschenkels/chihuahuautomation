import os
import sys
import glob
import inspect
import logging
import importlib

from constants import USER_FOLDER
from github import GithubEvenHandler

logging.basicConfig(
    format="%(asctime)s - %(levelname)s - %(message)s",
    level=logging.DEBUG
)

# add the user's folder to the path, this is a dependency to everything else
sys.path.insert(0, USER_FOLDER)

if __name__ == "__main__":
    logging.debug(f"current dir is: {os.getcwd()}")

    GithubEventHandler.process_event()
