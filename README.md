# Docker ArmA 3 Server

[![Build project](https://github.com/PurplProto/docker-arma3-server/actions/workflows/build.yml/badge.svg)](https://github.com/PurplProto/docker-arma3-server/actions/workflows/build.yml)

A flexible ArmA 3 dedicated server in Docker.

## Features

- Can install additional packages for mods that require them (including i386 packages built for 32bit machines)
- Running mods from the ArmA 3 install directory
- Updating ArmA 3 on launch
- Supports Steam accounts protected by Steam Guard (note: a separate account is recommended and is not required to own ArmA 3 to download the server software)

## Environment variables

| Name                   | Default value | Description                                                                                                                      |
| ---------------------- | ------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| EXTRA_PACKAGES         |               | Additional packages to install, i.e `libtbb2:i386 zlib1g:i386` if you want to use [extDB3](https://github.com/SteezCram/extDB3). |
| MODS_RUN_FROM_ARMA_DIR | true          | Copy the mods to the same folder the ArmA binary lives in. Useful for some mods that don't load when placed within a sub folder. |
| MODS_SHOULD_LOAD       | true          | Should instruct the ArmA 3 server to load the mods.                                                                              |
| ARMA_BINARY            | arma3server   | The ArmA 3 binary to use, i.e. `arma3server_x64` if you want to use the 64bit one.                                               |
| ARMA_CONFIG            | main.cfg      | Your server config file name.                                                                                                    |
| ARMA_EXTRA_PARAMS      |               | Extra parameters to pass when launching the ArmA 3 server.                                                                       |
| ARMA_PROFILE           | main          | The profile name. Stored in `/arma3/configs/profiles`                                                                            |
| ARMA_WORLD             | empty         | The world to load on launch.                                                                                                     |
| ARMA_LIMITFPS          | 1000          | Server FPS limit.                                                                                                                |
| ARMA_HEADLESS_CLIENTS  | 0             | Also launch this many headless clients.                                                                                          |
| STEAM_BRANCH           | public        | The branch Steam should use for downloading the ArmA 3 server.                                                                   |
| STEAM_BRANCH_PASSWORD  |               | The branch password.                                                                                                             |
| STEAM_USER             |               | The username for SteamCMD to use.                                                                                                |
| STEAM_PASSWORD         |               | The password for SteamCMD to use.                                                                                                |
| STEAM_GUARD_CODE       |               | The Steam guard code for SteamCMD to use.                                                                                        |
| STEAM_SKIP_INSTALL     | false         | Don't install/update the game.                                                                                                   |

Please note:

- The Steam Guard code may expire before the server gets to use it on the first boot of the container due to the initial install of SteamCMD taking up a bit of time.
- It is also recommended to use a separate account for dedicated servers.
- It is *not* required for the Steam account to own ArmA 3 to download the server software.
