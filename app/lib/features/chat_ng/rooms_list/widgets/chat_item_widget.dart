import 'package:acter/common/providers/chat_providers.dart';
import 'package:acter/features/chat/widgets/room_avatar.dart';
import 'package:acter/features/chat_ng/widgets/chat_item/chat_typing_lastmessage_container_widget.dart';
import 'package:acter/features/chat_ng/widgets/chat_item/display_name_widget.dart';
import 'package:acter/features/chat_ng/widgets/chat_item/last_message_time_widget.dart';
import 'package:acter/features/chat_ng/widgets/chat_item/bookmark_icon_widget.dart';
import 'package:acter/features/chat_ng/widgets/chat_item/mute_icon_widget.dart';
import 'package:acter/features/chat_ng/widgets/chat_item/unread_count_widget.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class ChatItemWidget extends ConsumerWidget {
  final String roomId;
  final bool showSelectedIndication;
  final Function()? onTap;
  final Animation<double>? animation;

  const ChatItemWidget({
    super.key,
    required this.roomId,
    this.showSelectedIndication = true,
    this.onTap,
    this.animation,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    //If chatNG is enabled, we render the ChatItemWidget
    final inner = _buildChatItem(context, ref);
    return animation != null
        ? SizeTransition(sizeFactor: animation!, child: inner)
        : inner;
  }

  Widget _buildChatItem(BuildContext context, WidgetRef ref) {
    final isChatSelected =
        showSelectedIndication && roomId == ref.watch(selectedChatIdProvider);
    return ListTile(
      selected: isChatSelected,
      selectedTileColor: Theme.of(context).colorScheme.primary,
      key: Key('chat-item-widget-$roomId'),
      dense: true,
      onTap: onTap,
      leading: RoomAvatar(roomId: roomId, showParents: true, avatarSize: 44),
      title: _buildChatTitle(),
      shape: Border(
        bottom: BorderSide(
          color: Theme.of(context).colorScheme.outline.withValues(alpha: 0.12),
          width: 0.5,
        ),
      ),
      subtitle: _buildChatSubtitle(isChatSelected),
    );
  }

  Widget _buildChatTitle() {
    return Row(
      children: [
        Expanded(child: DisplayNameWidget(roomId: roomId)),
        SizedBox(width: 12),
        LastMessageTimeWidget(roomId: roomId),
      ],
    );
  }

  Widget _buildChatSubtitle(bool isChatSelected) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.start,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Expanded(child: ChatTypingLastMessageContainerWidget(roomId: roomId, isSelected: isChatSelected)),
        BookmarkIconWidget(roomId: roomId),
        MuteIconWidget(roomId: roomId),
        UnreadCountWidget(roomId: roomId, isSelected: isChatSelected),
      ],
    );
  }
}
