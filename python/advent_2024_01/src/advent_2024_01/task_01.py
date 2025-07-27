from algo import read_lines_to_vec_i32
import numpy as np


def day_01_main() -> int:
    array_numbers = read_lines_to_vec_i32("python/advent_2024_01/data/data.txt")
    array_numbers.sort(axis=0)
    array_numbers_abs = np.abs(array_numbers[:, 0] - array_numbers[:, 1])

    return array_numbers_abs.sum()