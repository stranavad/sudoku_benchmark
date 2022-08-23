import time
import solver
import data_parser

# board_master = [
#     [7, 8, 0, 4, 0, 0, 1, 2, 0],
#     [6, 0, 0, 0, 7, 5, 0, 0, 9],
#     [0, 0, 0, 6, 0, 1, 0, 7, 8],
#     [0, 0, 7, 0, 4, 0, 2, 6, 0],
#     [0, 0, 1, 0, 5, 0, 9, 3, 0],
#     [9, 0, 4, 0, 6, 0, 0, 0, 5],
#     [0, 7, 0, 3, 0, 0, 0, 1, 2],
#     [1, 2, 0, 0, 0, 7, 4, 0, 0],
#     [0, 4, 9, 2, 0, 6, 0, 0, 7]
# ]

# print(solver.run(board_master))

def main():
    # measure time
    start = time.time_ns()

    data = data_parser.get_data("/home/stranavadavid/programming/sudoku/sudoku_new.csv")
    correct = 0
    incorrect = 0
    # print(len(data))
    # print(data[0])
    # solved_board = solver.run(data[0][0])
    # solver.print_board(solved_board)
    # solver.print_board(data[0][1])
    for board in data:
        solved_board = solver.run(board[0])
        # solver.print_board(solved_board)
        if solved_board == board[1]:
            correct += 1
        else:
            incorrect += 1

    end = time.time_ns()
    executed_time = (end-start)/1000000
    print("Boards: {}".format(correct))
    print("Execution time: {} miliseconds".format(executed_time))
    print("Time per board: {} miliseconds".format(executed_time/correct))


if __name__ == "__main__":
    main()

