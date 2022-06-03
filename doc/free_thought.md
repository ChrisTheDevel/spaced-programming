# Free thoughts

This document contains ideas and thoughts about this project. I may or may not contain anything reflecting the code-state/functionality of the application as it currently is. It serves mainly as a way for me to structure my thoughts by being forced to put them down in ASCII.
# 02 June 2022
Wrote quite a good error abstraction module that I believe should scale well. I still have to implement the functions that queries the db.

The focus right now is finishing the draft to those functions and then start testing everything.

How should i handle concurrent tests when my db does not support concurrent access? How can I create and delete the temporary directories concurrently?
- Every database test gets it's own database instance to use. 
- the function that tries to create the directory should have some retry logic such that it may fail but then try agains after a short duration. This mean I can still utilize the concurrent test framework provided by cargo.

I also streamed todays work in twitch. That was a lot of fun! I will probably create a youtube channel where I'll upload as well.

# 25 may 2022

What should my schema for each item be like?
- I need to include all data required by my spaced-rs library
- I need to include enough information to quickly retrieve the items that I am suposed to review today.
- I need to save the notes related to each card. (text field containing markdown)

# 22 May 2022

I want a way to combine the practice of solving CP problems on sites such as leetcode/kattis/oa with the practice of spaced repetition.

Features I want:
- continuous flow of new problems.
  - flow of urls
  - hook into rss feeds
  - input manual list of urls into queue
- interaction with xdg-open to launch urls
  - if I want to support multiple sources of problems then I should not launch an editor or init boilerplate project. This since they all have different input/output. At least not in an initial version.
- tui
  - enter new sources of problems
  - write notes to existing problem

Every review event should launch the url in the browser and then wait for the user to press a button which indicates the end of the review. By this point the user should have achieved a valid solution. When the user presses this button they should also see a bunch of notes related to the problem they've reviewed (containing terms, things to keep in mind). The user can then evaluate their performance (as required by the spaced-rs library).

## Architechture

async ui? - nice buzzword that I have no idea how to implement. I can identify a busy-loop when I see one but I can't really imagine how to design without it. This will have to be a topic of research if I want a design that utilizes it.

- https://www.reactivemanifesto.org/glossary#Message-Driven
