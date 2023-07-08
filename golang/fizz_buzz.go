package fizz_buzz

import "strconv"

const (
	MULTIPLE_OF_THREE_DIVIDENT = 3
	MULTIPLE_OF_FIVE_DIVIDENT  = 5
	ZERO_REMINDER              = 0
)

/*
fizz_buzz takes an integer as input and returns a string based on the following conditions:
  - If the number is divisible by both 3 and 5, it returns "FizzBuzz".
  - If the number is divisible by 3, it returns "Fizz".
  - If the number is divisible by 5, it returns "Buzz".
  - Otherwise, it returns the string representation of the input number.
*/
func fizz_buzz(number int) string {
	var (
		multiple_of_three bool = number%MULTIPLE_OF_THREE_DIVIDENT == ZERO_REMINDER
		multiple_of_five  bool = number%MULTIPLE_OF_FIVE_DIVIDENT == ZERO_REMINDER
	)

	if multiple_of_three && multiple_of_five {
		return "FizzBuzz"
	}

	if multiple_of_three {
		return "Fizz"
	}

	if multiple_of_five {
		return "Buzz"
	}

	return strconv.Itoa(number)
}
