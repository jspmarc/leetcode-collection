class Solution {
    fun longestSubsequence(s: String, k: Int): Int {
        if (s.isEmpty()) {
            return 0
        }

        var power = 1U
        var result = 0U
        var ones = 0
        val k = k.toUInt()

        for (i in (s.length - 1).downTo(0)) {
            if (s[i] == '1') {
                if (power > k) {
                    break
                }

                if (result + power <= k) {
                    result += power
                    ones++
                }
            }

            if (power <= k) {
                power *= 2U
            }
        }

        return s.count { it == '0' } + ones
    }
}

fun check(result: Int, expected: Int) {
    require(result == expected) { "expected $expected, but was $result" }
}

fun main() {
    var result: Int

    result = Solution().longestSubsequence("1001010", 5)
    check(result, 5)
    println("case pass")

    result = Solution().longestSubsequence("00101001", 1)
    check(result, 6)
    println("case pass")

    result = Solution().longestSubsequence("1111111001010", 10000000)
    check(result, 13)
    println("case pass")

    result = Solution().longestSubsequence("1111111001010", 100)
    check(result, 7)
    println("case pass")

    result = Solution().longestSubsequence("1111111111111111", 100)
    check(result, 6)
    println("case pass")

    result = Solution().longestSubsequence("111100010000011101001110001111000000001011101111111110111000011111011000010101110100110110001111001001011001010011010000011111101001101000000101101001110110000111101011000101", 11713332)
    check(result, 96)
}
