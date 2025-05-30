import 'package:acter/features/chat/widgets/convo_card.dart';
import 'package:acter/features/chat_ng/rooms_list/widgets/chat_item_widget.dart';
import 'package:acter/features/labs/model/labs_features.dart';
import 'package:acter/features/labs/providers/labs_providers.dart';
import 'package:acter/features/space/widgets/space_sections/section_header.dart';
import 'package:acter/router/utils.dart';
import 'package:acter_flutter_sdk/acter_flutter_sdk_ffi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:acter/l10n/generated/l10n.dart';
import 'package:acter/common/extensions/options.dart';

class ChatListWidget extends ConsumerWidget {
  final ProviderBase<List<Convo>> chatListProvider;
  final int? limit;
  final bool showSectionHeader;
  final VoidCallback? onClickSectionHeader;
  final String? sectionHeaderTitle;
  final bool? isShowSeeAllButton;
  final bool showSectionBg;
  final bool shrinkWrap;
  final bool showBookmarkedIndicator;
  final bool showSelectedIndication;
  final Widget emptyState;

  const ChatListWidget({
    super.key,
    required this.chatListProvider,
    this.limit,
    this.showSectionHeader = false,
    this.onClickSectionHeader,
    this.sectionHeaderTitle,
    this.isShowSeeAllButton,
    this.showSectionBg = true,
    this.shrinkWrap = true,
    this.showSelectedIndication = false,
    this.showBookmarkedIndicator = true,
    this.emptyState = const SizedBox.shrink(),
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final chatList = ref.watch(chatListProvider);
    if (chatList.isEmpty) return emptyState;

    final count = (limit ?? chatList.length).clamp(0, chatList.length);
    return showSectionHeader
        ? Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            SectionHeader(
              showSectionBg: showSectionBg,
              title: sectionHeaderTitle ?? L10n.of(context).chats,
              isShowSeeAllButton: isShowSeeAllButton ?? count < chatList.length,
              onTapSeeAll: onClickSectionHeader.map((cb) => () => cb()),
            ),
            chatListUI(ref, chatList, count),
          ],
        )
        : chatListUI(ref, chatList, count);
  }

  Widget chatListUI(WidgetRef ref, List<Convo> chatList, int count) {
    final isChatNG = ref.watch(isActiveProvider(LabsFeature.chatNG));
    return ListView.builder(
      shrinkWrap: shrinkWrap,
      itemCount: count,
      padding: EdgeInsets.all(16),
      physics: shrinkWrap ? const NeverScrollableScrollPhysics() : null,
      itemBuilder: (context, index) {
        final roomId = chatList[index].getRoomIdStr();
        return isChatNG
            ? ChatItemWidget(
              roomId: roomId,
              showSelectedIndication: showSelectedIndication,
              onTap: () => goToChat(context, roomId),
            )
            : Card(
              margin: EdgeInsets.only(bottom: 16),
              child: ConvoCard(
                showSelectedIndication: showSelectedIndication,
                roomId: roomId,
                onTap: () => goToChat(context, roomId),
              ),
            );
      },
    );
  }
}
