import numpy as np
import sys

def parse_num(line):
    return list(map(int, line.split(",")))

def parse_board_line(line):
    return list(map(int, line.split()))

def construct_board(lines):
    """ Construct a single board given an iterable of lines

    Args:
        lines (iterable): returns a board line one at a time

    Returns:
        np.array: nxn numpy array representing the bingo board
    """
    board = []
    for l in lines:
        bl = parse_board_line(l)  
        board.append(bl)
    return np.array(board)

def bingo(board, nums, n):
    """ Return whether there is a bingo for a single board

    Args:
        board (np.array): numpy array representing a nxn board of ints
        nums (list): a list of numbers drawn to check for a bingo
        n (int): size of the board (nxn)

    Returns:
        option: returns an int if a bingo was found, None if there was no bingo
    """
    rows = []
    cols = []
    for num in nums:
        (r, c) = np.where(board==num)
        rows.append(r[0])
        cols.append(c[0])

        if len(rows) == n or len(cols) == n:
            s = score(board, rows, cols, num)
            return s

    return None

def score(board, r_idx, c_idx, num):
    """ Return the score of a board given that a bingo was found

    Args:
        board (np.array): nxn numpy array of ints representing the board
        r_idx (list): list of row indices of the board numbers drawn
        c_idx (list): list of column indices of the board numbers drawn
        num (_type_): the last number drawn for bingo

    Returns:
        int: the score of the board calculated as the sum of all the numbers
             that were NOT drawn on the board multiplied by the last number drawn
    """
    r_idx = np.array(r_idx)
    c_idx = np.array(c_idx)
    num_drawn = board[r_idx, c_idx].sum()
    return (board.sum().sum() - num_drawn)*num

def take(n, l):
    """ Take the first n elements from an iterable l.
    Importantly, this mutates l so that the position in the iterable is not lost.

    Args:
        n (int): the number of elements to take from l
        l (iterable): an iterable that can return elements one at a time

    Returns:
        list: a list of n first elements from l
    """
    output = []
    for i, e in enumerate(l):
        output.append(e)
        if i == n-1:
            break
    return output

def main():
    lines = sys.stdin
    nums = parse_num(lines.readline())

    boards = []
    board_length = 5
    # filter for empty lines
    lines = filter(lambda x: x, lines)
    while True:
        l = take(board_length, lines)
        if len(l) < board_length:
            break

        board = construct_board(l)
        boards.append(board)
    bingo(board, nums, board_length)
    