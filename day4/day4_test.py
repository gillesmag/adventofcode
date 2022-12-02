import day4
import unittest


class TestInput(unittest.TestCase):

    def test_no_anagrams(self):
        examples = [
            ("abcde fghij", True),
            ("abcde xyz ecdab", False),
            ("a ab abc abd abf abj", True),
            ("iiii oiii ooii oooi oooo", True),
            ("oiii ioii iioi iiio", False)
        ]

        for ex in examples:
            self.assertEqual(not(day4.contains_anagrams(ex[0])), ex[1])


if __name__ == '__main__':
    unittest.main()
