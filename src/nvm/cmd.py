from .op_local import query_local_info


def cmd_list():
    info = query_local_info()
    if info is None or len(info.versions) == 0:
        print("No local node.js installation found.")
        return

    for version in info.versions:
        if version == info.current:
            print(f"* {version}")
        else:
            print(f"  {version}")
