package sandbox

func BinarySearch(nums []int, target int) int {
	low, high := 0, len(nums)-1

	for low <= high {
		mid := (low + high) / 2
		switch v := nums[mid]; {
		case v == target:
			return mid
		case v < target:
			low = mid + 1
		default:
			high = mid - 1
		}
	}

	return -1
}

func RecursiveBinarySearch(nums []int, target, low, high int) int {
	if low <= high {
		mid := (low + high) / 2
		switch v := nums[mid]; {
		case v == target:
			return mid
		case v < target:
			return RecursiveBinarySearch(nums, target, mid+1, high)
		default:
			return RecursiveBinarySearch(nums, target, low, mid-1)
		}
	}

	return -1
}
