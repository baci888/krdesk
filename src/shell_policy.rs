//! Compiled shell-client policy.
//!
//! `SHELL_ID_SERVER` being present at compile time is the switch. The values
//! choose which ID, relay and API servers the client dials. They are embedded
//! in the executable, so they are not a secret, and they do not decide which
//! account may use which machine.

use std::collections::HashMap;

use base::config::keys;
use serde_json::{Map, Value};

pub const REQUIRE_LOGIN_KEY: &str = "require-login";

pub fn non_empty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}

/// True when this binary was built with a shell ID server.
///
/// Filename licenses and `custom.txt` are how an upstream client is retargeted.
/// Both replace the compiled servers, so a shell build ignores them.
pub fn locks_server_config(id_server: Option<&str>) -> bool {
    non_empty(id_server).is_some()
}

pub fn locks_compiled_servers() -> bool {
    locks_server_config(option_env!("SHELL_ID_SERVER"))
}

pub fn builtin_client_config(
    id_server: Option<&str>,
    relay_server: Option<&str>,
    api_server: Option<&str>,
    key: Option<&str>,
    app_name: Option<&str>,
) -> Option<HashMap<String, Value>> {
    let id_server = non_empty(id_server)?;
    let app_name = non_empty(app_name).unwrap_or("KRDesk");

    let mut overrides = Map::new();
    overrides.insert(
        keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.into(),
        id_server.into(),
    );
    for (key_name, value) in [
        (keys::OPTION_RELAY_SERVER, non_empty(relay_server)),
        (keys::OPTION_API_SERVER, non_empty(api_server)),
        (keys::OPTION_KEY, non_empty(key)),
    ] {
        if let Some(value) = value {
            overrides.insert(key_name.into(), value.into());
        }
    }
    for key_name in [
        keys::OPTION_HIDE_SERVER_SETTINGS,
        keys::OPTION_HIDE_HELP_CARDS,
        keys::OPTION_DISABLE_DISCOVERY_PANEL,
        keys::OPTION_DISABLE_GROUP_PANEL,
    ] {
        overrides.insert(key_name.into(), "Y".into());
    }

    let mut defaults = Map::new();
    // Preference, not an override: the existing codec negotiation can leave AV1
    // when the peer cannot use it.
    defaults.insert(keys::OPTION_CODEC_PREFERENCE.into(), "av1".into());

    let mut data = HashMap::new();
    data.insert("app-name".to_owned(), app_name.into());
    data.insert("conn-type".to_owned(), "outgoing".into());
    data.insert(REQUIRE_LOGIN_KEY.to_owned(), "Y".into());
    data.insert("override-settings".to_owned(), Value::Object(overrides));
    data.insert("default-settings".to_owned(), Value::Object(defaults));
    Some(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_id_server_does_not_lock_or_build_a_client() {
        for id in [None, Some(""), Some("   ")] {
            assert!(!locks_server_config(id));
            assert!(builtin_client_config(id, None, None, None, None).is_none());
        }
    }

    #[test]
    fn compiled_id_server_locks_servers_and_keeps_av1_as_a_preference() {
        assert!(locks_server_config(Some(" id.example.com ")));
        let config = builtin_client_config(
            Some(" id.example.com "),
            Some(" "),
            Some(""),
            None,
            Some("  "),
        )
        .unwrap();
        assert_eq!(config["app-name"], "KRDesk");
        assert_eq!(config["conn-type"], "outgoing");
        assert_eq!(config[REQUIRE_LOGIN_KEY], "Y");

        let overrides = config["override-settings"].as_object().unwrap();
        assert_eq!(
            overrides
                .get(keys::OPTION_CUSTOM_RENDEZVOUS_SERVER)
                .unwrap(),
            "id.example.com"
        );
        assert!(overrides.get(keys::OPTION_RELAY_SERVER).is_none());
        assert!(overrides.get(keys::OPTION_API_SERVER).is_none());
        assert!(overrides.get(keys::OPTION_KEY).is_none());
        assert_eq!(
            overrides.get(keys::OPTION_HIDE_SERVER_SETTINGS).unwrap(),
            "Y"
        );
        assert!(overrides.get(keys::OPTION_CODEC_PREFERENCE).is_none());

        let defaults = config["default-settings"].as_object().unwrap();
        assert_eq!(defaults.get(keys::OPTION_CODEC_PREFERENCE).unwrap(), "av1");
    }

    #[test]
    fn explicit_shell_fields_are_kept() {
        let config = builtin_client_config(
            Some("id.example.com"),
            Some("relay.example.com"),
            Some("https://api.example.com"),
            Some("public-key"),
            Some("Desk"),
        )
        .unwrap();
        assert_eq!(config["app-name"], "Desk");
        let overrides = config["override-settings"].as_object().unwrap();
        assert_eq!(
            overrides.get(keys::OPTION_RELAY_SERVER).unwrap(),
            "relay.example.com"
        );
        assert_eq!(
            overrides.get(keys::OPTION_API_SERVER).unwrap(),
            "https://api.example.com"
        );
        assert_eq!(overrides.get(keys::OPTION_KEY).unwrap(), "public-key");
    }
}
