def spiral_position(n):
    edge_length = 1
    square_side_counter = 0
    x, y = 0, 0
    for i in range(1, n+1):
        # hit a corner, change direction
        if x == y:
            pass
        
        square_side_counter += 1



def main():
    pos = spiral_position(3)


if __name__ == '__main__':
    main()
