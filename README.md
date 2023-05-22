# Finescale&mdash;A finer scale understanding of your schedule
### &#128679; Under construction. Will be finished alongside lrt and other construction :) &#128679; -> Submit a pull request.
Are you annoyed needing to spend time going through the UofA course catalog
looking through prerequisites, corequisits, descriptions, and course times? It's
a necessary activity to build and understanding of which courses you want and need
to complete your program, even more so in the case of an honors student looking
to build their schedule. 

I'm certainly tired, so I wrote this simple app to hopefully make it somewhat
easier for people to select their courses. I can't promise that you will for
sure find this tool useful in the slightest, but I promise it's not maleware. If
you're interested give it a try.

# Features
- [ ] View a prerequisite and corequisite dependency graph.
  - [ ] Select a class and view all the necessary depencencies that must be
completed before it.
- [ ] Build a schedule optimized according to certain constraints. (Like
schedule buddy)
  - [ ] When a person wants their classes.
  - [ ] The order and position of their classes to minimize walking time.
  - [ ] Distribution of classes throughout the day.
- [ ] Check schedule against requirements needed for graduation.

# Related
- [schedubuddy.com](schedubuddy.com) helps build a schedule according to a list of classes.
You need to figure out the classes you need and schdubuddy will order them.
[aarctan/schedubuddy-web](https://github.com/aarctan/schedubuddy-web)
- [abenezerBelachew/unofficial-ualberta-api](https://github.com/abenezerBelachew/unofficial-ualberta-api/blob/master/scraper.py).
Some inspiration taken.

# Credit
- [Graphviz](https://graphviz.org/) is used for laying out the graph.
- [iced](https://iced.rs/) for gui.
