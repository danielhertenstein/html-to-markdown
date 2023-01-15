# Scrape Blog Posts to Jekyll

Basic script with lots of hardcoded values to scrape an existing blog post on a DSA site, convert it to a Jekyll Markdown file, and save all the necessary assets in the expected file structure.

## TODO

* Get the picture at the top if there is one
* Add a check for if two images have the same download path
* Increase flexibility
  * Could the selector text be given at runtime?
* Wayyyyyy better error handling
* Handle downloading base64 images
* Need to iterate over children of body instead of select `<p>`s
