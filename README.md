# Cinny desktop (Telegram Edition fork)

Cinny is a matrix client focusing primarily on simple, elegant and secure interface. The desktop app is made with Tauri.

This repository is a fork of [cinnyapp/cinny-desktop](https://github.com/cinnyapp/cinny-desktop) with the
Cinny Telegram Edition modifications. The web frontend is **not** a symlink here — it is a git submodule
pointing at [Novusbot/cinny](https://github.com/Novusbot/cinny) and pinned to a fork tag (`vX.Y.Z-tg.N`).
That is what lets CI check out the repository and build it.

## Getting a build

Every push to `dev` builds a Windows installer automatically. Nothing to configure, nothing to click:

* Push your changes to `dev`.
* Open the **Actions** tab, take the latest **Windows build** run.
* Scroll to **Artifacts** at the bottom and download `cinny-windows`.
* Unzip it — inside are `Cinny_desktop-x86_64.msi` and `Cinny_<version>_x64-setup.exe`. Send either one
  to your colleagues.

Artifacts are kept for 30 days. After that, re-run the workflow from the Actions tab (*Run workflow*)
to build again.

Installers are **not code-signed**, so Windows SmartScreen will warn on first run. Choose
*More info* → *Run anyway*.

There is no auto-updater in the fork: `tauri-plugin-updater` and its release signing keys were removed.
To give colleagues a new version, push to `dev` and send them the new installer.

## Local development

Firstly, to setup Rust, NodeJS and build tools follow [Tauri documentation](https://v2.tauri.app/start/prerequisites/).

Now, to setup development locally run the following commands:
* `git clone --recursive https://github.com/Novusbot/cinny-desktop.git`
* `cd cinny-desktop/cinny`
* `npm ci`
* `cd ..`
* `npm ci`

If you already cloned without `--recursive`, run `git submodule update --init --recursive`.

To build the app locally, run:
* `npm run tauri build`

To start local dev server, run:
* `npm run tauri dev`

### Bumping a release

The web frontend version defines the release version. Keep it in sync in three places:
`cinny/package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`.

Then tag the web repo and pin the submodule to it, otherwise CI builds an arbitrary revision:
```bash
cd cinny && git tag -a v4.12.8-tg.1 -m "..." && git push origin v4.12.8-tg.1
cd .. && git -C cinny fetch --tags && git -C cinny checkout v4.12.8-tg.1
git add cinny && git commit -m "chore: pin cinny submodule to v4.12.8-tg.1"
```

> `npm run tauri` copies this repository's `config.json` into `cinny/` before building. If you edit
> `config.json` here, the submodule working tree becomes dirty — that is expected, do not commit it into `cinny/`.

