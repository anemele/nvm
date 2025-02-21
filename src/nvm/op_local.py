import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable, Optional

from .config import load_config


@dataclass
class LocalInfo:
    versions: list[str] = field(default_factory=list)
    current: Optional[str] = None


def _get_node_vers(path: Path) -> Iterable[str]:
    for p in path.iterdir():
        if not p.is_dir():
            continue
        m = re.match(r"node-v(\d+\.\d+\.\d+)", p.name)
        if m is not None:
            yield m.group(1)


def query_local_info() -> LocalInfo:
    config = load_config()
    node_all = config.root / config.nvm_all

    versions = list(_get_node_vers(node_all))

    cmd = ["node", "--version"]
    try:
        stdout = subprocess.run(cmd, capture_output=True).stdout
    except FileNotFoundError:
        return LocalInfo(versions=versions)

    stdout = stdout.decode().strip()
    s = re.search(r"node-v(\d+\.\d+\.\d+)", stdout)
    if s:
        current = s.group(1)
    else:
        current = None
    return LocalInfo(versions=versions, current=current)
