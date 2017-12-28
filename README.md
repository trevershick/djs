# djs

[![Linux build status](https://travis-ci.org/trevershick/djs.svg?branch=master)](https://travis-ci.org/trevershick/djs)

DJS is a simple utility that helps to download artifacts from Jenkins.  In my day to day work i regularly have to download builds from Jenkins for testing and development purposes.  It quickly became tedious to locate the link, copy it and ```curl``` it to the place i needed.  Thus, djs was born.

```djs``` makes it easy to repeatedly download the same file from Jenkins where the location of said file differs by Jenkins build name (branch) and build number (build).  It does this in a number of ways:
* it will 'guess' your project based on the directory you're in.
* it will 'guess' the current branch based on the git project you're in.
* it will locate the build number by number or by a symbolic name.
* it allows you to save common parameters in a ```.djsrc``` file in the current directory as well as your home directory.

Usage
----
```djs -h``` output

```
Jenkins Solution Downloader (jds) 0.1.0
Trever Shick <trever.shick@tanium.com>
Helps download solution XMLs from Jenkins

USAGE:
    djs [FLAGS] [OPTIONS]

FLAGS:
    -n               If set to true, nothing will be downloaded.
    -h, --help       Prints help information
    -q               Turns off output
    -V, --version    Prints version information
    -v               If set to true, extra information will be sent to the console

OPTIONS:
    -e, --base <Base URL before getting to project root>
    -b, --branch <BRANCH>
    -j, --build <BUILD NUMBER>
    -d, --destination <SOLUTION>                            Sets the branch to download
    -p, --project <Project Name (Jenkins Path Element)>
    -s, --solution <SOLUTION>
    -S, --solution-filter <FILTER>
    -u, --url <Jenkins URL>
```


Case Study (mine)
----
My Jenkins server is setup with folders like this:

```
<JENKINS ROOT>/MyCorp/MyProject/<branch>
```

My branch and build numbers change often but my project does not.  I almost always go to the same Jenkins server, so I set the Jenkins URL and base parameter in my ```~/.djsrc```.  Also, I download all artifacts to the same folder for easy access by my hosted VMs.

```
    # the Jenkins URL
    url = "http://192.168.1.109:8080"

    # the first part of the Jenkins URL path to my builds
    base = "/job/MyCorp"

    # download them all here:
    destination = "/Users/trever.shick/Solutions"
```

I work on multiple projects as well, so I don't want to specify everything in my home directory.  I work in ```~/workspaces/primary/MyProject``` so in the ```~/workspaces/primary/MyProject/.djsrc``` I have the following:

```
solution = "MySolution.xml"
```

Now, when I run ```djs``` in ```~/workspaces/primary/MyProject``` the last successful build is downloaded to ```/Users/trever.shick/Solutions```.

Here's the output (with -v turned on [verbose output])
```
Reading ~/.djsrc...done
Reading .djsrc...done
Determine current git branch...done
Determine the current project...done
Resolving the download URL
Jenkins Base URL (url): http://192.168.1.109:8080 [source: /Users/trever.shick/.djsrc]
Jenkins Base Path (base): /job/MyCorp [source: /Users/trever.shick/.djsrc]
Project (project): MyProject [source: ./.djsrc]
Branch (branch): master [source: git]
Build (build): 15 [source: jenkins, was lastSuccessfulBuild from defaults]
Solution (solution): MySolution.xml [source: defaults]
Solution Filter (solution_filter): <empty> [source: defaults]
Destination (destination): /Users/trever.shick/MyCorp [source: /Users/trever.shick/.djsrc]
Destination Path (destination_path): /Users/trever.shick/MyCorp/myproject-master-15.xml
Resolved URL: http://192.168.1.109:8080/job/MyCorp/job/MyCorp/job/master/15/artifact/build-test/MySolution.xml
HTTP request sent... 200 OK
Length: 7340032 (7.00MB)
Saving to: /Users/trever.shick/Solutions/myproject-mysolution-master-15.xml
/Users/trever.shick/Solutions/myproject-mysolution-master-15.xml   [00:00:00] [=======] 7.00MB/7.00MB eta: 0s
  Done
```





Why "djs"?
---
Our final build artifacts are called 'Solutions'.  So, "djs" is "D"ownload "J"enkins "S" solution.


Installation
----

The only way to install at this time is via a homebrew tap.

```
  brew tap trevershick/djs https://github.com/trevershick/djs.git
  brew install trevershick/djs
```

My Setup
----


To Do
----
* init the .rc file
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
* We need an option to download without mangling,just a simple overwrite in the dir.
* we need an option to download without mangling but make directories to mimic the information from the filename, project,branch,etc.
* after a git guess, check to see if that build exists
* change latestSuccessful and lastKeepForever to see if there are ANY builds and report on that
* we should be able to 'guess' at a solution by looking for 'DELETE.xml' and taking the root (not the airgapped one)

