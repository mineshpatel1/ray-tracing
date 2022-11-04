import unittest
from vector import cross, dot, reflect, refract, Vector

class TestVectors(unittest.TestCase):
    def test_operations(self):
        x = Vector(2, 1, 5)
        y = Vector(4, 1, 1)

        self.assertEqual((x + y).coords, (6.0, 2.0, 6.0))
        self.assertEqual((x - y).coords, (-2.0, 0.0, 4.0))
        self.assertEqual((x * y).coords, (8.0, 1.0, 5.0))
        self.assertEqual((x / 2).coords, (1.0, 0.5, 2.5))
        self.assertEqual(dot(x, y), 14.0)
        self.assertEqual(cross(x, y).coords, (-4.0, 18.0, -2.0))
        self.assertEqual(dot(x, x), x.length_squared)
        self.assertEqual(reflect(x, y).coords, (-110.0, -27.0, -23.0))
        self.assertEqual(refract(x, y, 1.2).coords, (-334.8530318289354, -83.11325795723384, -78.31325795723384))


unittest.main()