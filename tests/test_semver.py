from semver import Version

from nvm.semver import map_versions


def test_map_versions():
    s = [
        "20.0.0",
        "19.1.0",
        "19.0.1",
        "18.19.0",
        "18.18.2",
        "18.18.1",
        "18.18.0",
        "18.17.1",
    ]
    res_map, res_vec = map_versions(s)
    assert len(res_vec) == 9
    assert res_vec == [
        "18.17",
        "18.18",
        "18.19",
        "18",
        "19.0",
        "19.1",
        "19",
        "20.0",
        "20",
    ]
    assert res_map["20"] == Version(20, 0, 0)
    assert res_map["19"] == Version(19, 1, 0)
    assert res_map["18"] == Version(18, 19, 0)
    assert res_map["18.18"] == Version(18, 18, 2)
