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
