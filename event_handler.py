import sys

from github import GithubContext
from users_code_parser import UsersCodeParser

# add the user's folder to the path, this is a dependency to everything else
sys.path.insert(0, USER_FOLDER)

# imports the ABC meant as interface with the user's code
from framework.pr_automation_template import PRAutomationTemplate

class GithubEventHandler:
    """Handles Github events"""
    def __init__(self):
        self.context = GithubContext()
        self.users_code = UsersCodeParser()

    def process_event(self):
        if self.context.is_pr():
            classes_to_run = self.users_code.get_implementations_of(
                PRAutomationTemplate
            )
            for cls in classes_to_run:
                print(cls().are_you_alive())
