import numpy as np
from algo._core import read_lines_to_vec_of_array1d_i8


# @nb.njit(fastmath=True, parallel=True, cache=True)
def is_increasing(array_number: np.typing.NDArray[np.int8], max_errors: int) -> bool:
    diffs = np.diff(array_number)
    for index, diff in enumerate(diffs):
        if diff <= 0 or diff > 3:
            if max_errors == 0:
                return False
            left_array_number_without_first = array_number[:index]
            left_array_number_without_second = array_number[: index + 1]
            right_array_number_without_first = array_number[index + 1 :]
            right_array_number_without_second = array_number[index + 2 :]

            array_number_without_first = np.concatenate(
                (left_array_number_without_first, right_array_number_without_first)
            )
            array_number_without_second = np.concatenate(
                (left_array_number_without_second, right_array_number_without_second)
            )
            return is_increasing(
                array_number_without_first, max_errors - 1
            ) or is_increasing(array_number_without_second, max_errors - 1)
    return True


# @nb.njit(fastmath=True, parallel=True, cache=True)
def is_decreasing(array_number: np.typing.NDArray[np.int8], max_errors: int) -> bool:
    diffs = np.diff(array_number)
    for index, diff in enumerate(diffs):
        if diff >= 0 or diff < -3:
            if max_errors == 0:
                return False
            left_array_number_without_first = array_number[:index]
            left_array_number_without_second = array_number[: index + 1]
            right_array_number_without_first = array_number[index + 1 :]
            right_array_number_without_second = array_number[index + 2 :]

            array_number_without_first = np.concatenate(
                (left_array_number_without_first, right_array_number_without_first)
            )
            array_number_without_second = np.concatenate(
                (left_array_number_without_second, right_array_number_without_second)
            )
            return is_decreasing(
                array_number_without_first, max_errors - 1
            ) or is_decreasing(array_number_without_second, max_errors - 1)
    return True


# @nb.njit(fastmath=True, parallel=True, cache=True)
def is_safe(array_number: np.typing.NDArray[np.int8]) -> bool:
    return is_increasing(array_number, 1) or is_decreasing(array_number, 1)


def count_safe(array_numbers: list[np.typing.NDArray[np.int8]]) -> int:
    counter: int = 0
    for array_number in array_numbers:
        if is_safe(array_number):
            counter += 1
    return counter


def day_02_task_02_main() -> int:
    array_numbers = read_lines_to_vec_of_array1d_i8(
        "python/advent_2024_02/data/data.txt"
    )
    return count_safe(array_numbers)
