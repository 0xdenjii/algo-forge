/**
 * Title: Proxy - Virtual Proxy for Lazy Image Loading
 * Problem: A gallery holds many high-resolution images; loading all of them up front is
 *   slow. Load each image only on first display, and share one in-flight load between
 *   concurrent callers.
 * Approach: ImageProxy implements the Image interface, creating/loading the RealImage on
 *   demand and memoizing the load promise.
 * Complexity: O(1) per call after the first load; one load per image at most.
 */
interface Image { readonly name: string; display(): Promise<string> }

let loads = 0;
class RealImage implements Image {
  private constructor(readonly name: string, private pixels: number) {}
  static async load(name: string): Promise<RealImage> {
    loads++;
    await new Promise((r) => setTimeout(r, 20)); // simulate disk/network
    return new RealImage(name, 4096 * 4096);
  }
  async display(): Promise<string> { return `${this.name} (${this.pixels} px)`; }
}

class ImageProxy implements Image {
  private loading?: Promise<RealImage>;
  constructor(readonly name: string) {}
  async display(): Promise<string> {
    this.loading ??= RealImage.load(this.name);
    return (await this.loading).display();
  }
}

(async () => {
  const gallery: Image[] = ["a.jpg", "b.jpg", "c.jpg", "d.jpg"].map((n) => new ImageProxy(n));
  console.log("after construction, loads =", loads); // 0
  const [x, y] = await Promise.all([gallery[1].display(), gallery[1].display()]);
  console.log(x, "|", y, "| loads =", loads); // 1 (shared in-flight load)
  await gallery[3].display();
  console.log("loads =", loads); // 2
})();
