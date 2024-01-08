# tproj

### [Download](https://github.com/gertminov/tproj/releases/latest)

This little command line tool helps you to create temporary project folders, for
when you just want to try something out and have it deleted automatically later.
The tool is split into two parts. The CLI tool, that creates the project folder,
and the cleaner tool, that deletes the folder automatically. The CLI Tools
creates two sub directories, `working` and `out`. If the cleanup tool finds a
project, that has contents in it's `out` directory, it will not delete the
project.

# Get Started

## Automatic Install (Windows)

1. Download the
   [tproj-setup.msi](https://github.com/gertminov/tproj/releases/latest)
2. Run the installer
3. Choose a folder where the temporary projects should be stored (in the
   installer)
4. Success

Now every time you click the `tproj.exe` a fresh new folder will be created,
that is deleted in a week. The `tproj.exe` shoud be added to your start menu, so
typing `[Windows Key] tproj [Enter]` should be a very fast way to create a new
folder

## Manual Way

### Windows

1. open `tproj.toml` and set `tempprojectdir` to the directory that sould hold
   your temporary projects
2. Add `tproj.exe` to your PATH. Either by adding the instal directory to the
   path, or by creating a Symlink from the exe to a directory that is already in
   the PATH
3. [Set up the Cleaner Tool](#cleaner-tool)

### MacOS

1. add the `tproj` exe to your PATH
2. run `tproj -i` to initialize the config file
3. open `tproj.toml` and set `tempprojectdir` to the directory that sould hold
   your temporary projects

# How to use

## Without Terminal

click the `tproj` exe

## CLI Tool

```sh
tproj # this will create a folder with the current date and time as the name
```

### Custom Name

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

### Flags

to open an explorer window instead of a new terminal window use

```pwsh
tproj -e
```

to open both a terminal and an explorer window use

```pwsh
tproj -b
```

to open the latest create project

```pws
tproj -l
```

to clean up all folders without contents in their `out` folder

```pws
tproj -c
```

to initialize the config file and (on MAcOS) register a launchd agent to run the
cleaning program every week

```pws
tproj -i
```

## Cleaner Tool

### Windows

1. Open Windows task scheduler
2. Create a new task
3. Set the trigger to a schedule you like (like every week or so)
4. Set the action to "execute program"
   1. set the path to the path of the `tproj` executable with the argument `-c`

### MacOS

#### Automatic way

1. run `tproj -i` this registeres a launchd agent to run the executabel with the
   `-c` argument every week

#### Manual Way

1. downlaod the `/macos/heimsoft.tproj.plist` file from the repo
2. replace the {{exe_location}} with the path to the executable
3. paste the `.plist` filte to `~/Library/LaunchAgents/`
4. register the agent using the following command
   `launchctl load ~/Library/LaunchAgents/heimsoft.tproj.plist`
5. you can test if that worked using the following commands
6. find out your id

```bash
id -u
```

2. start the agent manually

```bash
launchctl kickstart gui/{{YOUR_ID}}/heimsoft.tproj
```
