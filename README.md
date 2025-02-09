# DOSUKOI

```
██████╗  ██████╗ ███████╗██╗   ██╗██╗  ██╗ ██████╗  ██████╗
██╔══██╗██╔═══██╗██╔════╝██║   ██║██║ ██╔╝██╔═══██╗  ╚██╔╝
██║  ██║██║   ██║███████╗██║   ██║█████╔╝ ██║   ██║   ██║  
██║  ██║██║   ██║╚════██║██║   ██║██╔═██╗ ██║   ██║   ██║  
██████╔╝╚██████╔╝███████║╚██████╔╝██║  ██╗╚███ ███╔╝██████║  
╚═════╝  ╚═════╝ ╚══════╝ ╚═════╝ ╚═╝  ╚═╝ ╚═══╝╚═╝   ╚═╝  

   (╯°□°）╯︵ ┻━┻  
```

## About DOSUKOI
DOSUKOI is a simple CLI tool to stop Docker containers efficiently. It allows you to stop all running containers at once or stop containers from a specific Docker Compose project.

## Installation
You can install DOSUKOI using Homebrew:

```sh
brew tap Akito-n/dosukoi
brew install dosukoi
```

## Usage
To stop all running Docker containers:
```sh
dosukoi
```

To stop a specific Docker Compose project:
```sh
dosukoi my_project
```

To list all running Docker containers and select which ones to stop/kill:
```sh
dosukoi --l
```
- This will show a selectable list of running containers.
- Use arrow keys and space to select containers.
- Press enter to confirm the selection and stop/kill them.

To list all running Docker containers and kill selected ones instead of stopping them:
```sh
dosukoi --l -k
```

## Uninstallation
If you want to remove DOSUKOI:
```sh
brew uninstall dosukoi
```

## License
MIT License

## Contributing
Feel free to open issues or pull requests to improve DOSUKOI!

---

🚀 **Stop your containers with style! DOSUKOI!**

