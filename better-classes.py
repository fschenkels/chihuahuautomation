@dataclass
class Event:
    context: dict
    # TODO: type: str = ""

    def define_event_type(self):
        pass

    @property
    def is_push(self):
        self.define_event_type()
        return self.type == "push"

    @property
    def is_pr(self):
        self.define_event_type()
        return self.type == "pr"

    @property
    def is_scheduled(self):
        self.define_event_type()
        return self.type == "scheduled"

class ExecutableEvent(Event):
    def execute(self):
        pass

class GithubPREvent(ExecutableEvent)
    pass

class GitlabExecutableEvent(ExecutableEvent)
    pass

class AzureExecutableEvent(ExecutableEvent)
    pass


class PRAutomationTemplate:
    def pr_open(self):
        """Executed whenever a new PR is created"""
        pass

    def pr_updated(self):
        """Executed whenever a PR is updated"""
        pass

    def pr_closed(self):
        """Executed whenever a PR is closed"""
        pass


class GithubPRAutomationTemplate(GithubPREvent, PRAutomationTemplate):
    pass

class EventsFactory:
    def __init__(self):
        self.vendor = self.__verify_vendors_environment()
        self.available_events = {
            "github": GithubEvent,
            "gitlab": GitlabEvent,
            "azure": AzureEvent
        }

    def __verify_vendors_environment(self):
        # maybe a routine for figuring out the vendor from the 
        # execution environment at the startup
        return False

    def __extract_vendor_from_context(self, context)
        pass

    def generate_event(self):
        # TODO: context = os.environ.get( 
        if not self.vendor:
            self.vendor = self.__extract_vendor_from_context(context)
        return self.available_events[
            self.vendor
        ](
            context=context
        )

class EventsProcessor:
    def __init__(self):
        self.userscode = UsersCodeParser()


    def process(self, event):
        if event.is_pr:
            classes_to_run = self.userscode.get_implementations_of(
                [PRAutomationTemplate]
            )
        for cls in classes_to_run:
            cls.process()
