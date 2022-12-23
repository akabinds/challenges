"""
Reversing an integer means to reverse all its digits.

For example, reversing 2021 gives 1202. Reversing 12300 gives 321 as the leading zeros are not retained.
Given an integer num, reverse num to get reversed1, then reverse reversed1 to get reversed2. Return true if reversed2 equals num. Otherwise return false.
"""


def is_same_after_reversals(num: int) -> bool:
    return (int(str(num)[::-1].lstrip("0")[::-1].lstrip("0")) == num) if num else True


def main() -> None:
    case1 = is_same_after_reversals(526)
    assert case1

    case2 = is_same_after_reversals(1800)
    assert not case2

    case3 = is_same_after_reversals(0)
    assert case3


if __name__ == "__main__":
    main()
