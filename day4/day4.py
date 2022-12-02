def contains_duplicates(s):
    words = s.split(" ")

    return len(set(words)) == len(words)


def contains_anagram(s):
    words = s.split(" ")

    has_anagrams = False
    for i, w in enumerate(words):
        for w_w in words[i+1:]:
            if sorted(w) == sorted(w_w):
                has_anagrams = True
                break

    return has_anagrams


def main():
    f = open("input.txt")
    phrases = [p.strip() for p in f]
    dup_count = sum([contains_duplicates(p) for p in phrases])
    no_anagram_count = sum([not(contains_anagram(p)) for p in phrases])

    print("Number of passphrases without duplicates:", dup_count)
    print("Number of passphrases without anagrams:  ", no_anagram_count)


if __name__ == '__main__':
    main()
