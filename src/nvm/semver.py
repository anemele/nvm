from semver import Version

type MapVer = dict[str, Version]
type VecVer = list[str]


def map_versions(versions: list[str]) -> tuple[MapVer, VecVer]:
    res_map: MapVer = {}
    res_vec: VecVer = []

    for version in versions:
        try:
            sv = Version.parse(version)
        except Exception:
            continue

        major = str(sv.major)
        if major not in res_map:
            res_map[major] = sv
            res_vec.append(major)
        elif sv > res_map[major]:
            res_map[major] = sv

        mm = f"{sv.major}.{sv.minor}"
        if mm not in res_map:
            res_map[mm] = sv
            res_vec.append(mm)
        elif sv > res_map[mm]:
            res_map[mm] = sv

    return res_map, res_vec[::-1]
