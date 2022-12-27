"""
Given a 0-indexed integer array nums, return the smallest index i of nums such that i mod 10 == nums[i], or -1 if such index does not exist.

x mod y denotes the remainder when x is divided by y.
"""


def smallest_equal(nums: list[int]) -> int:
    for idx, e in enumerate(nums):
        if idx % 10 == e:
            return idx
        else:
            continue

    return -1


def main() -> None:
    case1 = smallest_equal([0, 1, 2])
    assert not case1

    case2 = smallest_equal([4, 3, 2, 1])
    assert case2 == 2

    case3 = smallest_equal([1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
    assert case3 == -1


if __name__ == "__main__":
    main()
