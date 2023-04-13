###

<img src="https://3gjaf-uyaaa-aaaal-qbxdq-cai.raw.ic0.app/static/media/logo_large.1eb5ead8b26a8ad5e527.png"
     alt="Served from the canister"
     style="margin-top: 20px; height: 48px; filter: drop-shadow(1px 1px 25px black)" />

##

# File manager canister backend

The code in this repository allows you to spin up a canister on the [internet computer](https://internetcomputer.org) that handles file storage and serving over https.

One of the unique features is that you can deploy websites as you would do in an traditional web2 FTP setting (ex; filezilla). Just by uploading files you can serve a website on a canister that could also handle backend logic.

This [canister](https://3gjaf-uyaaa-aaaal-qbxdq-cai.raw.ic0.app/) serves as an example. If the `index.html` file would be removed, the file manager frontend would not be reachable.

The repository for the file manager frontend can be found [here](https://github.com/rem-code-s/canister_file_manager_frontend).

---

For the purpose of this demo all files and directories have an `owner`, you can add and delete files and folders to the directories that you own. When adding files with an anonymous principal it is possible for other people to delete the files and / or directories.

### future ideas

- Move to stable storage(!)
- Implementation of access control for serving over http(s), basis is set but not integrated
- Improve fallback like serving a `404`, right now everything falls back to the `index.html` which can cause the frontend to function incorrectly if a file does not exist
- improve data access control / integrate canister owner
- add optional default index page where files and directories are displayed when visiting a directory as with default (unsecured) webservers
- combine `get_file_by_path` and `get_file_path` methods
- overal code cleanup