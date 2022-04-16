def normalize_esd_code(input: str):
    input.replace("-", "/")
    return "C-" + input
