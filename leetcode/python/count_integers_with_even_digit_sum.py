"""
Given a positive integer num, return the number of positive integers less than or equal to num whose digit sums are even.

The digit sum of a positive integer is the sum of all its digits.
"""


def count_even(num: int) -> int:
    count = 0

    for n in range(1, num + 1):
        if n < 10 and not n % 2:
            count += 1
        else:
            nums = [int(n) for n in str(n)]

            if not sum(nums) % 2:
                count += 1

    return count


def main() -> None:
    case1 = count_even(4)
    assert case1 == 2

    case2 = count_even(30)
    assert case2 == 14


if __name__ == "__main__":
    main()
