Cool path finding algorithm. Similar to A* but still kind of brute forcy, checking every possible path. Could be turned into A* by assigning distance values from the destination but that's not needed for this problem.

For part 2, I just reversed the path, going from top to bottom. Otherwise the shortest path could be find by bruteforcing, starting with every a and finding the shortest. Luckily, reversing only took few lines of code change.
