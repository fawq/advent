import numpy as np
from algo import read_lines_to_vec_i32
from numba import njit


@njit
def calculate_similarity_score(array_numbers: np.typing.NDArray[np.int32]) -> int:
    counts = np.array([np.sum(array_numbers[:, 1] == x) for x in array_numbers[:, 0]])
    similarity_score = array_numbers[:, 0] * counts

    return similarity_score.sum()

def day_02_main() -> int:
    array_numbers = read_lines_to_vec_i32("python/advent_2024_01/data/data.txt")
    return calculate_similarity_score(array_numbers)
