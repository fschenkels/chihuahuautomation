import logging

from framework.github import GithubContext

class PRAutomationTemplate(GithubContext):
    def __init__(self):
        super().__init__()

    def pr_open(self):
        """Executed whenever a new PR is created"""
        pass

    def pr_updated(self):
        """Executed whenever a PR is updated"""
        pass

    def pr_closed(self):
        """Executed whenever a PR is updated"""
        pass

