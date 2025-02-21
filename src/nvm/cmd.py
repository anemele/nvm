from .op_local import query_local_info
from .op_remote import get_index_semver


def cmd_list():
    info = query_local_info()
    if len(info.versions) == 0:
        print("No local node.js installation found.")
        return

    for version in info.versions:
        if version == info.current:
            print(f"* {version}")
        else:
            print(f"  {version}")


def cmd_list_remote():
    map_ver, vec_ver = get_index_semver()
    for version in vec_ver:
        print(f"  {version:7} ==>  {map_ver[version]}")
