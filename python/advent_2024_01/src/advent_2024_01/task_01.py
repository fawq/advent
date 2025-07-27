import numpy as np
from algo import read_lines_to_vec_i32
from numba import njit


@njit
def calculate_abs_sum(array_numbers: np.typing.NDArray[np.int32]) -> int:
    array_numbers_abs = np.abs(array_numbers[:, 0] - array_numbers[:, 1])
    return array_numbers_abs.sum()

def day_01_main() -> int:
    array_numbers = read_lines_to_vec_i32("python/advent_2024_01/data/data.txt")
    array_numbers.sort(axis=0)
    return calculate_abs_sum(array_numbers)
