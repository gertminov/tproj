# tproj

This little command line tool helps you to create temporary project folders, for
when you just want to try something out and have it deleted automatically later.
The tool is split into two parts. The CLI tool, that creates the project folder,
and the cleaner tool, that deletes the folder automatically.
The CLI Tools creates two sub directories, `working` and `out`.
If the cleanup tool finds a project, that has contents in it's `out` directory,
it will not delete the project.

It is my first endever with Rust so it's more of a learning experience than a
real tool.

## Get Started

1. open `tproj.toml` and set `tempprojectdir` to the directory that sould hold your temporary projects
2. Add `tproj.exe` to your PATH. Either by adding the instal directory to the path,
or by creating a Symlink from the exe to a directory that is already in the PATH
3. [Set up the Cleaner Tool](#cleaner-tool)

## How to use

### CLI Tool

```sh
tproj # this will create a folder with the current date and time as the name
```

or:

```sh
tproj myFoldername # the will create a folder named "myFoldername"
```

If you run both commands, your tproj folder should look somethin like this:

```
./tempProjects/
├── myFoldername/
│   ├── out
│   └── working
└── 2023-02-28 23-43/
    ├── out
    └── working
```


### Cleaner Tool
1. Open Windows task scheduler
2. Create a new task
3. Set the trigger to a schedule you like (like every week or so)
4. Set the action to "execute program" 
      1. set the path to the path of tprojCleaner.exe
