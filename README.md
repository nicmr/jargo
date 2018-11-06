# Jargo

A simple java build and dependency manager, following the philosophy of cargo and npm.


## 1. Features

#### 1.1 Creating a new java project
```bash
$ ~/code %    jargo --new foo
> successfully created project 'foo'
```

#### 1.2 Checking the current directory for a valid project file

```bash
$ ~/code/foo %    jargo --check
> Valid jargo.toml found at "~/code/foo/jargo.toml"
> Project name: foo
```

#### 1.3 Building a project

```bash
$ ~/code/foo %    jargo --build
> target: "~/home/code/foo/target"
> Compiling Main.java ...

$ ~/code/foo %  ls ./target
> Main.class

```

#### 1.4 Running a project
Main.java contents:
```java
class Main{
    public static void main(String[] args){
        System.out.println("Hello world");
    }
}
```
--
```bash
$ ~/code/foo %    jargo --run
> Hello, world!
```
Note: Run will automatically build the project, there is no need to use more than one command for a single tasks


## 2. Project structure
```
~/code/foo
    src/
        -- Main.java 
        # source files go here
    target/
        --  Main.class
        # compiled .class files go here
    jargo.toml
```




## 3. Planned features

- Better support for (sub)packages
- Support for dependencies (.jars) and file resources (images, config files..)
- Track source files: Only rebuild source files if they were changed
- Change project config from the command line
- Pass different entry point class to --run
- Different verbosity levels
- JUnit support