import os

# imports the ABC meant as interface with the user's code
from pr_automation_template import PRAutomationTemplate

class GithubContext:
    """Github context info"""

    def __init__(self):
        self.context = os.environ.get(
            "GITHUB_CONTEXT",
            default=dict()
        )

    def is_pr():
        """Verify if the current event is PR related"""
        return True
        #return False

    def is_push():
        """Verify if the current event is push related"""
        return False

    def is_scheduled():
        """Verify if the current event is scheduled"""
        return False


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
