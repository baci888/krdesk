# Modifications / 修改说明

This repository is a modified version of RustDesk 1.5.1.
Upstream: https://github.com/rustdesk/rustdesk  (License: AGPL-3.0)

本仓库是 RustDesk 1.5.1 的修改版，遵循 AGPL-3.0 协议。

## Changes / 改动

- Server addresses (ID / relay / API) and the key are compiled in at build
  time from repository secrets; see `builtin_custom_client` in `src/common.rs`.
- Account login is required before use; see
  `flutter/lib/desktop/pages/desktop_home_page.dart`.
- Controller-only: the client connects out and cannot be controlled, and the
  server-settings UI is hidden.
- Default video codec is AV1.
- Windows x64 build workflow: `.github/workflows/shell-windows.yml`.
