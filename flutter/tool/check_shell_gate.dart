import '../lib/common/shell_gate.dart';

void check(String name, bool got, bool want) {
  if (got != want) {
    throw StateError('$name: got $got want $want');
  }
}

void main() {
  check(
    'empty option',
    shellOutgoingConnectAllowedFor(requireLoginOption: '', accessToken: ''),
    true,
  );
  check(
    'explicit N',
    shellOutgoingConnectAllowedFor(requireLoginOption: 'N', accessToken: '   '),
    true,
  );
  check(
    'required empty',
    shellOutgoingConnectAllowedFor(requireLoginOption: 'Y', accessToken: ''),
    false,
  );
  check(
    'required blank',
    shellOutgoingConnectAllowedFor(
      requireLoginOption: 'Y',
      accessToken: ' \n\t ',
    ),
    false,
  );
  check(
    'required any token',
    shellOutgoingConnectAllowedFor(
      requireLoginOption: 'Y',
      accessToken: 'not-a-server-acceptance',
    ),
    true,
  );
}
