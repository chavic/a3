import 'package:acter_flutter_sdk/acter_flutter_sdk_ffi.dart';
import 'package:mocktail/mocktail.dart';

class MockMsgContent extends Mock implements MsgContent {
  final String? mockBody;
  final String? mockFormattedBody;

  MockMsgContent({this.mockBody, this.mockFormattedBody});

  @override
  String body() => mockBody ?? 'body';

  @override
  String? formattedBody() => mockFormattedBody;
}
