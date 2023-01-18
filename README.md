# Scrape Blog Posts to Jekyll

Basic script with lots of hardcoded values to scrape an existing blog post on a DSA site, convert it to a Jekyll Markdown file, and save all the necessary assets in the expected file structure.

## TODO

* Increase flexibility
  * Could the selector text be given at runtime?
* Wayyyyyy better error handling
* Get author into liquid again
  * Format of authors isn't always the same
    * I've seen "By ...", "by ...", "By: ..."
  * Some articles have no byline
* Add support for nested lists
* Remove the Sacremento DSA links from the tests
* Translating `<br>`s to `  \n` is adding extra lines to the markdown
  * Analyze the collected vector before joining
