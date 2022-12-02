import day3
import unittest


class TestInput(unittest.TestCase):

    def test_step_count(self):
        examples = [
            (1, 0),
            (12, 3),
            (23, 2),
            (1024, 31)
        ]

        for ex in examples:
            self.assertEqual(day3.step_count(ex[0]), ex[1])


if __name__ == '__main__':
    unittest.main()
