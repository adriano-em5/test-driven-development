MULTIPLE_OF_THREE_DIVIDENT = 3
MULTIPLE_OF_FIVE_DIVIDENT = 5
ZERO_REMINDER = 0


def fizz_buzz(number: int):
    """
    Converts a number to a string following the FizzBuzz rules.

    FizzBuzz rules:
    - For multiples of three return “Fizz” instead of the number.
    - For the multiples of five return “Buzz”.
    - For numbers that are multiples of both three and five return “FizzBuzz”.

    Parameters:
    - number (int): The number to be converted.

    Returns:
    - str: The converted string according to the FizzBuzz rules.

    Raises:
    - ValueError: If the input is not an integer.
    """
    if not isinstance(number, int):
        raise ValueError(f"number must be an integer instance, got {type(number)}'")

    multiple_of_three: bool = number % MULTIPLE_OF_THREE_DIVIDENT == ZERO_REMINDER
    multiple_of_five: bool = number % MULTIPLE_OF_FIVE_DIVIDENT == ZERO_REMINDER

    if multiple_of_three and multiple_of_five:
        return "FizzBuzz"

    if multiple_of_three:
        return "Fizz"

    if multiple_of_five:
        return "Buzz"

    return str(number)
