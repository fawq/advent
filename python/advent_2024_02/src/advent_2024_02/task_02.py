import numpy as np
from algo import read_lines_to_vec_of_array1d_u32


def is_safe(array_numbers: np.typing.NDArray[np.uint32]) -> bool:
    for i in range(len(array_numbers) - 1):
        for j in range(i + 1, len(array_numbers)):
            if array_numbers[i] % array_numbers[j] == 0:
                return True
    return False

def day_02_main() -> int:
    array_numbers = read_lines_to_vec_of_array1d_u32(
        "python/advent_2024_02/data/data.txt"
    )
    array_numbers[0]
    return 0
