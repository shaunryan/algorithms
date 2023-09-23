
# from leet.easy.two_sum import two_sum

# answer = two_sum([2,7,11,15], 9)
# print(answer)

# from leet.easy.merge_sort import merge_sort, printList


# arr = [12, 11, 13, 5, 6, 7]
# merge_sort(arr)

# print(arr)

from leet.easy.quick_sort import quick_sort

data = [1, 7, 4, 1, 10, 9, -2]
 
size = len(data)
 
quick_sort(data, 0, size - 1)
 
print('Sorted Array in Ascending Order:')
print(data)