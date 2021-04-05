# Optional: run from anywhere
You can run that binary anywhere from the command line

To do this you need to add it to your path

For mac you can add to your path from /etc/paths

whatever editor you prefer.... vi, code, etc...

```sudo code /etc/paths```
I added a path like this, saved and authenticated with a password

/Users/`whoami`/code/rust/binaries
whoami command is surrounded by `

next copy your new binary over to where it needs to be, in that binaries folder

cp whateverYourProgramIsCalled /Users/`whoami`/code/rust/binaries
Then open a new terminal window
Check your command is in your path

```where whateverYourProgramIsCalled```

Run your command from anywhere

```whateverYourProgramIsCalled``` 
