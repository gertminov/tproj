# tproj

This little command line tool helps you to create temporary project folders, for
when you just want to try something out and have it deleted automatically later.
The tool is split into two parts. The CLI tool, that creates the project folder,
and the cleaner tool, that deletes the folder automatically.

It is my first endever with Rust so it's more of a learning experience than a
real tool

## How to use

### CLI Tool

```sh
tproj # this will create a folder with the current date and time as the name
```

or:

```sh
tproj myFoldername # the will create a folder named "myFoldername"
```

### Cleaner Tool
1. Open Windows task scheduler
2. Create a new task
3. Set the trigger to a schedule you like (like every week or so)
4. Set the action to "execute program" 
      1. set the path to the path of tempCleaner.exe
      2. add arguments -p <path/to/temp/projects/folder>
