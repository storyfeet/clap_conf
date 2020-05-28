Clap Conf
=========

A library for Handling Configuration files, Command Line arguments, and Environment variables together.

Purpose
-------

There are three main ways programs get operation parameters when initializing. 

* Command Line Arguments.
* Configuration files.
* Environment Variables.

If you only want command line arguments Clap is an awesome project, and helps your command line program give such a great and clear user experience.

However if you want your program to handle all three of these you have your work cut out for you.

Each of these behave in different ways but often you want to be able to work with one of these but fall back on the others if the first is not supplied.

For example. Assuming you have built a clap matches object

```rust
//without clap conf but with clap.

//This code ignores the fact that env returns String,(and has to)
//but most config files and clap returns &str Which makes it even more tricky to handle
let filename = clap_matches.value_of("filename").unwrap_or(
    config.get("filename").unwrap_or(
        std::env::var("PROG_FILENAME").unwrap_or("")
    )
);
```

clap\_conf provides a wrapper that handles this so that results can be collected using a builder like pattern.

```rust
use clap_conf::prelude::*;

//once at the top of the file.
let cfg = with_toml_env(&clap_matches,&["priority/config/location","another/possible/location"]);

//something like this for every item.
let filename = cfg.grab().arg("filename").conf("filename").env("PROG_FILENAME").def("None");

//Or if you do not want to use a default and instead return an option.
let filename = cfg.grab().arg("filename").conf("filename").env("PROG_FILENAME").done();

```

clap\_conf can also handle nested structures in your toml file.

```rust
let nested = cfg.grab().conf("a.b.nested_property").done();

```

Combining typed values was always going to be tricky, as the CLI and clap expect you to parse the result as you wish, but Toml does this for you. So for non string properties, it converts them back to string again.

It's not ideal, but it's better than nothing.



Changes
---------

## 0.1.5
Now uses std::error::Error;


## 1.4

Added a new method to ConfError called ```add_info(self, &str)->Self```
So you can lazily add info to the message
Added ```.req()``` to LocalGrabber;

Addded a MultiGrabber so getting arrays out is much simpler:
use ```grab_multi()``` like you would use ```grab()``` or ```grab local()```

## 1.3

Added .req(), and ```ask_default()``` to the Grabber. 


## 1.1

Added Localizer and now wraps toml Value in localizer for with\_toml\_env 
Added Local Grabber so ```grab_local()``` should return a path, local to the config file selected
