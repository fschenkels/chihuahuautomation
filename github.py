import os

class GithubContext:
    """Github context info"""

    def __init__(self):
        self.context = os.environ.get(
            "GITHUB_CONTEXT",
            default=dict()
        )

    def is_pr(self):
        """Verify if the current event is PR related"""
        return True
        #return False

    def is_push(self):
        """Verify if the current event is push related"""
        return False

    def is_scheduled(self):
        """Verify if the current event is scheduled"""
        return False


