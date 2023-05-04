
# the *con*figuration *man*ager
conman is a tool for managing linux dotfiles. With a focus on providing a program that can provide the following functionality:

[ ] creating a repository of configuration files
- allowing the store of mutliple configurations for a single program
- allowing the switch between different configurations easily
- backing up that repository with multiple format options
- version control of individual configurations
- importing individual or groups of configs from other repositories
- encryption of piticular files before a export
- facilate custom user hook scripts to be deployed on certain actions

overall, the goal is to make your dotfiles as portable as you are.

## When is conman applicable
conman is designed to be used in a variety of situations. Ultimately is was designed with the idea to manage "configurations"
which i define as any change in an computer systems enviroment that a piece of software may use to change its own behavior. 
This encompses a wide variety of different things and situations, from simple files to enviroment variables. I eventually plan
for conman to allow it all. The most simple use case for conman is people who have any dotfiles at all, allowing them to backup 
their dotfiles and put them in easy to reach locations for importing on another machine. However, equally a person could use this 
software to switch between two different computer activities that require the same software configured differently, any use case 
for changing a piece of softwares behavior is a valid one. 

## the structure of management
conman manages files by copying them into a "repository" which is a  structured set of folders and files conman uses to store 
the dotfiles as well as any imformation about them. Conman uses a hierachigal management system to view your dotfiles:

1. program 
2. config 
3. configuration files 

every file you add belongs to a "config" which is a group of config files that work together, and every "config" belongs to a program. This 
structure is very natural as when a config file is viewed as a file a program uses to change its behavior it must belong to a program, overall 
this is a very organised and structured system. Each "config" has multiple files and so to can each "program" have multiple configurations.

## 

## installation

**technical to do**



