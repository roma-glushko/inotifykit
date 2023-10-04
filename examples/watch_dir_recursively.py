import os
from pathlib import Path

from inotify_toolkit import Notifier


def watch(watched_dir: Path) -> None:
    with Notifier() as notifier:
        notifier.watch([watched_dir])

        event = notifier.get()
        event

        # for event in notifier:
        #     print(event)


if __name__ == "__main__":
    watched_dir = Path("./watched_dir")
    os.makedirs(watched_dir, exist_ok=True)

    watch(watched_dir)
