from typing import *

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

def search_insert_position(nums: list[int], val: int) -> int:
    left, right = 0, len(nums)
    while left < right:
        mid = int(left + (right - left) / 2)
        if nums[mid] < target:
            left = mid + 1
        elif nums[mid] > target:
            right = mid
        else:
            return mid
    return left

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def reverse_linked_list(head: Optional[ListNode]) -> Optional[ListNode]:
    new = None
    while head:
        node = head.next
        head.next = new
        new = head
        head = node
    return new

def middle_node(head: Optional[ListNode]) -> Optional[ListNode]:
    fast = head
    while fast and fast.next:
        fast, head = fast.next.next, head.next
    return head

def has_cycle(head: Optional[ListNode]) -> bool:
    fast = head
    while fast and fast.next:
        fast, head = fast.next.next, head.next
        if head == fast:
            return True
    return False

def bubble_sort(nums: list[int]) -> list[int]:
    size = len(nums)
    for _ in nums:
        print(nums)
        is_sorted = True
        for n in range(size - 1):
            if nums[n] > nums[n + 1]:
                nums[n + 1], nums[n] = nums[n], nums[n + 1]
                is_sorted = False
        if is_sorted:
            return
    return nums