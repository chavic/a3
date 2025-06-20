import 'package:acter/common/providers/chat_providers.dart';
import 'package:acter/common/providers/space_providers.dart';
import 'package:acter/common/utils/utils.dart';
import 'package:acter/common/widgets/chat/chat_selector_drawer.dart';
import 'package:acter/common/widgets/event/event_selector_drawer.dart';
import 'package:acter/common/widgets/pin/pin_selector_drawer.dart';
import 'package:acter/common/widgets/spaces/space_selector_drawer.dart';
import 'package:acter/common/widgets/super_invite/superInvite_selector_drawer.dart';
import 'package:acter/common/widgets/task/taskList_selector_drawer.dart';
import 'package:acter/features/attachments/actions/add_edit_link_bottom_sheet.dart';
import 'package:acter/features/events/providers/event_providers.dart';
import 'package:acter/features/home/providers/client_providers.dart';
import 'package:acter/features/news/model/news_post_color_data.dart';
import 'package:acter/features/news/model/news_post_state.dart';
import 'package:acter/features/news/model/news_slide_model.dart';
import 'package:acter/features/pins/providers/pins_provider.dart';
import 'package:acter/features/tasks/providers/tasklists_providers.dart';
import 'package:acter_flutter_sdk/acter_flutter_sdk_ffi.dart';
import 'package:flutter/material.dart';
import 'package:acter/l10n/generated/l10n.dart';
import 'package:riverpod/riverpod.dart';

final newsStateProvider =
    StateNotifierProvider<NewsStateNotifier, NewsPostState>(
      (ref) => NewsStateNotifier(ref: ref),
    );

class NewsStateNotifier extends StateNotifier<NewsPostState> {
  final Ref ref;

  NewsStateNotifier({required this.ref}) : super(const NewsPostState());

  void changeTextSlideBackgroundColor() {
    UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
    PostColorScheme postColorScheme = getRandomElement(postColorSchemes);
    selectedUpdateSlide?.backgroundColor = postColorScheme.backgroundColor;
    selectedUpdateSlide?.foregroundColor = postColorScheme.foregroundColor;
    selectedUpdateSlide?.linkColor = postColorScheme.linkColor;

    state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
  }

  Future<void> changeNewsPostSpaceId(BuildContext context) async {
    final selecteSpaceId = ref.read(selectedSpaceIdProvider);
    state = state.copyWith(newsPostSpaceId: selecteSpaceId);
  }

  void setSpaceId(String spaceId) {
    state = state.copyWith(newsPostSpaceId: spaceId);
  }

  void clear() {
    state = const NewsPostState();
  }

  bool isEmpty() {
    return state == const NewsPostState();
  }

  Future<void> selectEventToShare(BuildContext context) async {
    final eventId = await selectEventDrawer(context: context);
    RefDetails? refDetails;
    if (eventId != null) {
      final selectedEvent = await ref.watch(
        calendarEventProvider(eventId).future,
      );
      refDetails = await selectedEvent.refDetails();
    }
    UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
    selectedUpdateSlide?.refDetails = refDetails;
    state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
  }

  Future<void> selectPinToShare(BuildContext context) async {
    final pinId = await selectPinDrawer(context: context);
    RefDetails? refDetails;
    if (pinId != null) {
      final selectedPin = await ref.watch(pinProvider(pinId).future);
      refDetails = await selectedPin.refDetails();
    }
    UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
    selectedUpdateSlide?.refDetails = refDetails;
    state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
  }

  Future<void> selectTaskListToShare(BuildContext context) async {
    final taskListId = await selectTaskListDrawer(context: context);
    RefDetails? refDetails;
    if (taskListId != null) {
      final selectedTaskList = await ref.watch(
        taskListProvider(taskListId).future,
      );
      refDetails = await selectedTaskList.refDetails();
    }
    UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
    selectedUpdateSlide?.refDetails = refDetails;
    state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
  }

  Future<void> selectSpaceToShare(BuildContext context) async {
    final selectedSpaceId = await selectSpaceDrawer(
      context: context,
      canCheck: (m) => m?.canString('CanInvite') == true,
    );
    RefDetails? refDetails;
    if (selectedSpaceId != null) {
      final selectedSpace = await ref.read(
        spaceProvider(selectedSpaceId).future,
      );
      refDetails = await selectedSpace.refDetails();
    }
    UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
    selectedUpdateSlide?.refDetails = refDetails;
    state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
  }

  Future<void> selectChatToShare(BuildContext context) async {
    final selectedChatId = await selectChatDrawer(context: context);
    RefDetails? refDetails;
    if (selectedChatId != null) {
      final selectedChat = await ref.read(chatProvider(selectedChatId).future);
      if (selectedChat == null) return;
      refDetails = await selectedChat.refDetails();
    }
    UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
    selectedUpdateSlide?.refDetails = refDetails;
    state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
  }

  Future<void> enterLinkToShare(BuildContext context) async {
    showAddEditLinkBottomSheet(
      context: context,
      bottomSheetTitle: L10n.of(context).addLink,
      onSave: (title, link) async {
        Navigator.pop(context);
        final client = await ref.read(alwaysClientProvider.future);
        RefDetails refDetails = client.newLinkRefDetails(title, link);
        UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
        selectedUpdateSlide?.refDetails = refDetails;
        state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
      },
    );
  }

  Future<void> selectInvitationCodeToShare(BuildContext context) async {
    final selectedInviteCode = await selectSuperInviteDrawer(context: context);
    RefDetails? refDetails;
    if (selectedInviteCode != null) {
      refDetails = selectedInviteCode.refDetails();
    }
    UpdateSlideItem? selectedUpdateSlide = state.currentUpdateSlide;
    selectedUpdateSlide?.refDetails = refDetails;
    state = state.copyWith(currentUpdateSlide: selectedUpdateSlide);
  }

  void changeTextSlideValue(String body, String? html) {
    state.currentUpdateSlide?.text = body;
    state.currentUpdateSlide?.html = html;
  }

  void changeSelectedSlide(UpdateSlideItem newsSlideModel) {
    state = state.copyWith(currentUpdateSlide: newsSlideModel);
  }

  void addSlide(UpdateSlideItem newsSlideModel) {
    List<UpdateSlideItem> newsSlideList = [
      ...state.newsSlideList,
      newsSlideModel,
    ];
    state = state.copyWith(
      newsSlideList: newsSlideList,
      currentUpdateSlide: newsSlideModel,
    );
  }

  void deleteSlide(int index) {
    List<UpdateSlideItem> newsSlideList = [...state.newsSlideList];
    newsSlideList.removeAt(index);
    if (newsSlideList.isEmpty) {
      state = state.copyWith(
        newsSlideList: newsSlideList,
        currentUpdateSlide: null,
      );
    } else if (index == newsSlideList.length) {
      state = state.copyWith(
        newsSlideList: newsSlideList,
        currentUpdateSlide: newsSlideList[index - 1],
      );
    } else {
      state = state.copyWith(
        newsSlideList: newsSlideList,
        currentUpdateSlide: newsSlideList[index],
      );
    }
  }
}
