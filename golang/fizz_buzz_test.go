package fizz_buzz

import "testing"

func TestFizzBuzzGetString(t *testing.T) {

	got := fizz_buzz(4)
	want := "4"

	if got != want {
		t.Errorf("got %q, wanted %q", got, want)
	}
}

func TestFizzBuzzMultipleOfThree(t *testing.T) {

	got := fizz_buzz(27)
	want := "Fizz"

	if got != want {
		t.Errorf("got %q, wanted %q", got, want)
	}
}

func TestFizzBuzzMultipleOfFive(t *testing.T) {

	got := fizz_buzz(25)
	want := "Buzz"

	if got != want {
		t.Errorf("got %q, wanted %q", got, want)
	}
}

func TestFizzBuzzMultipleOfThreeAndFive(t *testing.T) {

	got := fizz_buzz(30)
	want := "FizzBuzz"

	if got != want {
		t.Errorf("got %q, wanted %q", got, want)
	}
}
