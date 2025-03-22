import os

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


