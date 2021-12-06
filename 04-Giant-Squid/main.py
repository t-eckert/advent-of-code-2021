from itertools import zip_longest
from rich import print


def main():
    puzzle = load_puzzle("./input.txt")
    called_numbers, boards = format_puzzle(puzzle)
    last_called, winner = play_game_1(called_numbers, boards)
    unmarked_sum = sum(
        sum(place[0] for place in row if not place[1]) for row in winner.places
    )
    print(f"Final score game 1: {unmarked_sum * last_called}")

    last_called, winner = play_game_2(called_numbers, boards)
    unmarked_sum = sum(
        sum(place[0] for place in row if not place[1]) for row in winner.places
    )
    print(winner.places)
    print(f"Final score game 2: {unmarked_sum * last_called}")


class Board:
    def __init__(self, numbers: list[str]) -> None:
        self.places: list[tuple[int, bool]] = []
        self.values: dict[int, tuple[int, int]] = {}
        for x, line in enumerate(numbers):
            self.places.append([])
            for y, number in enumerate(line.replace("  ", " ").strip().split(" ")):
                self.places[x].append(
                    (
                        int(number.strip()),
                        False,
                    )
                )
                self.values[int(number.strip())] = (x, y)

    def mark(self, number: int) -> bool:
        loc = self.values.get(number)
        if loc == None:
            return False
        x, y = loc
        self.places[x][y] = (self.places[x][y][0], True)
        return True

    def is_winner(self) -> bool:
        # Check rows
        if any(all(place[1] for place in row) for row in self.places):
            return True
        # Check cols
        if any(
            all(self.places[x][y][1] for y in range(len(self.places)))
            for x in range(len(self.places[0]))
        ):
            return True
        return False


def load_puzzle(path: str) -> list[str]:
    with open(path, "r") as f:
        lines = f.readlines()
    return lines


def format_puzzle(puzzle: list[str]) -> tuple[list[int], list[Board]]:
    called_numbers = [int(number) for number in puzzle[0].strip().split(",")]
    boards = [Board(group[1:]) for group in grouper(puzzle[1:], 6)]
    return (called_numbers, boards)


def play_game_1(called_numbers: list[int], boards: list[Board]) -> tuple[int, Board]:
    # Plays the called numbers and returns the winning board
    for number in called_numbers:
        for board in boards:
            was_marked = board.mark(number)
            if was_marked and board.is_winner():
                return (number, board)


def play_game_2(called_numbers: list[int], boards: list[Board]) -> tuple[int, Board]:
    # Plays to find the last board that would win.
    board_tracker = [False for _ in range(len(boards))]
    for number in called_numbers:
        for index, board in enumerate(boards):
            was_marked = board.mark(number)
            if was_marked and board.is_winner():
                board_tracker[index] = True
        if board_tracker.count(False) == 1:
            idx = board_tracker.index(False)
            return (number, boards[idx])

    return (None, None)


def grouper(iterable, n, fillvalue=None):
    "Collect data into non-overlapping fixed-length chunks or blocks"
    # grouper('ABCDEFG', 3, 'x') --> ABC DEF Gxx
    args = [iter(iterable)] * n
    return zip_longest(*args, fillvalue=fillvalue)


if __name__ == "__main__":
    main()


def test_row_win():
    numbers = [
        "12 75 58 21 87\n",
        "55 80 14 63 17\n",
        "37 35 76 92 56\n",
        "72 68 51 19 38\n",
        "91 60 34 30 88\n",
    ]

    board = Board(numbers)

    # Win in a row
    for x in [37, 35, 76, 92, 56]:
        board.mark(x)

    assert True == board.is_winner()


def test_row_col():
    numbers = [
        "12 75 58 21 87\n",
        "55 80 14 63 17\n",
        "37 35 76 92 56\n",
        "72 68 51 19 38\n",
        "91 60 34 30 88\n",
    ]

    board = Board(numbers)

    # Win in a row
    for x in [58, 14, 76, 51, 34]:
        board.mark(x)

    assert True == board.is_winner()
