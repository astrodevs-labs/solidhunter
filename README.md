
This is an open source project for linting [Solidity](http://solidity.readthedocs.io/en/develop/) code. This project
provides **Style Guide** validations.

## Installation

Download the last release from [here](https://github.com/astrodevs-labs/solidhunter/releases).

## Usage

First initialize a configuration file, if you don't have one:

```sh
solidhunter --init
```

This will create a `.solidhunter.json` file with the default rules enabled. 

Run `solidhunter` without arguments to get more information:

```text
Usage: solidhunter.exe [OPTIONS]

Options:
  -p, --path <PROJECT_PATH>    Specify project path [default: .]
  -e, --exclude <IGNORE_PATH>  Exclude part of the project path
  -r, --rules <RULES_FILE>     Specify rules file [default: .solidhunter.json]
  -v, --verbose                Verbose output
  -i, --init                   Initialize rules file
  -h, --help                   Print help information
  -V, --version                Print version information
```

## Configuration

You can use a `.solidhunter.json` file to configure Solidhunter for the whole project.

This file has the following
format:


```json
{
  "name": "solidhunter",
  "includes": [],
  "plugins": [],
  "rules": [
    {
      "id": "line-max-len",
      "severity": 2,
      "data": [
        "80"
      ]
    },
    {
      "id": "code-complexity",
      "severity": 2,
      "data": [
        "7"
      ]
    },
    {
      "id": "quotes",
      "severity": 1,
      "data": []
    }
  ]
}
```
A full list of all supported rules can be found [here](https://github.com/astrodevs-labs/solidhunter/tree/main/solidhunter-lib/src/rules).


You can disable a rule by simply removing the entry in the file.

## IDE Integrations

  - **[Visual Studio Extention](https://github.com/astrodevs-labs/solidhunter-vscode)**
