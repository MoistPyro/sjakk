FILENAME_IN = "bobby_game.txt"
FILENAME_OUT = "bobby_game_tidy.txt"

with open(FILENAME_IN, mode="rb") as file:
    content = str(file.read())
    content = content.replace("\\n", " ")
    moves = content.split(".")

    one_turn_per_line = []
    for s in moves[1:]:
        temp_moves = s.strip()
        temp_moves = temp_moves.split(" ")
        temp_moves.pop()

        temp = " ".join(temp_moves)

        one_turn_per_line.append(temp)

    one_turn_per_line.pop()

with open(FILENAME_OUT, mode="w") as file:
    for line in one_turn_per_line:
        file.write(line + "\n")