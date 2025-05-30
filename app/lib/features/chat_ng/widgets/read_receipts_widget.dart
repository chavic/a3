import 'package:acter/common/providers/room_providers.dart';
import 'package:acter/common/utils/utils.dart';
import 'package:acter/features/chat_ng/providers/chat_room_messages_provider.dart';
import 'package:acter/l10n/generated/l10n.dart';
import 'package:acter_avatar/acter_avatar.dart';
import 'package:acter_flutter_sdk/acter_flutter_sdk_ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:quds_popup_menu/quds_popup_menu.dart';

class ReadReceiptsWidget extends ConsumerWidget {
  static const readReceiptsPopupMenuKey = Key('read_receipts_popup_menu');
  final TimelineEventItem item;
  final String roomId;
  final String messageId;
  final int showAvatarsLimit;

  const ReadReceiptsWidget({
    super.key,
    required this.item,
    required this.roomId,
    required this.messageId,
    this.showAvatarsLimit = 5,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final theme = Theme.of(context);
    final receipts = ref.watch(messageReadReceiptsProvider(item));
    final userIds = receipts.keys.toList();
    final timestamps = receipts.values.toList();

    return Theme(
      data: theme.copyWith(splashFactory: NoSplash.splashFactory),
      child: QudsPopupButton(
        items: _showDetails(context, userIds, timestamps),
        child: Wrap(
          spacing: -8,
          children: [
            ...List.generate(
              userIds.length <= showAvatarsLimit
                  ? userIds.length
                  : showAvatarsLimit,
              (i) => _buildUserReadAvatars(context, ref, userIds[i]),
            ),
            // overflow indicator
            if (userIds.length > showAvatarsLimit)
              CircleAvatar(
                radius: 8.5,
                child: Text(
                  '+${userIds.length - showAvatarsLimit}',
                  textScaler: const TextScaler.linear(0.6),
                  style: theme.textTheme.labelSmall,
                ),
              ),
          ],
        ),
      ),
    );
  }

  Widget _buildUserReadAvatars(
    BuildContext context,
    WidgetRef ref,
    String userId,
  ) {
    final avatarInfo = ref.watch(
      memberAvatarInfoProvider((roomId: roomId, userId: userId)),
    );

    final options = AvatarOptions.DM(avatarInfo, size: 8);

    return ActerAvatar(options: options);
  }

  List<QudsPopupMenuBase> _showDetails(
    BuildContext context,
    List<String> userIds,
    List<int> timestamps,
  ) {
    final colorScheme = Theme.of(context).colorScheme;
    final textTheme = Theme.of(context).textTheme;
    return [
      QudsPopupMenuWidget(
        builder: (context) {
          return Container(
            key: ReadReceiptsWidget.readReceiptsPopupMenuKey,
            decoration: BoxDecoration(
              color: colorScheme.surface,
              borderRadius: BorderRadius.circular(12),
            ),
            child: Column(
              children: [
                Padding(
                  padding: const EdgeInsets.symmetric(vertical: 8.0),
                  child: Text(
                    L10n.of(context).seenBy,
                    style: textTheme.labelLarge,
                  ),
                ),
                _ReadReceiptsList(
                  userIds: userIds,
                  timestamps: timestamps,
                  roomId: roomId,
                ),
              ],
            ),
          );
        },
      ),
    ];
  }
}

class _ReadReceiptsList extends ConsumerWidget {
  const _ReadReceiptsList({
    required this.userIds,
    required this.timestamps,
    required this.roomId,
  });
  final List<String> userIds;
  final List<int> timestamps;
  final String roomId;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final colorScheme = Theme.of(context).colorScheme;
    final textTheme = Theme.of(context).textTheme;
    return ListView.builder(
      shrinkWrap: true,
      itemCount: userIds.length,
      itemBuilder: (context, index) {
        final userId = userIds[index];
        final timestamp = timestamps[index];
        final member = ref.watch(
          memberAvatarInfoProvider((userId: userId, roomId: roomId)),
        );

        return ListTile(
          minTileHeight: 10,
          horizontalTitleGap: 4,
          dense: true,
          contentPadding: const EdgeInsets.symmetric(horizontal: 8),
          leading: ActerAvatar(options: AvatarOptions.DM(member, size: 8)),
          title: Text(
            member.displayName ?? userId,
            style: textTheme.labelSmall,
          ),
          trailing: Text(
            jiffyDateTimestamp(context, timestamp, showDay: true),
            style: textTheme.labelSmall?.copyWith(color: colorScheme.onSurface),
            overflow: TextOverflow.ellipsis,
          ),
        );
      },
    );
  }
}
