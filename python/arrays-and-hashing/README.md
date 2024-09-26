# Arrays and Hashing (Python)

Each file is standalone; run `python <file>` to execute its self-checks.

| File | Problem |
| --- | --- |
| [0001_two_sum.py](0001_two_sum.py) | Two Sum: One pass with a hash map value -> index; check complement before inserting. |
| [0217_contains_duplicate.py](0217_contains_duplicate.py) | Contains Duplicate: Track seen values in a set, stop at the first repeat. |
| [0219_contains_duplicate_ii.py](0219_contains_duplicate_ii.py) | Contains Duplicate II: Store the last index of each value; compare distance on repeat. |
| [0242_valid_anagram.py](0242_valid_anagram.py) | Valid Anagram: Compare character frequency counters. |
| [0049_group_anagrams.py](0049_group_anagrams.py) | Group Anagrams: Key each word by its 26-letter count tuple (avoids sorting each word). |
| [0347_top_k_frequent_elements.py](0347_top_k_frequent_elements.py) | Top K Frequent Elements: Count, then bucket sort by frequency (bucket index = count). |
| [0238_product_of_array_except_self.py](0238_product_of_array_except_self.py) | Product of Array Except Self: Prefix products left-to-right into the output, then multiply by a running suffix. |
| [0036_valid_sudoku.py](0036_valid_sudoku.py) | Valid Sudoku: One pass, recording (kind, index, digit) keys in a single set. |
| [0128_longest_consecutive_sequence.py](0128_longest_consecutive_sequence.py) | Longest Consecutive Sequence: Put values in a set; only start counting from values whose predecessor is absent. |
| [0271_encode_and_decode_strings.py](0271_encode_and_decode_strings.py) | Encode and Decode Strings: Length-prefix each string: '<len>#<payload>'. Decoding reads digits up to '#'. |
| [0560_subarray_sum_equals_k.py](0560_subarray_sum_equals_k.py) | Subarray Sum Equals K: Prefix sums with a hash map of prefix -> occurrences; add count[prefix - k]. |
| [0525_contiguous_array.py](0525_contiguous_array.py) | Contiguous Array: Map 0 -> -1, track first index of each running balance; equal balance = zero-sum span. |
| [0974_subarray_sums_divisible_by_k.py](0974_subarray_sums_divisible_by_k.py) | Subarray Sums Divisible by K: Count prefix sums by residue mod k; pairs with equal residue form valid subarrays. |
| [0523_continuous_subarray_sum.py](0523_continuous_subarray_sum.py) | Continuous Subarray Sum: Store the first index of each prefix residue mod k; a repeat at distance >= 2 wins. |
| [0169_majority_element.py](0169_majority_element.py) | Majority Element: Boyer-Moore voting: keep a candidate and a counter. |
| [0229_majority_element_ii.py](0229_majority_element_ii.py) | Majority Element II: Extended Boyer-Moore with two candidates, then verify counts. |
| [0041_first_missing_positive.py](0041_first_missing_positive.py) | First Missing Positive: Cyclic placement: swap each value v in [1, n] to index v-1; first mismatch is the answer. |
| [0448_find_all_numbers_disappeared_in_an_array.py](0448_find_all_numbers_disappeared_in_an_array.py) | Find All Numbers Disappeared in an Array: Mark presence by negating nums[v-1]; indices still positive are missing. |
| [0442_find_all_duplicates_in_an_array.py](0442_find_all_duplicates_in_an_array.py) | Find All Duplicates in an Array: Negate nums[v-1] on visit; if it is already negative, v is a duplicate. |
| [0287_find_the_duplicate_number.py](0287_find_the_duplicate_number.py) | Find the Duplicate Number: Treat i -> nums[i] as a linked list; Floyd's cycle detection finds the cycle entry. |
| [0268_missing_number.py](0268_missing_number.py) | Missing Number: XOR all indices 0..n with all values; pairs cancel, leaving the missing one. |
| [0136_single_number.py](0136_single_number.py) | Single Number: XOR everything; duplicates cancel out. |
| [0349_intersection_of_two_arrays.py](0349_intersection_of_two_arrays.py) | Intersection of Two Arrays: Set intersection, iterating over the smaller set. |
| [0350_intersection_of_two_arrays_ii.py](0350_intersection_of_two_arrays_ii.py) | Intersection of Two Arrays II: Counter intersection (min of counts). |
| [0205_isomorphic_strings.py](0205_isomorphic_strings.py) | Isomorphic Strings: Compare the 'first-occurrence index' pattern of both strings. |
| [0290_word_pattern.py](0290_word_pattern.py) | Word Pattern: Two dictionaries enforcing the mapping in both directions. |
| [0383_ransom_note.py](0383_ransom_note.py) | Ransom Note: Counter subtraction: the note's counts must be covered by the magazine's. |
| [0387_first_unique_character_in_a_string.py](0387_first_unique_character_in_a_string.py) | First Unique Character in a String: Count characters, then scan for the first with count 1. |
| [0454_4sum_ii.py](0454_4sum_ii.py) | 4Sum II: Meet in the middle: count all a+b sums, then look up -(c+d). |
| [0705_design_hashset.py](0705_design_hashset.py) | Design HashSet: Separate chaining over a list of buckets with resize at load factor 0.75. |
| [0706_design_hashmap.py](0706_design_hashmap.py) | Design HashMap: Separate chaining with [key, value] pairs and doubling resize. |
| [0380_insert_delete_getrandom_o1.py](0380_insert_delete_getrandom_o1.py) | Insert Delete GetRandom O(1): Array of values + dict value -> index; remove swaps the target with the last element. |
| [count_pairs_with_given_sum.py](count_pairs_with_given_sum.py) | Count Pairs With Given Sum: Single pass; add frequency of the complement seen so far, then record the value. |
| [0599_minimum_index_sum_of_two_lists.py](0599_minimum_index_sum_of_two_lists.py) | Minimum Index Sum of Two Lists: Index the first list in a dict, scan the second tracking the best sum. |
| [rabin_karp_string_search.py](rabin_karp_string_search.py) | Rabin-Karp String Search: Rolling hash mod a large prime; verify candidate matches to rule out collisions. |
| [0146_lru_cache.py](0146_lru_cache.py) | LRU Cache: Hash map with insertion order (OrderedDict); move_to_end on access, popitem(last=False) to evict. |
