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

To get started with DOCKINS, install it from github page;

Once Dockins is insatlled, you can graphically initialize a new project by running the following command :

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

### Contributing

DOCKINS is an open source project and contributions are welcome. We will be glad to receive any help, because our skills are very poor.
[GitHub repo](https://github.com/fywws/Dockins)

