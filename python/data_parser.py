import csv


def get_data(path):
    return_value = []
    with open(path, newline='') as file:
        csv_file = csv.reader(file)
        for line in csv_file:
            # print(line)
            # print(line)
            return_value.append((generate_board(line[0]), generate_board(line[1])))

    return return_value


def generate_board(line):
    board = []
    # generate board
    count = 0
    for i in range(0, 9):
        board.append([])
        for j in range(0, 9):
            board[i].append(int(line[count]))
            count += 1

    return board


# get_data("/home/stranavadavid/programming/sudoku/sudoku_new.csv")
