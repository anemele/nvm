from pathlib import Path

NODE_HOME = Path.home() / ".nodejs"
NODE_HOME.mkdir(exist_ok=True)
NODE_ALL = NODE_HOME / "all"
NODE_ALL.mkdir(exist_ok=True)
NODE_BIN = NODE_HOME / "bin"
NODE_TMP = NODE_HOME / "tmp"
NODE_TMP.mkdir(exist_ok=True)
