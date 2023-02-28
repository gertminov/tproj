# tproj

This little command line tool helps you to create temporary project folders, for
when you just want to try something out and have it deleted automatically later.
It is my first endever with Rust so it's more of a learning experience than a
real tool

## How to use

```sh
tproj # this will create a folder with the current date and time as the name
```

or:

```sh
tproj myFoldername # the will create a folder named "myFoldername"
```

# Todo

- [ ] make the base folder configurable. currently the base folder is
      `F:/tempProjects`
- [ ] explain how cleaner part works
