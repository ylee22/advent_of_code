import unittest
import numpy as np
from aoc4 import parse_num, parse_board_line, take, construct_board, bingo, score


class TestParse(unittest.TestCase):
    def test_parse_num(self):
        num = parse_num("7,4,9,5,11,17,23,2,0")
        self.assertEqual([7,4,9,5,11,17,23,2,0], num)

    def test_parse_board_line(self):
        board_line = parse_board_line("22 13 17 11  0")
        self.assertEqual([22,13,17,11,0], board_line)

    def test_take(self):
        self.assertEqual(take(5, [1,2,3,4]), [1,2,3,4])
        self.assertEqual(take(3, [1,2,3,4]), [1,2,3])

    def test_take_iter(self):
        """ testing that the iterator state is mutated with take and that the index is not reset
        """
        my_iter = iter([1,2,3,4,5,6,7])
        self.assertEqual(take(3, my_iter), [1,2,3])
        self.assertEqual(take(2, my_iter), [4,5])
        self.assertEqual(take(2, my_iter), [6,7])

    def test_construct_board(self):
        lines = [
                "68 73 98 51 49",
                "82 56 87 64  8",
                "46  7 21 38 30",
                "66  5 86 97 74",
                "60 63 76 55 39",
                ]

        board = construct_board(lines)
        self.assertEqual(board[2,3], 38)
        self.assertEqual(board[4,0], 60)

class TestBingo(unittest.TestCase):
    def test_score(self):
        board = np.array([
                [0,1,2],
                [3,4,5],
                [6,7,8]
                ])
        rows = [0,0,0]
        cols = [0,1,2]
        s = score(board, rows, cols, 2)
        self.assertEqual(s, 66)

    def test_bingo(self):
        board = np.array([
                        [0,1,2],
                        [3,4,5],
                        [6,7,8]
                        ])
        self.assertEqual(bingo(board, [3,4,5], 3), 120)

if __name__ == '__main__':
    unittest.main()
