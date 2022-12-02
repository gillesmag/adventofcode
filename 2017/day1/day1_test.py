import day1
import unittest

class TestInput(unittest.TestCase):

    def test_captcha_sum_examples(self):
        examples = [("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)]
        for ex in examples:
            capsum = day1.captcha_sum(ex[0])
            self.assertEqual(capsum, ex[1])

    def test_halfway_step_captcha(self):
        examples = [
            ("1212", 6), ("1221", 0), ("123425", 4), ("123123", 12),
            ("12131415", 4)
        ]

        for ex in examples:
            capsum = day1.captcha_sum(ex[0], len(ex[0]) // 2)
            self.assertEqual(capsum, ex[1])

if __name__ == '__main__':
    unittest.main()
