// Mock class for the avatar info if needed
import 'package:acter_avatar/acter_avatar.dart';
import 'package:mocktail/mocktail.dart';

class MockAvatarInfo extends Mock implements AvatarInfo {
  @override
  final String uniqueId;

  String get userId => uniqueId;

  final String? mockDisplayName;

  @override
  String get displayName => mockDisplayName ?? 'Test User';

  @override
  TooltipStyle get tooltip => TooltipStyle.Combined;

  MockAvatarInfo({required this.uniqueId, this.mockDisplayName});
}
