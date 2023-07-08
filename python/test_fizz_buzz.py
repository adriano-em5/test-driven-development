from unittest import TestCase

import pytest

import fizz_buzz


class TestFizzBuzz(TestCase):
    def test_get_string_of_number(self):
        assert fizz_buzz.fizz_buzz(1) == "1"

    def test_cannot_get_string_of_non_number(self):
        number = "5"
        error = f"number must be an integer instance, got {type(number)}'"
        with pytest.raises(ValueError, match=error):
            fizz_buzz.fizz_buzz(number)

    def test_get_fizz_for_multiples_of_three(self):
        assert fizz_buzz.fizz_buzz(9) == "Fizz"
        assert fizz_buzz.fizz_buzz(5) != "Fizz"

    def test_get_buzz_for_multiples_of_five(self):
        assert fizz_buzz.fizz_buzz(10) == "Buzz"
        assert fizz_buzz.fizz_buzz(11) != "Buzz"

    def test_get_fizzbuzz_for_multiples_of_three_and_five(self):
        assert fizz_buzz.fizz_buzz(15) == "FizzBuzz"
        assert fizz_buzz.fizz_buzz(11) != "FizzBuzz"
