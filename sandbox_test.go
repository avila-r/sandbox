package sandbox_test

import (
	"testing"

	"sandbox"
)

func Test_BinarySearch(t *testing.T) {
	cases := []struct {
		input    []int
		target   int
		expected int
	}{
		{input: []int{1, 2, 3, 4, 5}, target: 3, expected: 2},
		{input: []int{1, 2, 3, 4, 5}, target: 1, expected: 0},
		{input: []int{1, 2, 3, 4, 5}, target: 5, expected: 4},
		{input: []int{1, 2, 3, 4, 5}, target: 6, expected: -1},
		{input: []int{}, target: 3, expected: -1},
		{input: []int{1}, target: 1, expected: 0},
		{input: []int{1}, target: 2, expected: -1},
	}

	for _, c := range cases {
		result := sandbox.BinarySearch(c.input, c.target)

		if result != c.expected {
			t.Errorf("Failed: input %v, target %d, expected %d, got %d\n",
				c.input, c.target, c.expected, result)
		}
	}

}
