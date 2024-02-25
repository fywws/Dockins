# DOCKINS

> [!WARNING]
> This is all under development!

### A util for auto-generating docker files and images

Dockins is a versatile utility designed to automate the generation of Dockerfiles and Docker images. This tool simplifies the process of creating Docker configurations by streamlining repetitive tasks, saving time, and ensuring consistency across projects.

### Key features:

- Automatic Dockerfile generation
- .config files support
- Support for multiple languages and frameworks
- Dependency management
- Environment variable support
- Easy to use

### Getting Started

To get started with DOCKINS, install it from GitHub page;

Once Dockins is installed, you can graphically initialize a new project by running the following command :

```
dockn init
```

You may also use .config files such as .toml, .properties :

```
dockn init --config dockn.toml
```
> [!NOTE]
> Name of config file may be different



Example of dockn.toml :

```
language=python // go, nodejs, rust
make-script=true // false
port=8080
db= // dont write it or leave blank, if you dont have DataBase
```

# Dockins-YML

### A util for auto-generation of docker compose file

Dockins yml is user-friendly CLI that allows you to have flexible configuration.

To create you compose file use

```
dockny init 
```

To create your config you should use 

```
dockny config
```

To see supported frontend technologies you should use 

```
dockny fl
``` 

To see supported backend technologies you should use

```
dockny bl
```

To see supported server (host) technologies you should use

```
dockny sl
```

And finally to see supported database technologies you should use

```
dockny dbl
```

> [!WARNING]
> Keep in mind that everything is still under development and will be improved


You may also make an env variable via terminal in :

Linux
```
export PATH=$PATH:dockn || dockny
```

Windows
```
set PATH=%PATH%;dockn.exe || dockny.exe
```


### Contributing

DOCKINS is an open source project and contributions are welcome. We will be glad to receive any help, because our skills are very poor.
[GitHub repo](https://github.com/fywws/Dockins)

