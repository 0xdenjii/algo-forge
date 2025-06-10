# Problem: LeetCode 1125 - Smallest Sufficient Team
#
# Given required skills and people with skill lists, return the smallest set of
# people covering all required skills.
#
# Approach: DP over skill masks: dp[mask] = smallest team reaching mask; relax with each person.
# Complexity: O(P 2^S) time, O(2^S) space

def smallest_sufficient_team(req_skills: list[str], people: list[list[str]]) -> list[int]:
    idx = {s: i for i, s in enumerate(req_skills)}
    full = (1 << len(req_skills)) - 1
    dp: dict[int, list[int]] = {0: []}
    for p, skills in enumerate(people):
        pm = 0
        for s in skills:
            pm |= 1 << idx[s]
        if pm == 0:
            continue
        for mask, team in list(dp.items()):
            nm = mask | pm
            if nm not in dp or len(dp[nm]) > len(team) + 1:
                dp[nm] = team + [p]
    return dp[full]


if __name__ == "__main__":
    assert sorted(smallest_sufficient_team(["java", "nodejs", "reactjs"],
                                           [["java"], ["nodejs"], ["nodejs", "reactjs"]])) == [0, 2]
    print("ok")
