# Two Pointers & Sliding Window (Python)

Each file is self-contained; run `python <file>` to execute its built-in asserts.

| File | Description |
|---|---|
| [`0003_longest_substring_without_repeating_characters.py`](0003_longest_substring_without_repeating_characters.py) | 3. Longest Substring Without Repeating Characters: Variable window with last-seen index map |
| [`0011_container_with_most_water.py`](0011_container_with_most_water.py) | 11. Container With Most Water: Converging pointers, move the shorter wall |
| [`0015_3sum.py`](0015_3sum.py) | 15. 3Sum: Sort + two pointers with duplicate skipping |
| [`0016_3sum_closest.py`](0016_3sum_closest.py) | 16. 3Sum Closest: Sort + two pointers tracking nearest sum |
| [`0018_4sum.py`](0018_4sum.py) | 18. 4Sum: Two fixed indices + two pointers |
| [`0026_remove_duplicates_from_sorted_array.py`](0026_remove_duplicates_from_sorted_array.py) | 26. Remove Duplicates from Sorted Array: Slow/fast write pointer, in place |
| [`0027_remove_element.py`](0027_remove_element.py) | 27. Remove Element: Write pointer filtering in place |
| [`0042_trapping_rain_water.py`](0042_trapping_rain_water.py) | 42. Trapping Rain Water: Two pointers with running left/right maxima |
| [`0075_sort_colors.py`](0075_sort_colors.py) | 75. Sort Colors: Dutch national flag, three pointers |
| [`0076_minimum_window_substring.py`](0076_minimum_window_substring.py) | 76. Minimum Window Substring: Expand/shrink window with need counter |
| [`0080_remove_duplicates_from_sorted_array_ii.py`](0080_remove_duplicates_from_sorted_array_ii.py) | 80. Remove Duplicates from Sorted Array II: Write pointer allowing at most two copies |
| [`0088_merge_sorted_array.py`](0088_merge_sorted_array.py) | 88. Merge Sorted Array: Merge from the back in place |
| [`0125_valid_palindrome.py`](0125_valid_palindrome.py) | 125. Valid Palindrome: Converging pointers skipping non-alphanumerics |
| [`0142_linked_list_cycle_ii.py`](0142_linked_list_cycle_ii.py) | 142. Linked List Cycle II (with 141): Floyd tortoise and hare, plus cycle entry |
| [`0167_two_sum_ii_input_array_is_sorted.py`](0167_two_sum_ii_input_array_is_sorted.py) | 167. Two Sum II - Input Array Is Sorted: Find 1-indexed pair summing to target in sorted array |
| [`0187_repeated_dna_sequences.py`](0187_repeated_dna_sequences.py) | 187. Repeated DNA Sequences: Fixed-size rolling window with bit encoding |
| [`0209_minimum_size_subarray_sum.py`](0209_minimum_size_subarray_sum.py) | 209. Minimum Size Subarray Sum: Shrinking window over positive numbers |
| [`0219_contains_duplicate_ii.py`](0219_contains_duplicate_ii.py) | 219. Contains Duplicate II: Sliding set of the last k elements |
| [`0239_sliding_window_maximum.py`](0239_sliding_window_maximum.py) | 239. Sliding Window Maximum: Monotonic deque of indices |
| [`0283_move_zeroes.py`](0283_move_zeroes.py) | 283. Move Zeroes: Swap non-zeros forward, preserving order |
| [`0287_find_the_duplicate_number.py`](0287_find_the_duplicate_number.py) | 287. Find the Duplicate Number: Floyd cycle detection on index graph |
| [`0340_longest_substring_with_at_most_k_distinct_characters.py`](0340_longest_substring_with_at_most_k_distinct_characters.py) | 340. Longest Substring with At Most K Distinct Characters: Window with a char-count map (159 is k=2) |
| [`0345_reverse_vowels_of_a_string.py`](0345_reverse_vowels_of_a_string.py) | 345. Reverse Vowels of a String (with 344): Swap from both ends |
| [`0392_is_subsequence.py`](0392_is_subsequence.py) | 392. Is Subsequence: Greedy pointer over the target |
| [`0424_longest_repeating_character_replacement.py`](0424_longest_repeating_character_replacement.py) | 424. Longest Repeating Character Replacement: Window bounded by length - maxFreq <= k |
| [`0438_find_all_anagrams_in_a_string.py`](0438_find_all_anagrams_in_a_string.py) | 438. Find All Anagrams in a String: Fixed window with frequency counter |
| [`0567_permutation_in_string.py`](0567_permutation_in_string.py) | 567. Permutation in String: Fixed window with 26-slot match counter |
| [`0643_maximum_average_subarray_i.py`](0643_maximum_average_subarray_i.py) | 643. Maximum Average Subarray I: Fixed-size running sum |
| [`0680_valid_palindrome_ii.py`](0680_valid_palindrome_ii.py) | 680. Valid Palindrome II: Converging pointers with one allowed deletion |
| [`0713_subarray_product_less_than_k.py`](0713_subarray_product_less_than_k.py) | 713. Subarray Product Less Than K: Count windows ending at each right index |
| [`0763_partition_labels.py`](0763_partition_labels.py) | 763. Partition Labels: Extend window to the last occurrence |
| [`0876_middle_of_the_linked_list.py`](0876_middle_of_the_linked_list.py) | 876. Middle of the Linked List: Slow/fast pointers |
| [`0904_fruit_into_baskets.py`](0904_fruit_into_baskets.py) | 904. Fruit Into Baskets: Longest window with at most two types |
| [`0977_squares_of_a_sorted_array.py`](0977_squares_of_a_sorted_array.py) | 977. Squares of a Sorted Array: Fill result from the largest magnitude end |
| [`0992_subarrays_with_k_different_integers.py`](0992_subarrays_with_k_different_integers.py) | 992. Subarrays with K Different Integers: exactly(K) = atMost(K) - atMost(K-1) |
| [`1004_max_consecutive_ones_iii.py`](1004_max_consecutive_ones_iii.py) | 1004. Max Consecutive Ones III: Window with at most k zeros |
| [`1423_maximum_points_you_can_obtain_from_cards.py`](1423_maximum_points_you_can_obtain_from_cards.py) | 1423. Maximum Points You Can Obtain from Cards: Minimise the complementary middle window |
| [`1838_frequency_of_the_most_frequent_element.py`](1838_frequency_of_the_most_frequent_element.py) | 1838. Frequency of the Most Frequent Element: Sort + window bounded by increment budget |
| [`smallest_difference_pair_two_arrays.py`](smallest_difference_pair_two_arrays.py) | Smallest Difference Pair Across Two Arrays: Sort both, walk two pointers |
