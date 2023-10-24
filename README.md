# <span style="color:red">*[WIP]- Use at your own risk* |</span> tools (Your modern Microservices Companion!) 

[![Build](https://github.com/xN4P4LM-org/tools/actions/workflows/build.yaml/badge.svg)](https://github.com/xN4P4LM-org/tools/actions/workflows/build.yaml) | [![Test](https://github.com/xN4P4LM-org/tools/actions/workflows/test.yaml/badge.svg?branch=main)](https://github.com/xN4P4LM-org/tools/actions/workflows/test.yaml)

------
## Overview
This project was inspired by all the various shell, python, make, and other scripts needed to orchestrate a microservices and similarly large containerized project. 

The goal of this project is to provide a single customizable tool that can be used to orchestrate the various tasks needed to develop, build, and support your microservices project.

# Table of Contents
- [Overview](#overview)
- [Table of Contents](#table-of-contents)
- [Features](#features)
    - [Current Features](#current-features)
    - [Planned Features](#planned-features)
        - [Nice to have features](#would-be-nice-to-have-features)
- [Requirements](#requirements)
    - [Required software to run these tools](#required-software-to-run-these-tools)
        - [Optional software to run these tools](#optional-software-to-run-these-tools)
    - [Required software to develop this project](#required-software-to-develop-this-project)
- [Usage](#usage)
- [Configuration](#configuration)
- [Development](#development)

# Current Features
- Initialize a project
    - Currently only supports getting the submodule for an existing project

## Planned Features
- Initializing a project:
    - Support to create a new project from scratch
    - Encourage the use of a template repository to ensure consistency across projects
- Service management:
    - Support to create a new service in a project
    - Encourage the use of a template repository to ensure consistency across projects
    - Encourage the creation of a new template repository if one does not exist
- Development environment:
    - Manage bringing up and down a local development environment using docker-compose

### Would be nice to have features
- Development environment:
    - Manage bringing up and down a local development environment using minikube or similar
- In-situ development environment:
    - Manage the creation of a development environment using terraform or similar
    - Manage the configuration of a development environment using ansible or similar
    - Manage the deployment of a development environment using helm or similar

# Requirements

### Required software to run these tools:
- **Git** with support for `git submodule`
- **Docker** or compatible container enging with support for `Docker Compose`

#### Optional software to run these tools:
- **GitHub CLI** - `gh` - can be avoided by providing a GitHub token in the config.yaml file
    - Used to enable the following functionality:
        - Repository:
            - Create a repository using a template - [GitHub API Docs](https://docs.github.com/en/rest/reference/repos#create-a-repository-using-a-template)
            - Create a repository from scratch - [GitHub API Docs](https://docs.github.com/en/rest/reference/repos#create-a-repository-for-the-authenticated-user)

if using a token, it must have the following permissions:
- `repo: read` - if creating a repository from a template
- `repo: read and write` - if creating a repository from scratch

# Usage
```bash
tools [FLAGS] [SUBCOMMAND] [OPTIONS]
```


## Required software to develop this project:
- **Rust** - `cargo` - [Rustup](https://rustup.rs/)
- **Git** with support for `git submodule` - [Git](https://git-scm.com/downloads)
- **Docker** or compatible container enging with support for `Docker Compose` - [Docker](https://docs.docker.com/get-docker/)