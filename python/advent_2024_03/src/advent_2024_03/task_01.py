import re

import numpy as np


def sum_of_muls(path: str, mul_pattern_regexp: re.Pattern) -> int:
    output = np.fromregex(
        path,
        mul_pattern_regexp,
        [("first_number", np.uint32), ("second_number", np.uint32)],
    )
    product = output["first_number"] * output["second_number"]
    return product.sum()


def day_03_task_01_main() -> int:
    mul_pattern_regexp = re.compile(
        r"mul\((?P<first_number>\d{1,3}),(?P<second_number>\d{1,3})\)"
    )
    return sum_of_muls(
        "python/advent_2024_03/data/data.txt", mul_pattern_regexp=mul_pattern_regexp
    )
