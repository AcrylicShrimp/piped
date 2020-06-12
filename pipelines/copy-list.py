from shutil import copyfile


def name():
    return "copy-list"


def desc():
    return "copy the src files to dst"


def attributes():
    return {
        "src": {
            "desc": "source files to be copied",
            "value_type": "StrList",
            "default_value": None
        },
        "dst": {
            "desc": "destination paths for the copied files to be placed",
            "value_type": "StrList",
            "default_value": None
        }
    }


def execute(values: dict):
    src = values["src"]
    dst = values["dst"]

    if len(src) != len(dst):
        print("copy-list error; src and dst have different length")

    for i in range(len(src)):
        copyfile(src[i], dst[i])
