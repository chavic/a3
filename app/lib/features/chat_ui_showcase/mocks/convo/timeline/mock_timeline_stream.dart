import 'package:acter/features/chat_ui_showcase/mocks/convo/timeline/mock_timeline_item_diff.dart';
import 'package:acter_flutter_sdk/acter_flutter_sdk_ffi.dart';
import 'package:mocktail/mocktail.dart';

class MockTimelineStream extends Mock implements TimelineStream {
  final List<MockTimelineItemDiff> mockTimelineItemDiffs;
  final Stream<TimelineItemDiff>? mockMessagesStream;
  final bool mockPaginateBackwards;
  final bool mockSendMessage;
  final bool mockEditMessage;
  final bool mockReplyMessage;
  final bool mockMarkAsRead;
  final bool mockToggleReaction;
  final bool mockDrop;

  MockTimelineStream({
    required this.mockTimelineItemDiffs,
    this.mockMessagesStream,
    this.mockPaginateBackwards = true,
    this.mockSendMessage = true,
    this.mockEditMessage = true,
    this.mockReplyMessage = true,
    this.mockMarkAsRead = true,
    this.mockToggleReaction = true,
    this.mockDrop = true,
  });

  @override
  Stream<TimelineItemDiff> messagesStream() =>
      mockMessagesStream ??
      Stream.fromIterable(mockTimelineItemDiffs.map((diff) => diff));

  @override
  Future<bool> paginateBackwards(int count) =>
      Future.value(mockPaginateBackwards);

  @override
  Future<bool> sendMessage(MsgDraft draft) => Future.value(mockSendMessage);

  @override
  Future<bool> editMessage(String eventId, MsgDraft draft) =>
      Future.value(mockEditMessage);

  @override
  Future<bool> replyMessage(String eventId, MsgDraft draft) =>
      Future.value(mockReplyMessage);

  @override
  Future<bool> markAsRead(bool public) => Future.value(mockMarkAsRead);

  @override
  Future<bool> toggleReaction(String eventId, String key) =>
      Future.value(mockToggleReaction);

  @override
  void drop() => mockDrop;
}
