# djs

[![Linux build status](https://travis-ci.org/trevershick/djs.svg?branch=master)](https://travis-ci.org/trevershick/djs)

To Do
----
* init the .rc file
* change latestSuccessful and lastKeepForever to see if there are ANY builds and report on that
* We need an option to download without mangling,just a simple overwrite in the dir.
* we need an option to download without mangling but make directories to mimic the information from the filename, project,branch,etc.
* after a git guess, check to see if that build exists
* cli validator
* add tests

Done
----
* a git guess in a non git folder should return None
* 'guess' the project. it's reasonable to assume the directory that djs is being executed in is the project directory, confirm with Jenkins once guess is made
* add 'latest' to the mix
* a git guess should only override defaults, notthing from a djsrc file (compare_and_set semantics with source)
* finding artifact doesn't work when there's > 1, specifying the entire relativePath doesn't work either (build-test/package/ClientDeploy.xml)


Nice to Haves
----
* we should be able to 'guess' at a solution by looking for 'DELETE.xml' and taking the root (not the airgapped one)

