# JellyRoller - The CLI Jellyfin Controller Utility for Linux and Windows

JellyRoller is an open source CLI Jellyfin Controller written in Rust that works on Windows and Linux. Its primary purpose is to allow administration of a Jellyfin application from the command line.

## How it works
On the first execution, JellyRoller prompts for information to authenticate as an admin user.  Once this authentication has succeeded, an API key is created and stored within the JellyrRoller configuration.  JellyRoller then uses the Jellyfin API to manage the server.

Any previous user auth tokens will be converted to an API key upon next execution when upgrading from JellyRoller < 0.3.

## Usage Information

```
jellyroller 0.4.0-ALPHA
A CLI controller for managing Jellyfin

USAGE:
    jellyroller.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

Commands:
  add-user                   Creates a new user
  delete-user                Deletes an existing user
  list-users                 Lists the current users with basic information
  reset-password             Resets a user's password
  server-info                Displays the server information
  list-logs                  Displays the available system logs
  show-log                   Displays the requested logfile
  reconfigure                Reconfigure the connection information
  get-devices                Show all active devices
  remove-device-by-username  Removes all devices associated with the specified user
  get-scheduled-tasks        Show all scheduled tasks and their status
  execute-task-by-name       Executes a scheduled task by name
  scan-library               Start a library scan
  disable-user               Disable a user
  enable-user                Enable a user
  grant-admin                Grants the specified user admin rights
  revoke-admin               Revokes admin rights from the specified user
  restart-jellyfin           Restarts Jellyfin
  shutdown-jellyfin          Shuts down Jellyfin
  get-libraries              Gets the libraries available to the configured user
  get-plugins                Returns a list of installed plugins
  add-users                  Uses the supplied file to mass create new users
  update-users               Mass update users in the supplied file
  create-report              Creates a report of either activity or available movie items
  search-media               Executes a search of your media
  update-image-by-name       Updates image of specified file by name
  update-image-by-id         Updates image of specified file by id
  help                       Print this message or the help of the given subcommand(s)

```

## Installation

**Note:** All installation instructions assume the end-user can handle adding the application to their user's PATH.

### Building From Source

Currently built with rustc 1.69.0. If building on a Linux machine, you may need to install openssl-devel.

```
git clone <git location>
cd jellyroller
cargo build
```

### Initial Configuration

When running JellyRoller for the first time, you will be prompted to configure against your Jellyfin instance. You will be prompted for various items which are described below.
| Prompt | Description |
| ------------- | ------------- |
| Please enter your Jellyfin URL: | The URL to your Jellyfin instance. Depending on your setup, you may need to provide the port. Examples include http://myjellyfin.lab or http://localhost:8096. |
| Please enter your Jellyfin username: | Username with admin rights that JellyRoller will use to execute commands. |
| Please enter your Jellyfin password: | Password associated with the username being used. |

### Downloading Release

See Releases for binaries. I can currently supply builds for x86_64 Windows and x86_64 Linux. Please open an issue if you would like to request an additional format.

## Roadmap

Please open issues for feature requests or enhancements.
