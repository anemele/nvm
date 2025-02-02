import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from .consts import NODE_ALL


@dataclass
class LocalInfo:
    current: str
    versions: list[str] = field(default_factory=list)


def _is_node_path(path: Path) -> bool:
    start_with_v = path.name.startswith("v")
    node_exist = (path / "node.exe").exists()
    return start_with_v and node_exist


def query_local_info() -> Optional[LocalInfo]:
    if not NODE_ALL.exists():
        return None

    stdout = subprocess.run("node --version", capture_output=True).stdout
    stdout = stdout.decode()
    current = stdout.strip().removeprefix("v")

    versions = [
        p.name.removeprefix("v") for p in NODE_ALL.iterdir() if _is_node_path(p)
    ]

    return LocalInfo(current=current, versions=versions)
