from enum import Enum

class EventType(Enum):
    PUSH = 1
    PULL_REQUEST = 2
    SCHEDULED = 3

class PullRequestType(Enum):
    OPENED = 1
    UPDATED = 2
    CLOSED = 3

class Vendor(Enum):
    GITHUB = 1
    GITLAB = 2
    AZURE = 3

class Event:
    def __init__(
        self,
        context: dict,
    ):
        self.context = context
        # TODO: parse it
        self.etype = EventType.PUSH
        self.vendor = Vendor.GITHUB

    @property
    def is_push(self):
        return EventType.PUSH == self.etype

    @property
    def is_pr(self):
        return EventType.PULL_REQUEST == self.etype

    @property
    def is_scheduled(self):
        return EventType.SCHEDULED == self.etype

class ExecutableEvent(Event):
    def __init__(
        self,
        context: dict,
    ):
        super().__init__(context)

    def execute(self):
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

class PullRequestExecutableEvent(ExecutableEvent):
    def __init__(
        self,
        context: dict,
        template: PRAutomationTemplate,
    ):
        ExecutableEvent.__init__(context)
        self.subtype = PullRequestType.UPDATED
        self.parse_subtype()
        self.template = template
        self.template.context = context

    def execute(self):
        if PullRequestType.OPENED == self.subtype:
            self.template.pr_open()
        elif PullRequestType.UPDATED == self.subtype:
            self.template.pr_updated()
        else:
            self.template.pr_closed()

    def parse_subtype(self):
        pass

class GithubPRExecutableEvent(PullRequestExecutableEvent):
    def __init__(
        self,
        context: dict,
        template: PRAutomationTemplate,
    ):
        super().__init__(
            context,
            template
        )

    def parse_subtype(self):
        # concrete implementation
        pass

    @classmethod
    def from_event_and_template(cls, event, template):
        return cls(
            context = context
            template = template 
        )

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
