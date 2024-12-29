def reverse_words_in_a_string_iii(s: str) -> str:
    s = list(s)
    l = 0
    
    for i in range(len(s)):
        if s[i] == " " or i == len(s) - 1:
            r = i
            
            if i == len(s) - 1 and s[i] != " ":
                r += 1
            
            while l < r:
                s[l], s[r - 1] = s[r - 1], s[l]
                l += 1
                r -= 1
                
            l = i + 1
    
    return ''.join(s)

def two_sum(nums: list[str], target: int) -> list[str]:
    appearances = {}
    for i, n in enumerate(nums):
        j = appearances.get(n)
        if j is not None:
            return [j, i]
        appearances[target - n] = i
        
def remove_duplicates(nums: list[int]) -> int:
    if not nums:
        return 0
    left = 0
    for right in range(1, len(nums)):
        if nums[left] != nums[right]:
            left += 1
            nums[left] = nums[right]
    return left + 1

def remove_element(nums: list[int], val: int) -> int:
    l = 0
    r = len(nums)
    while l < r:
        if nums[l] == val:
            nums[l] = nums[r - 1]
            r -= 1
        else:
            l += 1
    return r