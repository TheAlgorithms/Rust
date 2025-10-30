import java.util.*;

class Solution {
    public long maxAlternatingSum(int[] nums) {
        int n = nums.length;
        int posCounts = (n + 1) / 2;
        Integer[] absVals = Arrays.stream(nums).map(Math::abs).boxed().toArray(Integer[]::new);
  Arrays.sort(absVals, (a, b) -> Integer.compare(b, a));

        long sumPos = 0, sumNeg = 0;
        for (int i = 0; i < n; i++) {
            long sq = 1L * absVals[i] * absVals[i];
            if (i < posCounts) sumPos += sq;
            else sumNeg += sq;
        }
        return sumPos - sumNeg;
    }
}
