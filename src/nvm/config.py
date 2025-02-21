from dataclasses import dataclass, field
from pathlib import Path

from mashumaro.mixins.toml import DataClassTOMLMixin

NVM_RC_PATH = Path.home() / ".nvmrc"


@dataclass
class Config(DataClassTOMLMixin):
    root: Path = field(default=Path.home() / ".nvm")
    nvm_all: str = field(default="all")
    nvm_bin: str = field(default="bin")
    nvm_tmp: str = field(default="tmp")
    mirror: str = field(default="https://nodejs.org/dist")


def load_config():
    if NVM_RC_PATH.exists():
        config = Config.from_toml(NVM_RC_PATH.read_text())
    else:
        config = Config()
        print(f"Creating {NVM_RC_PATH} with default configuration")
        NVM_RC_PATH.write_text(config.to_toml())

    config.root.mkdir(exist_ok=True)
    (config.root / config.nvm_all).mkdir(exist_ok=True)
    (config.root / config.nvm_tmp).mkdir(exist_ok=True)

    return config
