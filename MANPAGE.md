# NAME

**djs** - download files from Jenkins

# SYNOPSIS

**djs** \[**-nhqVv**\] \[**-u** url\] \[**-e** path\] \[**-b** base\]
\[**-j** build\] \[**-d** destination\] \[**-p** project\] \[**-s**
solution\] \[**-S** solution\_filter\] \[**-D** *destination-template*\]

# DESCRIPTION

This manual page describes the **djs** utility. It has been built to
work with Jenkins when using the **Folders** plugin and expects a
hierarchical folder setup like:

**/Base/Project/Branch**

**Base** can contain zero or more elements like **/MyCorp/job/GroupA**.

**djs** strings together the parameters provided to form API requests to
Jenkins to locate builds and artifacts. If you have a non-standard (from
djs’ perspective) it might take a little experimentation to get your
setup configured correctly.

It’s easy to do this using the **-n**(dry-run) and **-v**(verbose)
options on the command line. Once you’ve determined what your proper and
path are, add them to your *~/.djsrc* file in the following fashion:

    url = "http://myjenkins.server:8080"
    base = "job/MyTopLevel"

Once they’re in *~/.djsrc* you’ll not need to set them again. You can
also specify the **–branch** and **–project** and **–build** if you want
but if you execute **djs** from within a local *git* repo, it will
*guess* setting **project** to the current git project directory and
**branch** to the current *git* branch. It will also default **build**
to *lastSuccessfulBuild* and utilizing the configuration it already has,
it will interrogate Jenkins for the actual build number. The only thing
left to do is to specify the file name to download via **–solution**. It
is a common pattern to specify solution in the *git* project repository
in the following fashion:

    solution = "myfile.xml"

Once this is set, **djs** will combine **./.djsrc** with the settings in
**~/.djsrc** and any command line options to form a full artifact
download URL and it will proceed to download the file and save it in
**–destination**.

## FLAGS

  - \-n, –dry-run  
    If set to true, nothing will be downloaded.
  - \-h, –help  
    Prints help information
  - \-q, –quiet  
    Turns off output
  - \-V, –version  
    Prints version information
  - \-v, –verbose  
    If set to true, extra information will be sent to the console

## OPTIONS

  - \-u, --url *jenkins-url*  
    Use *jenkins-url* to contact the Jenkins server. This should NOT
    include any trailing slashes. An example of this is
    *http://myjenkins.com:8080*

  - \-e, --base *path*  
    Use *path* as the base path which is appended onto the *jenkins-url*
    which forms the first part of the full Jenkins URLs used to
    interrogate Jenkins for build information and download files (ex. if
    *path*=**job/MyCorp**
    http://jenkins.com/**job/MyCorp**/job/Project/job/…)

  - \-p, --project *project*  
    Use *project* in the Jenkins URL paths (ex. *project*=**MyProject**
    http://jenkins.com/BasePath/job/**MyProject**/job/…)

  - \-b, --branch *branch*  
    Use *branch* to locate the artifact. This is used in the
    construction of the URLs used to interrogate Jenkins (ex. if
    *branch*=x, http://jenkins.com/BasePath/job/Project/job/**x**/…)

  - \-j, --build *build-specifier*  
    Use *build-specifier* to locate the job from which to download
    artifacts. *build-specifier* may be a number or any of the following
    symbolic specifiers:
    
    **lastSuccessfulBuild** - interrogate Jenkins for the last
    successful build of the given *branch* and *project*. Once resolved,
    the *build* will be internally updated to that build number.
    
    **lastKeepForever** - locate the last build that is being ‘kept
    forever’
    
    **latest** - the latest build failure or not

  - \-s, --solution *solution*  
    Specifies the file to locate in Jenkins and download (ex.
    MySolution.xml)

  - \-S, --solution-filter *filter*  
    Use *filter* to disambiguate between artifacts in Jenkins. If more
    than one *solution* exists in the artifacts of the specific *build*
    then the relative path of the artifact will be inspected to find a
    match for *filter*.

  - \-d, --destination *destination*  
    Use *destination* as the location to which files should be
    downloaded. If the location is not a directory then it’s taken to be
    a file name and the artifact will be downloaded to *destination*
    irrespective of the artifact name, build, branch or any other
    information.

  - \-D, --destination-template *template*  
    Use *template* to format the output filename. See the **Destination
    Template** section.

## Destination Path

By default djs will download your file to the current directory unless
otherwise specified by `destination` or `-d` which can be either a
directory or a filename. If `destination` points to a directory then djs
will rename the downloaded file with the following template:

``` 
  {project}-{branch}-{build}.{extension} #yields myproj-mybranch-15.xml
```

or if your branch name “djs-123” contains the project name “djs” then

``` 
  {branch}-{build}.{extension} #yields djs-15.xml
```

You can customize this via the `destination_template` configuration
value in your .djsrc file or via the `-D` command line option. The value
is a string that contains format specifiers. Examples are shown
below.

``` 
  {project}-{solution_basename}-{solution_filter}-{branch}-{build}-{build_abbreviation}.{solution_extension}
  #yields
  ./proj1-solution-filter1-branch1-latest-lt.txt"
```

## Format Specifiers

All format specifiers come in three flavors. Lowercase, Uppercase and
Preserve Case. Shown below are the output values for various specifiers
and the input value “Project1”.

| Input    | Format  | Output   |
| -------- | ------- | -------- |
| Project1 | project | project1 |
| Project1 | Project | Project1 |
| Project1 | PROJECT | PROJECT1 |

**All Specifiers**

All the specifiers below are available in the three variants mentioned
above, Lowercase, Uppercase and Preserve
Case

| Specifier           | Example Value                                              |
| ------------------- | ---------------------------------------------------------- |
| project             | project1                                                   |
| solution            | myfile.xml                                                 |
| solution\_basename  | myfile                                                     |
| solution\_extension | xml                                                        |
| solution\_filter    | build-test                                                 |
| branch              | 1\_fix\_caps                                               |
| branch\_nums        | 1                                                          |
| branch\_alphas      | fixcaps                                                    |
| build               | 15                                                         |
| build\_abbreviation | ls (ls=lastSuccessfulBuild, lt=latest, kf=lastKeepForever) |

# FILES

  - *~/.djsrc*  
    Per-user default file.

  - *./.djsrc*  
    Per directory defaults.

# BUGS

See GitHub Issues: <https://github.com/trevershick/djs/issues>

# AUTHOR

Trever Shick <trever.shick@tanium.com>
