def run(board_original):
    board = board_original
    solve(board)
    return board


def solve(board):
    find = find_empty(board)
    if not find:
        return True
    else:
        row, col = find

    for i in range(1, 10):
        if valid(board, i, row, col):
            board[row][col] = i

            if solve(board):
                return True

            board[row][col] = 0

    return False


def valid(board, new_value, row, col):
    # Horizontal lines
    for item in board[row]:
        if item == new_value:
            return False

    # Vertical lines
    for index in range(0, 9):
        if board[index][col] == new_value:
            return False

    # In square
    index_horizontal = col - col % 3
    index_vertical = row - row % 3

    square_slice = board[index_vertical:index_vertical + 3]

    for row in square_slice:
        if new_value in row[index_horizontal:index_horizontal + 3]:
            return False

    return True


def print_board(board):
    for i in range(len(board)):
        if i % 3 == 0 and i != 0:
            print("- - - - - - - - - - - - - ")

        for j in range(len(board[0])):
            if j % 3 == 0 and j != 0:
                print(" | ", end="")

            if j == 8:
                print(board[i][j])
            else:
                print(str(board[i][j]) + " ", end="")


def find_empty(board):
    for i in range(len(board)):
        for j in range(len(board[0])):
            if board[i][j] == 0:
                return i, j

    return None
