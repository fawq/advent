import numba as nb
import numpy as np
from algo import read_lines_to_vec_of_array1d_i8


@nb.njit(fastmath=True, parallel=True, cache=True)
def is_safe(array_number: np.typing.NDArray[np.int8]) -> np.bool:
    diff = np.diff(array_number)
    return np.all((diff >= 1) & (diff <= 3)) or np.all((diff <= -1) & (diff >= -3))

@nb.njit(fastmath=True, parallel=True, cache=True)
def count_safe(array_numbers: list[np.typing.NDArray[np.int8]]) -> int:
    counter: int = 0
    for array_number in array_numbers:
        if is_safe(array_number):
            counter += 1
    return counter

def day_01_main() -> int:
    array_numbers = read_lines_to_vec_of_array1d_i8(
        "python/advent_2024_02/data/data.txt"
    )
    return count_safe(array_numbers)
