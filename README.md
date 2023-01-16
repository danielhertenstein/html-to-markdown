# Scrape Blog Posts to Jekyll

Basic script with lots of hardcoded values to scrape an existing blog post on a DSA site, convert it to a Jekyll Markdown file, and save all the necessary assets in the expected file structure.

## TODO

* Get the picture at the top if there is one
* Add a check for if two images have the same download path
* Increase flexibility
  * Could the selector text be given at runtime?
* Wayyyyyy better error handling
* Handle downloading base64 images
* Should all spans do nothing?
* Get author into liquid again
  * Format of authors isn't always the same
    * I've seen "By ...", "by ...", "By: ..."
  * Some articles have no byline
* Add support for nested lists
* Many picture paths are nested
* Add support for `<table>`s
