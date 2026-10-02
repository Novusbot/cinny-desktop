# Cinny desktop (Telegram Edition fork)

Cinny is a matrix client focusing primarily on simple, elegant and secure interface. The desktop app is made with Tauri.

This repository is a fork of [cinnyapp/cinny-desktop](https://github.com/cinnyapp/cinny-desktop) with the
Cinny Telegram Edition modifications. The web frontend is **not** a symlink here — it is a git submodule
pointing at [Novusbot/cinny](https://github.com/Novusbot/cinny), pinned to an exact commit recorded in this
repository — either the current `dev` commit (monthly cycle) or a fork tag `vX.Y.Z-tg.N` (releases).
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

## Monthly cycle: shipping a Windows build to colleagues

This is the routine path — no version bump, no tag. Use it every time the web frontend gets new work.

1. Merge and push your web changes to `dev` in [Novusbot/cinny](https://github.com/Novusbot/cinny).
2. Pin the submodule to that new commit:
   ```bash
   cd cinny && git fetch origin && git checkout origin/dev && cd ..
   git add cinny && git commit -m "chore: pin cinny submodule to current dev"
   ```
3. Push the desktop repo: `git push origin dev`. If you touched `.github/workflows/*`, also
   `git push origin dev:main` — `main` is the default branch, and GitHub only offers **Run workflow**
   for workflows that exist there.
4. Wait for the **Windows build** run (~11 minutes), then download the `cinny-windows` artifact from the
   **Actions** tab.
5. Send the installer to your colleagues.

> **Step 2 is not optional.** The desktop app is built from exactly the submodule commit recorded in this
> repository — not from the latest `dev` of the web repo. If you push web changes and forget to pin them,
> colleagues get a Windows build with the old frontend and no error anywhere.

## Bumping a release

A release adds a version number and a tag on top of the monthly cycle above.

The web frontend version defines the release version. Keep it in sync in three places:
`cinny/package.json`, `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`.

Then tag the web repo and pin the submodule to it, otherwise CI builds an arbitrary revision:
```bash
cd cinny && git tag -a v4.12.8-tg.1 -m "..." && git push origin v4.12.8-tg.1
cd .. && git -C cinny fetch --tags && git -C cinny checkout v4.12.8-tg.1
git add cinny && git commit -m "chore: pin cinny submodule to v4.12.8-tg.1"
```
Push the desktop repo (`git push origin dev`) and take the resulting `cinny-windows` artifact as described
in the monthly cycle.

Installers stay **not code-signed**, and there is **no auto-updater** in the fork: to give colleagues a new
version, push to `dev` and send them the new installer.

> `npm run tauri` copies this repository's `config.json` into `cinny/` before building. If you edit
> `config.json` here, the submodule working tree becomes dirty — that is expected, do not commit it into `cinny/`.
