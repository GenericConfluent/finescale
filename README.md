# Finescale&mdash;A finer scale understanding of your schedule
### &#128679; Under construction. Will be finished alongside LRT and other construction :) &#128679; &Rightarrow; Submit a pull request.
Are you annoyed needing to spend time going through the UofA course catalogue
looking through prerequisites, corequisites, descriptions, and course times?
It's a necessary activity to build an understanding of which courses you want
and need to complete your program, even more so in the case of an honours
student looking to build their schedule.

I'm certainly tired, so I wrote this simple app to hopefully make it somewhat
easier for people to select their courses. I can't promise that you will for
sure find this tool useful, but I promise it's not malware. If you're interested
give it a try.

## Features
- [ ] Scrape the UAlberta catalogue to build a course graph.
- [x] Autofill required courses based on a list of desired courses and the course graph.
- [x] Organize required courses into a course set (a collection of classes that could be taken over a semester)
- [ ] Export course sets to Schedubuddy.
- [ ] Build a schedule optimized according to certain constraints:
  - [ ] The order and position of their classes to minimize walking time.
  - [ ] Distribution of classes throughout the day.
- [ ] Requirement templates: Allow specification of credit breakdowns, courses that must be taken in a specific
year, etc. Useful for things like honours student planners.

## Related
- [schedubuddy.com](schedubuddy.com) helps build a schedule according to a list of classes.
You need to figure out the classes you need and Schedubuddy will generate a bunch of prospective schedules based
on when available classes are for the term of choice.
[aarctan/schedubuddy-web](https://github.com/aarctan/schedubuddy-web)
- [abenezerBelachew/unofficial-ualberta-api](https://github.com/abenezerBelachew/unofficial-ualberta-api/blob/master/scraper.py).
Some inspiration was taken.

## Credit
- [`layout-rs`](https://github.com/nadavrot/layout) provides the graph layouts.
- [`iced`](https://iced.rs/) for gui.
