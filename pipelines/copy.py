from shutil import copyfile


def name():
    return "copy"


def desc():
    return "copy the src file to dst"


def attributes():
    return {
        "src": {
            "desc": "a source file to be copied",
            "value_type": "Str",
            "default_value": None
        },
        "dst": {
            "desc": "a destination path for the copied file to be placed",
            "value_type": "Str",
            "default_value": None
        }
    }


def execute(values: dict):
    copyfile(values["src"], values["dst"])
