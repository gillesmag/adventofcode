prelude = """let mut x = 0;
let mut y = 0;
let mut z = 0;
let mut w = 0;
"""

def main():
    lines = open("input.txt").read().strip().split("\n")


    print(prelude)

    ops = {
        "add": "+=",
        "mul": "*=",
        "div": "/=",
        "mod": "%=",
    }

    for line in lines:
        tokens = line.split(" ")
        if tokens[0] == "inp":
            print("println!(\"{}\", z);")
            print()
            print("{} = inputs.next().unwrap();".format(tokens[1]))
        elif tokens[0] in ["add", "mul", "div", "mod"]:
            op = ops[tokens[0]]
            if op == "*=" and tokens[2] == "0":
                print("{} = 0;".format(tokens[1]))
            else:
                print("{} {} {};".format(tokens[1], op, tokens[2]))
        elif tokens[0] == "eql":
            print("{} = ({} == {}) as i64;".format(tokens[1], tokens[1], tokens[2]))

if __name__ == "__main__":
    main()
