import 'package:flutter_hbb/common/shell_gate.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  test('connects are unchanged when login is not required', () {
    expect(
      shellOutgoingConnectAllowedFor(
        requireLoginOption: '',
        accessToken: '',
      ),
      isTrue,
    );
    expect(
      shellOutgoingConnectAllowedFor(
        requireLoginOption: 'N',
        accessToken: '   ',
      ),
      isTrue,
    );
  });

  test('a required login rejects an empty or blank stored token', () {
    expect(
      shellOutgoingConnectAllowedFor(
        requireLoginOption: 'Y',
        accessToken: '',
      ),
      isFalse,
    );
    expect(
      shellOutgoingConnectAllowedFor(
        requireLoginOption: 'Y',
        accessToken: ' \n\t ',
      ),
      isFalse,
    );
  });

  test('any non-empty stored token passes this local gate', () {
    expect(
      shellOutgoingConnectAllowedFor(
        requireLoginOption: 'Y',
        accessToken: 'not-a-server-acceptance',
      ),
      isTrue,
    );
  });
}
