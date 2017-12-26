# djs

To Do
----
* add 'latest' to the mix
* 'guess' the project. it's reasonable to assume the directory that djs is being executed in is the project directory, confirm with Jenkins once guess is made
* init the .rc file
* after a git guess, check to see if that build exists
* change latestSuccessful and lastKeepForever to see if there are ANY builds and report on that
* a git guess in a non git folder should return None
* a git guess should only override defaults, notthing from a djsrc file (compare_and_set semantics with source)
* we should be able to 'guess' at a solution by looking for 'DELETE.xml' and taking the root (not the airgapped one)
* finding artifact doesn't work when there's > 1, specifying the entire relativePath doesn't work either (build-test/package/ClientDeploy.xml)
* We need an option to download without mangling,just a simple overwrite in the dir.
* we need an option to download without mangling but make directories to mimic the information from the filename, project,branch,etc.

* cli validator

* add tests
