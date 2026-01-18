# String Algorithms (Rust)

One problem per file. Each file is a standalone program (`rustc --edition 2021 file.rs && ./file`), and its `main` checks the examples with asserts.

## LeetCode problems

| File | Description |
|------|-------------|
| 0003_longest_substring_without_repeating_characters.rs | Sliding window with a last-seen table |
| 0005_longest_palindromic_substring_manacher.rs | Manacher's algorithm, O(n) |
| 0006_zigzag_conversion.rs | Simulates writing the rows in a zigzag |
| 0010_regular_expression_matching.rs | DP for `.` and `*` regex matching |
| 0014_longest_common_prefix.rs | Shrinks the prefix across all strings |
| 0020_valid_parentheses.rs | Stack of expected closing brackets |
| 0028_find_index_of_first_occurrence_kmp.rs | KMP prefix function search |
| 0043_multiply_strings.rs | Schoolbook big-number multiplication |
| 0044_wildcard_matching.rs | Greedy matching that backtracks to the last `*` |
| 0049_group_anagrams.rs | HashMap keyed by letter-count signature |
| 0072_edit_distance.rs | Levenshtein DP with one rolling row |
| 0076_minimum_window_substring.rs | Sliding window with a need-count table |
| 0139_word_break.rs | Prefix DP over the word lengths in the dictionary |
| 0151_reverse_words_in_a_string.rs | Splits on whitespace, reverses and joins |
| 0187_repeated_dna_sequences.rs | 2-bit rolling hash over 10-letter windows |
| 0208_implement_trie.rs | Trie with nodes stored in a Vec arena |
| 0214_shortest_palindrome.rs | KMP on s#rev(s) to find the longest palindromic prefix |
| 0242_valid_anagram.rs | Unicode-aware char counting |
| 0424_longest_repeating_character_replacement.rs | Sliding window with a max-frequency count |
| 0438_find_all_anagrams_in_a_string.rs | Fixed window that tracks how many letter counts match |
| 0459_repeated_substring_pattern.rs | Smallest period from the KMP prefix function |
| 0516_longest_palindromic_subsequence.rs | Interval DP stored in O(n) space |
| 0567_permutation_in_string.rs | Fixed window that compares count arrays |
| 0647_palindromic_substrings.rs | Expands around each of the 2n-1 centers |
| 0686_repeated_string_match_rabin_karp.rs | Rabin-Karp search on the repeated string |
| 0796_rotate_string.rs | Z-function search for goal inside s+s |
| 1044_longest_duplicate_substring.rs | Binary search on length plus a double rolling hash |
| 1143_longest_common_subsequence.rs | LCS DP that also rebuilds one subsequence |
| 1392_longest_happy_prefix.rs | Last value of the prefix function |

## Classic algorithms

| File | Description |
|------|-------------|
| z_algorithm.rs | Z-function and search for all occurrences |
| aho_corasick.rs | Multi-pattern automaton with failure and output links |
| suffix_array_lcp.rs | Suffix array by prefix doubling, Kasai LCP and a count of distinct substrings |
| suffix_automaton.rs | Suffix automaton for distinct substrings, membership and longest common substring |
| boyer_moore_horspool.rs | Horspool bad-character skip search |
| booth_minimal_rotation.rs | Lexicographically smallest rotation in O(n) |
