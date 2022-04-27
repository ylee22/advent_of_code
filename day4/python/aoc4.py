import numpy as np
import io

def parse_num(line):
    return list(map(int, line.split(",")))

def parse_board_line(line):
    return list(map(int, line.split()))

def construct_board(lines):
    board = []
    for _ in range(5):
        bl = parse_board_line(lines.next())  
        board.append(bl)
    
    return np.array(board)

def bingo(board, nums):
    rows = []
    cols = []
    for n in nums:
        (r, c) = np.where(board==n)
        rows.append(r)
        cols.append(c)
    
        if len(rows) == 5 or len(cols) == 5:
            s = score(board, rows, cols)
            return s
        else:
            return False

def score(board, r_idx, c_idx):
    r_idx = np.array(r_idx)
    c_idx = np.array(c_idx)
    num_drawn = board[r_idx, c_idx].sum()
    return board.sum().sum() - num_drawn

def main():
    lines = io.StringIO()
    nums = parse_num(lines.next())

    boards = []
    # filter for empty lines
    lines = filter(lambda x: x, lines)
    while lines:
        board = construct_board(lines)
        boards.append(board)
    bingo(board, nums)
    