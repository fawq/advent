import re
from io import StringIO
from pathlib import Path
from typing import Literal

import numpy as np


def get_segments(
    line: str, split_by: Literal["do()", "don't()"] = "don't()"
) -> list[str]:
    splitted_segments = line.split(split_by, maxsplit=1)
    if split_by == "don't()":
        if len(splitted_segments) == 1:
            return [splitted_segments[0]]
        return [splitted_segments[0], *get_segments(splitted_segments[1], "do()")]
    if len(splitted_segments) == 1:
        return []
    return [*get_segments(splitted_segments[1], "don't()")]


def sum_of_muls_in_segment(string_io: StringIO, mul_pattern_regexp: re.Pattern) -> int:
    output = np.fromregex(
        string_io,
        mul_pattern_regexp,
        [("first_number", np.uint32), ("second_number", np.uint32)],
    )
    product = output["first_number"] * output["second_number"]
    return product.sum()


def sum_of_muls(lines: str, mul_pattern_regexp: re.Pattern) -> int:
    sum_of_muls: int = 0
    segments = get_segments(lines)
    for segment in segments:
        string_io = StringIO(segment)
        sum_of_muls += sum_of_muls_in_segment(string_io, mul_pattern_regexp)

    return sum_of_muls


def day_03_task_02_main() -> int:
    mul_pattern_regexp = re.compile(
        r"mul\((?P<first_number>\d{1,3}),(?P<second_number>\d{1,3})\)"
    )

    lines = Path("python/advent_2024_03/data/data.txt").read_text()
    return sum_of_muls(lines, mul_pattern_regexp=mul_pattern_regexp)
