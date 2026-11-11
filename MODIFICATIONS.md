# Modifications / 修改说明

This repository is a modified version of RustDesk 1.5.1.
Upstream: https://github.com/rustdesk/rustdesk (GNU AGPL-3.0).
The shell snapshot was imported on 2026-11-05. This note was corrected on 2026-11-11.

本仓库是 RustDesk 1.5.1 的修改版。上游协议是 GNU AGPL-3.0。
发布修改版可执行文件时，需要按 AGPL 提供对应源码。窗口标题可以在编译期改成 KRDesk，图标和大量界面资源仍是上游的 RustDesk 素材。

## What the shell build actually does / 套壳构建实际做了什么

These behaviors exist only when `SHELL_ID_SERVER` is non-empty at compile time.
A build without that variable is the upstream client.

- `src/shell_policy.rs` compiles the ID server, and any non-empty relay, API
  server and public key, into the custom-client map. The strings are not in
  git. They can be read out of the executable. Hiding them is not authorization.
- The same build sets `conn-type` to `outgoing`. The existing server startup
  then does not register for incoming control or install the Windows service.
  That is not a check that this process has no other listener.
- `require-login` makes the Flutter UI store an account token before it opens
  a session. The dialog can be closed; manual ID entry, recent peers, deep
  links and the multi-window connect path now use the same local check.
  Any non-empty stored string passes. The account server is not asked, and the
  peer ID is not compared with an assigned device.
- Server fields are override settings, so the normal option file does not
  replace them. A shell build also ignores `custom.txt` and a Windows
  executable-name license, which otherwise replace those servers.
- AV1 is a default codec preference, not an override. The existing negotiation
  can leave AV1 when the peer cannot encode or decode it. The settings page
  can still change the preference.
- `.github/workflows/shell-windows.yml` builds a Windows x64 executable on a
  push to `shell` or a manual run. It does not sign the file and it does not
  prove that a login, a device assignment, or a lease is enforced.

## What this repository does not do / 本仓库没有做的事

There is no connection-layer allowlist, no lease expiry disconnect, and no
local data wipe. An active session is not closed when the token later fails.
Passwords can still be passed on a deep link after a token is stored, and the
token and peer passwords remain on the local machine.

Purchasing, country and machine selection, payment, lease length, gift months,
supplier ownership, commission, and optional third-party account provisioning
belong in a separate business backend. None of that is implemented here, and
none of those rules are compiled into this client.

The client can dial a RustDesk ID through the compiled ID and relay servers.
It does not know which supplier owns a machine, which machine a payment
assigned, when a lease ends, or whether an add-on account is active. Those
decisions have to be enforced by the business backend together with the relay
or the device agent. A list filtered only inside this client can be edited or
bypassed. The person who physically controls a Mac can use that Mac; this
software cannot prevent the host owner from doing so.

## Backend boundary / 后端边界

The client already speaks the existing RustDesk account API (`/api/login`,
`/api/currentUser`, `/api/logout`, and the address-book pull). That API is not
a rental authorization service.

A separate backend would need its own contract before a lease can be enforced.
This client does not call it:

- identify the signed-in account
- return only machines leased to that account, with the lease end
- decide, for one peer ID, whether this account may connect now
- tell the relay or the device agent to refuse or drop the session when the
  lease ends

Until that exists and is tested against a real server and a real Mac, this
client cannot claim those controls. `LeaseAuthorization::Unavailable` is the
only answer this tree can give, and `production_rental_allowed` is false.
There is no stand-in HTTP handler that returns allow.

Same-country relay, same-country capacity, and refusing a cross-country
fallback are also outside this client. `regional_relay_decision` stays
`NotSchedulable` for any count of compiled servers. One compiled address
cannot prove it is in the device's country, and this client has no health
list to refuse when that country has no node.

The business backend, not this repository, is also where customer, supplier,
and platform-admin data have to be separated. A device needs a supplier
owner. One supplier must not receive another supplier's customers or
credentials. Self-operated and third-party machines can share that model
later. Commission and settlement are undecided. Korea is the stated first
market. Later names are Japan, the United States, and Southeast Asia
without a country list, with a preference for staying near mainland
China. Hong Kong and Macau are excluded. The optional Claude account is
not sold or provisioned by this client; a future purchase page can show
it as unavailable until its price and the legal-entity conditions exist. Suggested split: this client keeps the remote
session, a Mac agent enrolls the device, and the business API is the only
place that records a lease and answers whether a connection is allowed.
