[![Build Status](https://travis-ci.com/georgehipp/sysinfo_report.svg?branch=master)](https://travis-ci.com/georgehipp/sysinfo_report)

# SysInfo Report

Cross Platform CLI Tool to display System Information

## Getting Started

These instructions will get a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

[Rust Toolchain](https://www.rust-lang.org/tools/install) 
```
$ rustup show
Default host: x86_64-unknown-linux-gnu

stable-x86_64-unknown-linux-gnu (default)
rustc 1.35.0 (3c235d560 2019-05-20)
```
[git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) 
Example:
```
$ git --version
git version 2.17.1
```
An IDE to assist development.
I prefer [VSCode](https://code.visualstudio.com/)
with [Rust(rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

[rustfmt](https://github.com/rust-lang/rustfmt) to keep the files clean
```
rustup component add rustfmt --toolchain stable-x86_64-unknown-linux-gnu
```

### Installing

To start development:

Clone this repository into your local workspace enter the folder

```
$ git clone https://github.com/georgehipp/sysinfo_report.git
$ cd sysinfo-report
```

Validate Code

```
$ cargo build
   Compiling sysinfo_report v0.2.0 (/home/george/Projects/rust/sysinfo_report)
    Finished dev [unoptimized + debuginfo] target(s) in 1.18s
```

Try it out on your system

```
$ ./target/debug/sysinfo_report os
{"Version:": "18.04", "Type:": "Ubuntu"}
$ cargo run -- os
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/sysinfo_report os`
{"Type:": "Ubuntu", "Version:": "18.04"}
```

## Running the tests

Testing is currently only at the integration level, unit tests have not been implemented yet.

### Break down into end to end tests

Integration Testing

```
$ cargo test
...
test integration::cpu_test ... ok
test integration::os_test ... ok
test integration::processes_test ... ok
test integration::components_test ... ok
test integration::invalid_option_test ... ok
test integration::disk_test ... ok
test integration::network_test ... ok
test integration::memory_test ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### And coding style tests

Use rustfmt to ensure code is following standard formating

```
$ cargo fmt
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Dropwizard](http://www.dropwizard.io/1.0.2/docs/) - The web framework used
* [Maven](https://maven.apache.org/) - Dependency Management
* [ROME](https://rometools.github.io/rome/) - Used to generate RSS Feeds

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/georgehipp/sysinfo_report/tags). 

## Authors

See also the list of [contributors](https://github.com/georgehipp/sysinfo_report/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Hat tip to anyone whose code was used
* Inspiration
* etc


