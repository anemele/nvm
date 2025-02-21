from dataclasses import dataclass

import orjson
import requests
from mashumaro.mixins.json import DataClassJSONMixin

from .config import load_config
from .semver import MapVer, VecVer, map_versions


@dataclass
class Index(DataClassJSONMixin):
    version: str
    # date: str
    # files: list[str]
    # npm: Optional[str]
    # v8: str
    # uv: Optional[str]
    # zlib: Optional[str]
    # openssl: Optional[str]
    # modules: Optional[str]
    # lts: Value
    # security: bool

    @classmethod
    def from_json(cls, s: str | bytes):
        data = orjson.loads(s)
        return [cls.from_dict(item) for item in data]


def get_index() -> list[Index]:
    config = load_config()
    index_url = f"{config.mirror}/index.json"
    response = requests.get(index_url, headers={"User-Agent": "NVM Client"})
    return Index.from_json(response.content)


def get_index_semver() -> tuple[MapVer, VecVer]:
    index = get_index()
    versions = [item.version[1:] for item in index]
    return map_versions(versions)
