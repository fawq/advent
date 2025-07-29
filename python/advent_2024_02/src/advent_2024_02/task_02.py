from algo import read_lines_to_array2d_u32


def day_02_main() -> int:
    array_numbers = read_lines_to_array2d_u32("python/advent_2024_02/data/data.txt")
    array_numbers.sort(axis=0)
    return 0
