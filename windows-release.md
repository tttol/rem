---
title: Windows Installer
sidebar:
  order: 1
i18nReady: true
---

import CommandTabs from '@components/CommandTabs.astro';
import { Tabs, TabItem } from '@astrojs/starlight/components';

Tauri applications for Windows are either distributed as Microsoft Installers (`.msi` files) using the [WiX Toolset v3]
or as setup executables (`-setup.exe` files) using [NSIS].

Please note that `.msi` installers can **only be created on Windows** as WiX can only run on Windows systems.
Cross-compilation for NSIS installers is shown below.

This guide provides information about available customization options for the installer.

## Building

To build and bundle your app into a Windows installer you can use the Tauri CLI and run the `tauri build` command in a Windows computer:

<CommandTabs
  npm="npm run tauri build"
  yarn="yarn tauri build"
  pnpm="pnpm tauri build"
  deno="deno task tauri build"
  bun="bun tauri build"
  cargo="cargo tauri build"
/>

:::note[VBSCRIPT requirement for MSI packages]

Building MSI packages (`"targets": "msi"` or `"targets": "all"` in `tauri.conf.json`) requires the VBSCRIPT optional feature to be enabled on Windows. This feature is enabled by default on most Windows installations, but if you encounter errors like `failed to run light.exe`, you may need to enable it manually through **Settings** → **Apps** → **Optional features** → **More Windows features**. See the [Prerequisites guide](/start/prerequisites/#vbscript-for-msi-installers) for detailed instructions.

:::

### Build Windows apps on Linux and macOS

Cross compiling Windows apps on Linux and macOS hosts is possible with caveats when using [NSIS].
It is not as straight forward as compiling on Windows directly and is not tested as much.
Therefore it should only be used as a last resort if local VMs or CI solutions like GitHub Actions don't work for you.

:::note

Signing cross compiled Windows installers requires an external signing tool.
See the [signing documentation] for more information.

:::

Since Tauri officially only supports the MSVC Windows target, the setup is a bit more involved.

#### Install NSIS

<Tabs syncKey="OS">
<TabItem label="Linux">
Some Linux distributions have NSIS available in their repositories, for example on Ubuntu you can install NSIS by running this command:

```sh title=Ubuntu
sudo apt install nsis
```

But on many other distributions you have to compile NSIS yourself or download Stubs and Plugins manually that weren't included in the distro's binary package.
Fedora for example only provides the binary but not the Stubs and Plugins:

```sh title=Fedora
sudo dnf in mingw64-nsis
wget https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip
unzip nsis-3.zip
sudo cp nsis-3.08/Stubs/* /usr/share/nsis/Stubs/
sudo cp -r nsis-3.08/Plugins/** /usr/share/nsis/Plugins/
```

</TabItem>

<TabItem label="macOS">
On macOS you will need [Homebrew] to install NSIS:

```sh title=macOS
brew install nsis
```

</TabItem>
</Tabs>

#### Install LLVM and the LLD Linker

Since the default Microsoft linker only works on Windows we will also need to install a new linker.
To compile the Windows Resource file which is used for setting the app icon among other things
we will also need the `llvm-rc` binary which is part of the LLVM project.

<Tabs syncKey="OS">
<TabItem label="Linux">
  
```sh title="Ubuntu"
sudo apt install lld llvm
```

On Linux you also need to install the `clang` package if you added dependencies that compile C/C++ dependencies as part of their build scripts.
Default Tauri apps should not require this.

</TabItem>
<TabItem label="macOS">
```sh title=macOS
brew install llvm
```

On macOS you also have to add `/opt/homebrew/opt/llvm/bin` to your `$PATH` as suggested in the install output.

</TabItem>
</Tabs>

#### Install the Windows Rust target

Assuming you're building for 64-bit Windows systems:

```sh
rustup target add x86_64-pc-windows-msvc
```

#### Install `cargo-xwin`

Instead of setting the Windows SDKs up manually we will use [`cargo-xwin`] as Tauri's "runner":

```sh
cargo install --locked cargo-xwin
```

By default `cargo-xwin` will download the Windows SDKs into a project-local folder.
If you have multiple projects and want to share those files you can set the `XWIN_CACHE_DIR` environment variable with a path to the preferred location.

#### Building the App

Now it should be as simple as adding the runner and target to the `tauri build` command:

<CommandTabs
  npm="npm run tauri build -- --runner cargo-xwin --target x86_64-pc-windows-msvc"
  yarn="yarn tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc"
  pnpm="pnpm tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc"
  deno="deno task tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc"
  bun="bun tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc"
  cargo="cargo tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc"
/>

The build output will then be in `target/x86_64-pc-windows-msvc/release/bundle/nsis/`.

### Building for 32-bit or ARM

The Tauri CLI compiles your executable using your machine's architecture by default.
Assuming that you're developing on a 64-bit machine, the CLI will produce 64-bit applications.

If you need to support **32-bit** machines, you can compile your application with a **different** [Rust target][platform support]
using the `--target` flag:

<CommandTabs
  npm="npm run tauri build -- --target i686-pc-windows-msvc"
  yarn="yarn tauri build --target i686-pc-windows-msvc"
  pnpm="pnpm tauri build --target i686-pc-windows-msvc"
  deno="deno task tauri build --target i686-pc-windows-msvc"
  bun="bun tauri build --target i686-pc-windows-msvc"
  cargo="cargo tauri build --target i686-pc-windows-msvc"
/>

By default, Rust only installs toolchains for your machine's target,
so you need to install the 32-bit Windows toolchain first: `rustup target add i686-pc-windows-msvc`.

If you need to build for **ARM64** you first need to install additional build tools.
To do this, open `Visual Studio Installer`, click on "Modify", and in the "Individual Components" tab install the "C++ ARM64 build tools".
At the time of writing, the exact name in VS2022 is `MSVC v143 - VS 2022 C++ ARM64 build tools (Latest)`.  
Now you can add the rust target with `rustup target add aarch64-pc-windows-msvc` and then use the above-mentioned method to compile your app:

<CommandTabs
  npm="npm run tauri build -- --target aarch64-pc-windows-msvc"
  yarn="yarn tauri build --target aarch64-pc-windows-msvc"
  pnpm="pnpm tauri build --target aarch64-pc-windows-msvc"
  deno="deno task tauri build --target aarch64-pc-windows-msvc"
  bun="bun tauri build --target aarch64-pc-windows-msvc"
  cargo="cargo tauri build --target aarch64-pc-windows-msvc"
/>

:::info

Note that the NSIS installer itself will still be x86 running on the ARM machine via emulation. The app itself will be a native ARM64 binary.

:::

