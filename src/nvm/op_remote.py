from dataclasses import dataclass

from mashumaro.mixins.json import DataClassJSONMixin


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
