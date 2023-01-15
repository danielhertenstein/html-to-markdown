# Scrape Blog Posts to Jekyll

Basic script with lots of hardcoded values to scrape an existing blog post on a DSA site, convert it to a Jekyll Markdown file, and save all the necessary assets in the expected file structure.

## TODO

* Scrape multiple sites
* Convert `<img>`s inside `<p>`s and `<strong>`s
* Increase flexibility
  * Could the selector text be given at runtime?
* Wayyyyyy better error handling
* Put the date where it is supposed to go
* Put the author where it is supposed to go
* Handle links to other pages on the same site
* Make sure pdf downloads work
* Fix permalink
  * Should be `/blog/<year>/<month>/<day>/<something>`
