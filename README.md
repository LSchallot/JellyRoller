# JellyRoller - The CLI Jellyfin Controller Utility for Linux and Windows

JellyRoller is an open source CLI Jellyfin Controller written in Rust that works on Windows and Linux. Its primary purpose is to allow administration of a Jellyfin application from the command line.

## How it works
On the first execution, JellyRoller prompts for information to authenticate as an admin user.  Once this authentication has succeeded, an API key is created and stored within the JellyrRoller configuration.  JellyRoller then uses the Jellyfin API to manage the server.

Any previous user auth tokens will be converted to an API key upon next execution when upgrading from JellyRoller < 0.3.

## Usage Information

```
A CLI controller for managing Jellyfin

Usage: jellyroller <COMMAND>

Commands:
  add-user                     Creates a new user
  add-users                    Uses the supplied file to mass create new users
  apply-backup                 Applies the specified backup
  completions                  Generate Shell completions
  create-backup                Creates a new backup (metadata, trickplay, subtitles, database)
  create-report                Creates a report of either activity or available movie items
  delete-user                  Deletes an existing user
  disable-user                 Disable a user
  enable-user                  Enable a user
  execute-task-by-name         Executes a scheduled task by name
  generate-report              Generate a report for an issue
  get-backups                  Get a list of current backups
  get-devices                  Show all devices
  get-libraries                Gets the libraries available to the configured user
  get-packages                 Lists all available packages
  get-plugins                  Returns a list of installed plugins
  get-repositories             Lists all current repositories
  get-scheduled-tasks          Show all scheduled tasks and their status
  grant-admin                  Grants the specified user admin rights
  initialize                   Perform a silent initialization
  install-package              Installs the specified package
  list-logs                    Displays the available system logs
  list-users                   Lists the current users with basic information
  reconfigure                  Reconfigure the connection information
  register-library             Registers a new library
  register-repository          Registers a new Plugin Repository
  remove-device-by-username    Removes all devices associated with the specified user
  reset-password               Resets a user's password
  revoke-admin                 Revokes admin rights from the specified user
  restart-jellyfin             Restarts Jellyfin
  scan-library                 Start a library scan
  search-media                 Executes a search of your media
  server-info                  Displays the server information
  server-setup                 Setup a new server using a configuration file.
  show-log                     Displays the requested logfile
  shutdown-jellyfin            Shuts down Jellyfin
  update-image-by-id           Updates image of specified file by id
  update-image-by-name         Updates image of specified file by name
  update-metadata              Updates metadata of specified id with metadata provided by specified file
  update-users                 Mass update users in the supplied file
  update-user-profile-picture  Update a user's profile picture
  help                         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version


```

## Installation

**Note:** All installation instructions assume the end-user can handle adding the application to their user's PATH.

### Mac / Linux (Homebrew)
```
brew tap LSchallot/JellyRoller https://github.com/LSchallot/JellyRoller
```
#### (Linux)
```
brew install jellyroller
```
#### (Mac)
```
brew install --build-from-source jellyroller
```
### Windows (Scoop)
```
scoop add bucket jellyroller https://github.com/lschallot/jellyroller.git
scoop update
scoop install jellyroller
```

### Building From Source

Currently built with rustc 1.88.0. If building on a Linux machine, you may need to install openssl-devel.

```
cargo install --git https://github.com/LSchallot/JellyRoller
```

### Initial Configuration

When running JellyRoller for the first time, you will be prompted to configure against your Jellyfin instance. You will be prompted for various items which are described below.
| Prompt | Description |
| ------------- | ------------- |
| Please enter your Jellyfin URL: | The URL to your Jellyfin instance. Depending on your setup, you may need to provide the port. Examples include http://myjellyfin.lab or http://localhost:8096. |
| Please enter your Jellyfin username: | Username with admin rights that JellyRoller will use to execute commands. |
| Please enter your Jellyfin password: | Password associated with the username being used. |

### Custom Configuration
As of 0.5.0, it is possible to keep your configuration file alongside of the JellyRoller executable.  Simply save your configuration in the same directory with the name "jellyroller.config" and it will be used automatically.  Keep in mind that this configurtion file will contain your API key, so secure the file as needed.

### Downloading Release

See Releases for binaries. I can currently supply builds for x86_64 Windows and x86_64 Linux. Please open an issue if you would like to request an additional format.

## Roadmap

Please open issues for feature requests or enhancements.
