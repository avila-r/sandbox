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
    
string = "Let's take LeetCode contest"
print(reverse_words_in_a_string_iii(string))