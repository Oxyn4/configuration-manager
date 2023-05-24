
# The *con*figuration *man*ager
conman is a tool for managing Linux dotfiles. With a focus on providing a program that can provide the following functionality:

- [x] creating a repository of configuration files
- [x] allowing the store of multiple configurations for a single program
- [x] listing contents of repository
- [x] removing programs, configs and files from a repository
- [ ] allowing the switch between different configurations easily
- [ ] backing up that repository with multiple format options
- [ ] version control of individual configurations
- [ ] importing individual or groups of configs from other repositories
- [ ] encryption of particular files before an export
- [ ] facilate custom user hook scripts to be deployed on certain actions

Overall, the goal is to make your dotfiles as portable as you are.

## When is conman applicable
conman is designed with the idea to manage "program configurations" which i define as any change in an computer systems environment that 
a piece of software may use to change its own behaviour.  This encompasses a variety of different things, from files to environment 
variables. I eventually plan for conman to allow it all. The simplest use case for conman is people who have any dotfiles at all, 
allowing them to back up their dotfiles and put them in easy to reach locations for importing to another machine. However, equally a 
person could use this software to switch between two different computer activities that require the same software configured differently, 
any use case for changing a piece of software behaviour is a valid one.
## The structure of management
conman manages files by copying them into a "repository" which is a structured set of folders and files conman uses to store 
the dotfiles as well as any information about them. Conman uses a hierarchical management system to view your dotfiles:

1. program 
2. config 
3. configuration files 

Every file you add belongs to a "config" which is a group of config files that work together, and every "config" belongs to a program. This 
structure is very natural as when a config file is viewed as a file a program uses to change its behavior it must belong to a program, overall 
this is a very organized and structured system. Each "config" has multiple files and so to can each "program" have multiple configurations.

# Usage

## Two different modes
conman runs in two different modes. Portable and Installed. On startup conman analyses its own enviroment to determine which mode to run in,
it looks at where its executable is placed. The mode it runs in doesnt detemine much other than log file locations and where conman looks for its
configuration. The goal of the portable mode is to have the smallest footprint on the machines file system as possible and only modify and move
files needed for conmans main goal. 

## installing conman 
to install conman simply clone the repository:
```sh 
git clone https://github.com/Oxyn4/configuration-manager
```
then install with cargo:
```sh 
cd configuration-manager && cargo install --path . 
```
this will put the cargo binary under ~/.cargo/bin/cm you can then verify installation:
```sh 
cm -v 
```
this will print the version.
## repository manipulation and inspection
## inspecting a repository 
with conman installed we can now begin to play with its functionality. Conman automatically creates us a repository, 
we can inspect it with the "ls" command which lists a repositories contents.
```sh 
cm ls 
```
this shows its programs and their configurations and tracked files. it is really useful for visuallising current
state of your configurations.
### add command 
the add command is used to add a program, config or file to a repository:
```sh 
cm add neovim
```
this adds a program called neovim to the repository. we can add a configuration for neovim:
```sh 
cm add neovim blue_colorscheme
```
here we have added a configuration called blue_colorscheme. finally lets add a file:
```sh 
cm add neovim blue_colorscheme init.lua
```
now we have a file in our config. You can follow this pattern to add more programs, configs and files.
you can also add multiple files at once:
```sh 
cm add neovim blue_colorscheme file.lua file2.lua 
```








