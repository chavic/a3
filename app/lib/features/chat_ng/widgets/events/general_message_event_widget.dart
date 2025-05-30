import 'package:acter/common/providers/room_providers.dart';
import 'package:acter/features/chat_ng/providers/chat_list_providers.dart';
import 'package:acter/features/chat_ng/widgets/events/text_message_widget.dart';
import 'package:acter_flutter_sdk/acter_flutter_sdk_ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class GeneralMessageEventWidget extends ConsumerWidget {
  final String roomId;
  final TimelineEventItem eventItem;

  const GeneralMessageEventWidget({
    super.key,
    required this.roomId,
    required this.eventItem,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final message = ref.watch(lastMessageTextProvider(eventItem));

    //If message is null, return empty
    if (message == null) return const SizedBox.shrink();

    //Get text style
    final textStyle = lastMessageTextStyle(context, ref, roomId);

    //Providers
    final isDM = ref.watch(isDirectChatProvider(roomId)).valueOrNull ?? false;

    //Render
    final List<InlineSpan> spans = [];
    if (!isDM) {
      final senderName = ref.watch(
        lastMessageDisplayNameProvider((
          roomId: roomId,
          userId: eventItem.sender(),
        )),
      );

      spans.add(TextSpan(text: senderName, style: textStyle));
      spans.add(TextSpan(text: ': ', style: textStyle));
    }

    spans.add(TextSpan(text: message, style: textStyle));

    return RichText(
      text: TextSpan(children: spans, style: textStyle),
      maxLines: 2,
      overflow: TextOverflow.ellipsis,
    );
  }
}
