# Appendage
`sudo tee -a` cost me an arm and a leg!

### What is Appendage?

Glad you've asked, `appendage` is a tool for appending text to files. That's all it does.

### How do I use it?

Run `apd <file> <text>`. It's that simple, and if the file doesn't exist, it will be created for you! Neat.

If you need superuser, just run it with sudo, no need for any pipes or any other garbage, just write what you want to write, to the file you want to write to


For the advanced `appendage` users, we also have a few extra flags that do some stuff

| Short |      Long      | Description                                                                                              |
|------:|:--------------:|:---------------------------------------------------------------------------------------------------------|
|  `-q` |   `--quiet`    | Don't print confirmation message to stdout                                                               |
|  `-v` |  `--verbose`   | Prints out debug messages, outlining exactly what it's doing                                             |
|  `-n` | `--no-newline` | Directly append to the file without first writing a newline<br/>This is not a good idea for config files |
 |  `-c` | `--no-create`  | Don't create the file if it doesn't already exist                                                        |

### Exit Codes

`appendage` has many, many ways of terminating, depending on what broke (or didn't!)

| Exit Code | Description                          |
|----------:|:-------------------------------------|
|       `0` | It worked                            |
|       `1` | It didn't                            |
|       `2` | You got the arguments wrong, somehow |
|       `3` | It couldn't create the file          |

As `appendage` is a simple program, this is literally all that can go wrong.

The only possible error can be that it couldn't append to the file, and in that case it'll print out the corresponding OS Error.

Everything else is user error and your fault :)

### Enjoy!