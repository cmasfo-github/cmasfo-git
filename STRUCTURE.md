
# Structure

* _archive
* _dev-main
* cmasfo-app
  * dev-app
* cmasfo-cms
  * dev-cms
* cmasfo-ssg
  * dev-ssg
* cmasfo-web
  * dev-web
* dev-base
* rust

## Archive

I collect lots of datas that are related to development here.

There are language-specific features, or just some development theories.

## The Dev Crate

The rust dev crate has 6 branches.

It starts with 'base' branch, and ends with 'main' branch.

And there are 4 branches between them.

All product projects are dependent on each of them.

'dev-base' offers a starting template,  
each repos modify it for their own purposes,  
and 'dev-main' tries to merge all of them into one.

Then it restarts from the 'dev-base'.

## The Rust Project

This is an unique project for the main repository.

I implement lots of features, while I'm maintaining them as one project.

Those features are mostly from the archive.

And this also works like the test version of dev crate.
