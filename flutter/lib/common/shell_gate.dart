/// Local shell gate. A stored token is only a non-empty string on this device.
/// It is not evidence the account server accepted it, and it does not limit
/// which peer ID the client may dial.
bool shellTokenPresent(String accessToken) => accessToken.trim().isNotEmpty;

bool shellOutgoingConnectAllowedFor({
  required String requireLoginOption,
  required String accessToken,
}) {
  if (requireLoginOption != 'Y') return true;
  return shellTokenPresent(accessToken);
}
