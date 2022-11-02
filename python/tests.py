import unittest
from vector import dot, Vector

class TestVectors(unittest.TestCase):
    def test_operations(self):
        x = Vector(2, 1, 5)
        y = Vector(4, 1, 1)

        self.assertEqual(dot(x, y), 14.0)
        self.assertEqual((x + y).coords, (6.0, 2.0, 6.0))
        self.assertEqual((x - y).coords, (-2.0, 0.0, 4.0))
        self.assertEqual((x * y).coords, (8.0, 1.0, 5.0))
        self.assertEqual((x / 2).coords, (1.0, 0.5, 2.5))


unittest.main()