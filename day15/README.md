This one deserves a README.md

Holly this was hard. I tried to implement part 1 solution in part 2 aswell but I knew since the start that it won't be fast enough. Since it was 4 million ^2 operations.

I first tried to create a vector of bools with initial false states and the algorithm filled the vector with true if the signal covered there. When we detect a vector with a false in it after the loop, then that's how we know where it was. This was way too slow. ( O(n^2 * m) with large array initializations)

In my second attempt I decided to go for checking every possible position within the boundary and bruteforce the location with manhattan distances. That was slightly faster since we didn't have to allocate vector with 4 million elements but it was still not fast enough. (O(n^2 * m) lightweight but slow)

My last attempt, I went back to line aprroach, calculating each intersecting range for the y coordinate that is being looped. This is similar to my part 1 and part 2 first attempt. But main difference is that instead of creating range objects and looping through them, I just merged them into one big range. This time, ranges are just two integers that represent upper and lower bounds. This only took number comparasions so the operations were lightweight and time complexity also dropped. ( O(n * m) )
To know which y coordinate was the one we were searching for, the algorithm checked if the range merge was successul. With another function, the algorithm checks which x coordinate is not covered by the ranges and calculates the final result.

This one took solid brainpower. I hope oncoming days won't be too harsh.
