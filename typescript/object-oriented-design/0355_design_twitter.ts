/**
 * LeetCode 355 - Design Twitter
 * Users post tweets, follow/unfollow others, and fetch the 10 most recent tweet ids
 * from themselves and everyone they follow.
 * Approach: per-user tweet lists stamped with a global clock; merge followee lists from their newest ends.
 * Time: getNewsFeed O(10 * F) for F followees. Space: O(users + tweets + follows).
 */
export class Twitter {
  private time = 0;
  private readonly tweets = new Map<number, Array<[time: number, id: number]>>();
  private readonly follows = new Map<number, Set<number>>();

  postTweet(userId: number, tweetId: number): void {
    if (!this.tweets.has(userId)) this.tweets.set(userId, []);
    this.tweets.get(userId)!.push([this.time++, tweetId]);
  }

  getNewsFeed(userId: number): number[] {
    const users = new Set(this.follows.get(userId) ?? []);
    users.add(userId);
    const ptrs = [...users].map((u) => {
      const list = this.tweets.get(u) ?? [];
      return { list, i: list.length - 1 };
    });
    const feed: number[] = [];
    while (feed.length < 10) {
      let best = -1;
      for (let k = 0; k < ptrs.length; k++) {
        const p = ptrs[k];
        if (p.i < 0) continue;
        if (best < 0 || p.list[p.i][0] > ptrs[best].list[ptrs[best].i][0]) best = k;
      }
      if (best < 0) break;
      feed.push(ptrs[best].list[ptrs[best].i--][1]);
    }
    return feed;
  }

  follow(followerId: number, followeeId: number): void {
    if (followerId === followeeId) return;
    if (!this.follows.has(followerId)) this.follows.set(followerId, new Set());
    this.follows.get(followerId)!.add(followeeId);
  }
  unfollow(followerId: number, followeeId: number): void {
    this.follows.get(followerId)?.delete(followeeId);
  }
}

const tw = new Twitter();
tw.postTweet(1, 5);
console.log(tw.getNewsFeed(1)); // [5]
tw.follow(1, 2);
tw.postTweet(2, 6);
console.log(tw.getNewsFeed(1)); // [6, 5]
tw.unfollow(1, 2);
console.log(tw.getNewsFeed(1)); // [5]
