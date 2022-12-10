This one was really cool, not too complicated but still made me think. 

When it comes to continuous loops like game loops or stuff like this, it seems like that the order of the operations are not that important but reordering them correclty is important and hard to get it right at first. This is what happened in this problem. I coudln't figure out the order of operations and skipped important ones with `continue` statements. 

In this problem I implemented a multithread solution at first because ~~I didn't want to mix up input parsing with cycles. There needed to be a lot of conditions to have the two loops in one thread work correctly.~~ As I was writing this sentence I just realised I could just use `iter().next()` instead of having a seperate thread and channel to send commands. 

Well, I added an extra part 3 with this implementation for part 2. At least it was a mini practice for multithreaded programming lol.

My part 2 result. It's really cool that have prepared a problem like this instead of the sum of conditions or the highest value like they usually are.

```
####  ##  #    #  # ###  #    ####   ## 
#    #  # #    #  # #  # #    #       # 
###  #    #    #### ###  #    ###     ##
#    # ## #    #  # #  # #    #       ##
#    #  # #    #  # #  # #    #    #  ##
####  ### #### #  # ###  #### #     ##  
```
