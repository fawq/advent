from typing import Counter
from algo import read_lines_to_vec_i32
import numpy as np

def get_from_counter(key, counter):
    result = counter.get(key)
    return result if result is not None else 0

def day_02_main() -> int:
    array_numbers = read_lines_to_vec_i32("python/advent_2024_01/data/data.txt")
    array_counter_of_left = Counter(array_numbers[:, 1])
    get_counter = np.vectorize(get_from_counter)
    similarity_score = array_numbers[:, 0] * get_counter(array_numbers[:, 0], array_counter_of_left)

    return similarity_score.sum()