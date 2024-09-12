# pipchecker
Tool to manage local Python dependencies for Python projects.

This tool not only checks on Pypi for new version of our dependencies but also aims to update local dependencies when triggered.

## Installation
Coming soon

### UV
It is recommended to have installed (uv)[https://github.com/astral-sh/uv] from the guys of Astral which is a tool that aims to replace pip in a near future. It is written in Rust as well and it's extremely fast. 

## How to use it
This binary works as a CLI app so, it is needed to pass some subcommands to let it know what we want to do. Subcommands will be listed here:
- *check*: This command will check local packages version against Pypi packages version and will inform if there are new versions that can be applied.
- *inspect*: This command allows us to know a little more of a given package, getting the info from Pypi.
- *update*: This command will update local packages to their latest known version on Pypi. 

