/**
 * LeetCode 355 - Design Twitter
 * Support postTweet, getNewsFeed (10 most recent tweets from self + followees), follow, unfollow.
 * Approach: per-user tweet lists stamped with a global clock; k-way merge from each list's tail.
 * Time: getNewsFeed O(10 * F) for F followees. Space: O(users + tweets).
 */
export class Twitter {
  private time = 0;
  private tweets = new Map<number, [number, number][]>(); // [time, tweetId]
  private follows = new Map<number, Set<number>>();

  postTweet(userId: number, tweetId: number): void {
    if (!this.tweets.has(userId)) this.tweets.set(userId, []);
    this.tweets.get(userId)!.push([this.time++, tweetId]);
  }

  getNewsFeed(userId: number): number[] {
    const users = [userId, ...(this.follows.get(userId) ?? [])];
    const ptr = users.map(u => (this.tweets.get(u)?.length ?? 0) - 1);
    const out: number[] = [];
    while (out.length < 10) {
      let best = -1;
      let bestT = -1;
      users.forEach((u, i) => {
        if (ptr[i] >= 0) {
          const t = this.tweets.get(u)![ptr[i]][0];
          if (t > bestT) {
            bestT = t;
            best = i;
          }
        }
      });
      if (best < 0) break;
      out.push(this.tweets.get(users[best])![ptr[best]--][1]);
    }
    return out;
  }

  follow(a: number, b: number): void {
    if (a === b) return;
    if (!this.follows.has(a)) this.follows.set(a, new Set());
    this.follows.get(a)!.add(b);
  }

  unfollow(a: number, b: number): void {
    this.follows.get(a)?.delete(b);
  }
}

const tw = new Twitter();
tw.postTweet(1, 5);
tw.follow(1, 2);
tw.postTweet(2, 6);
console.log(tw.getNewsFeed(1)); // [6, 5]
tw.unfollow(1, 2);
console.log(tw.getNewsFeed(1)); // [5]
