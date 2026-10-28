# Flatpak

`com.anyplaynet.Launcher.yml` packs the launcher as a Flatpak. It compiles nothing: it installs the
release binary the release workflow already built, so the Flatpak runs the same code as the
AppImage. The release workflow's `flatpak` job builds it and adds
`AnyPlay-Launcher-<version>-x86_64.flatpak` to the release. The file is not part of the signed
update manifest; Flatpak users update through Flatpak.

## Build locally

On Linux with `flatpak` and `flatpak-builder` installed, from the repository root:

    tools/linux/flatpak.sh <version> <launcher binary> <THIRD-PARTY-LICENSES.txt> <output folder>
    flatpak install --user <output folder>/AnyPlay-Launcher-<version>-x86_64.flatpak
    flatpak run com.anyplaynet.Launcher

The runtime and SDK (`org.freedesktop.Platform` and `Sdk` 26.08) come from Flathub.

## Permissions

| Permission | Why |
|---|---|
| `--share=network` | Sign-in, the AnyPlay network, downloads and updates of AnyPS5. |
| `--share=ipc` | Shared memory for X11. |
| `--socket=x11`, `--socket=wayland` | The window, on either display server. |
| `--socket=pulseaudio` | Sound (also PipeWire's PulseAudio socket). |
| `--device=all` | Game pads, `/dev/uinput` for the virtual controller, and the GPU (`dri`). |
| `--filesystem=home` | Games live in the home folder and the launcher starts them from there. |
| `--talk-name=org.freedesktop.Notifications` | Desktop notifications. |

The icon is 64x64 and sits in `hicolor/64x64`. Replace it with a larger one when the artwork exists.
