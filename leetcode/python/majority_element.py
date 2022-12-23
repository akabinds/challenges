"""
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.
"""

from collections import Counter

def majority_element(nums: list[int]) -> int:
    return Counter(nums).most_common()[0][0]


def main() -> None:
    case1 = majority_element([3, 2, 3])
    assert case1 == 3

    case2 = majority_element([2,2,1,1,1,2,2])
    assert case2 == 2


if __name__ == "__main__":
    main()
