"""
Given three integer arrays nums1, nums2, and nums3, return a distinct array containing all the values that are present in at least two out of the three arrays. You may return the values in any orde
"""


def two_out_of_three(nums1: list[int], nums2: list[int], nums3: list[int]) -> list[int]:
    n1s, n2s, n3s = set(nums1), set(nums2), set(nums3)
    return list((n1s & n2s) | (n2s & n3s) | (n1s & n3s))


def main() -> None:
    case1 = two_out_of_three([1, 1, 3, 2], [2, 3], [3])
    assert case1 == [2, 3]

    case2 = two_out_of_three([3, 1], [2, 3], [1, 2])
    assert case2 == [1, 2, 3]

    case3 = two_out_of_three([1, 2, 2], [4, 3, 3], [5])
    assert not case3


if __name__ == "__main__":
    main()
