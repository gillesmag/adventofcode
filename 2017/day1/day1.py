def captcha_sum(s, step=1):
    captcha_sum = 0
    for i, digit in enumerate(s):
        if digit == s[(i+step) % len(s)]:
            captcha_sum += int(digit)
    return captcha_sum

def main():
    # Don't forget to strip input, we'll check all chars but we don't care about
    # newline(s)
    num = open("input.txt").read().strip()
    num2 = open("input2.txt").read().strip()

    print("First part solution:")
    print(captcha_sum(num))

    print("Second part solution:")
    print(captcha_sum(num2, len(num2) // 2))

if __name__ == '__main__':
    main()
