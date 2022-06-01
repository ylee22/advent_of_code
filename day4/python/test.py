import unittest
from aoc4 import parse_num, parse_board_line, construct_board


class Test(unittest.TestCase):
    def test_everything(self):
        num = parse_num("7,4,9,5,11,17,23,2,0")
        self.assertEqual([7,4,9,5,11,17,23,2,0], num)
        board_line = parse_board_line("22 13 17 11  0")
        self.assertEqual([22,13,17,11,0], board_line)

        lines = ["",
                "68 73 98 51 49",
                "82 56 87 64  8",
                "46  7 21 38 30",
                "66  5 86 97 74",
                "60 63 76 55 39",
                "",
                "92 20 87 77 52",
                "72 29 81 24 64",
                "26 16 19 79 68",
                "8 53 90 14 74",
                "28 89 78 54 15",
                "",
                "13 17 35  2 85",
                "37 87 57 74 65",
                "60 21 18 98 96",
                "4 51 46 84  0",
                "90 75 80 41 64",
                ""]

        board = construct_board(lines)
        self.assertEqual(board[0][2,3], 38)
    

if __name__ == '__main__':
    unittest.main()
