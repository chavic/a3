import 'dart:async';

import 'package:acter/common/providers/notifiers/space_notifiers.dart';
import 'package:acter/common/providers/room_providers.dart';
import 'package:acter_avatar/acter_avatar.dart';
import 'package:acter_flutter_sdk/acter_flutter_sdk_ffi.dart';
import 'package:logging/logging.dart';
import 'package:riverpod/riverpod.dart';

final _log = Logger('a3::common::space_providers');

/// Provider the list of all spaces, keeps up to date with the order and the underlying client
final spacesProvider = NotifierProvider<SpaceListNotifier, List<Space>>(
  () => SpaceListNotifier(),
);

final hasSpacesProvider = Provider(
  (ref) => ref.watch(spacesProvider).isNotEmpty,
);

final bookmarkedSpacesProvider = Provider(
  (ref) => ref.watch(spacesProvider).where((s) => s.isBookmarked()).toList(),
);

final unbookmarkedSpacesProvider = Provider(
  (ref) => ref.watch(spacesProvider).where((s) => !s.isBookmarked()).toList(),
);

/// Map a spaceId to the space, keeps up to date with underlying client
/// it will pend forever if the space isn’t found.
final spaceProvider = FutureProvider.family<Space, String>((
  ref,
  spaceId,
) async {
  final maybeSpace = await ref.watch(maybeSpaceProvider(spaceId).future);
  if (maybeSpace != null) {
    return maybeSpace;
  }
  final completer = Completer<Space>();
  return completer.future;
});

final isActerSpace = FutureProvider.autoDispose.family<bool, String>((
  ref,
  spaceId,
) async {
  final space = await ref.watch(spaceProvider(spaceId).future);
  return await space.isActerSpace();
});

final createSpaceOnboardingDataProvider = FutureProvider.autoDispose
    .family<bool, String>((ref, spaceId) async {
      final space = await ref.watch(spaceProvider(spaceId).future);
      return await space.createOnboardingData();
    });

final spaceBookmarkProvider = NotifierProvider<SpaceBookmarkNotifier, Map<String, bool>>(
  () => SpaceBookmarkNotifier(),
);

/// Attempts to map a spaceId to the space, but could come back empty (null) rather than throw.
/// keeps up to date with underlying client even if the space wasn’t found initially,
final maybeSpaceProvider =
    AsyncNotifierProvider.family<AsyncMaybeSpaceNotifier, Space?, String>(
      () => AsyncMaybeSpaceNotifier(),
    );

/// gives current context space id
final selectedSpaceIdProvider = StateProvider.autoDispose<String?>(
  (ref) => null,
);

/// gives current context space details based on id, will throw null if id is null
final selectedSpaceDetailsProvider = Provider.autoDispose<SpaceItem?>((ref) {
  final selectedSpaceId = ref.watch(selectedSpaceIdProvider);
  if (selectedSpaceId == null) return null;
  return ref.watch(briefSpaceItemProvider(selectedSpaceId));
});

class SpaceItem {
  final Member? membership;
  final Space? space;
  final String roomId;
  final AvatarInfo avatarInfo;
  final List<Member> activeMembers;

  const SpaceItem({
    this.membership,
    this.space,
    required this.roomId,
    required this.activeMembers,
    required this.avatarInfo,
  });
}

class SpaceRelationsOverview {
  bool hasMore;
  List<String> knownSubspaces;
  List<String> knownChats;
  List<String> suggestedIds;
  Space? mainParent;
  List<Space> parents;
  List<Space> otherRelations;

  SpaceRelationsOverview({
    required this.knownSubspaces,
    required this.knownChats,
    required this.suggestedIds,
    required this.mainParent,
    required this.parents,
    required this.otherRelations,
    required this.hasMore,
  });
}

/// Whether the user has at least one space, where they have the requested permission
final hasSpaceWithPermissionProvider = FutureProvider.family
    .autoDispose<bool, String>((ref, permission) async {
      final spaces = ref.watch(spacesProvider);
      for (final element in spaces) {
        final membership = await element.getMyMembership();
        if (membership.canString(permission)) return true;
      }
      // none found
      return false;
    });

/// Get the list of known spaces as SpaceItem filled in brief form
/// (only spaceProfileData, no activeMembers) but with user membership attached.
/// Stays up to date with underlying client info
final _spaceIdAndNames = FutureProvider.autoDispose<List<_SpaceIdAndName>>((
  ref,
) async {
  final spaces = ref
      .watch(bookmarkedSpacesProvider)
      .followedBy(ref.watch(unbookmarkedSpacesProvider));
  List<_SpaceIdAndName> items = [];
  for (final space in spaces) {
    final roomId = space.getRoomIdStr();
    final dispName = await ref.watch(roomDisplayNameProvider(roomId).future);
    items.add((roomId, dispName));
  }
  return items;
});

class SpaceNotFound extends Error {}

typedef _SpaceIdAndName = (String, String?);

final searchedSpacesProvider = FutureProvider.autoDispose<List<String>>((
  ref,
) async {
  final searchValue = ref.watch(roomSearchValueProvider);
  final allSpaces = await ref.watch(_spaceIdAndNames.future);

  if (searchValue == null || searchValue.isEmpty) {
    return allSpaces.map((item) {
      final (roomId, dispName) = item;
      return roomId;
    }).toList();
  }

  final searchTerm = searchValue.toLowerCase();

  final foundSpaces = List<String>.empty(growable: true);

  for (final (roomId, dispName) in allSpaces) {
    if (roomId.contains(searchTerm) ||
        dispName?.toLowerCase().contains(searchTerm) == true) {
      foundSpaces.add(roomId);
    }
  }

  return foundSpaces;
});

/// Get the SpaceItem of the given spaceId filled in brief form
/// (only spaceProfileData, no activeMembers). Stays up to date with underlying
/// client info
final briefSpaceItemProvider = Provider.autoDispose.family<SpaceItem, String>((
  ref,
  spaceId,
) {
  final avatarInfo = ref.watch(roomAvatarInfoProvider(spaceId));
  return SpaceItem(
    roomId: spaceId,
    membership: null,
    activeMembers: [],
    avatarInfo: avatarInfo,
  );
});

/// Get the members invited of a given roomId the user knows about. Errors
/// if the space isn’t found. Stays up to date with underlying client data
/// if a space was found.
final spaceInvitedMembersProvider = FutureProvider.autoDispose
    .family<List<Member>, String>((ref, roomIdOrAlias) async {
      final space = await ref.watch(spaceProvider(roomIdOrAlias).future);
      if (!space.isJoined()) return [];
      final members = await space.invitedMembers();
      return members.toList();
    });

/// Get the SpaceRelationsOverview of related SpaceItem for the space. Errors if
/// the space or any related space isn’t found. Stays up  to date with underlying
/// client data if a space was found.
final spaceRelationsOverviewProvider =
    FutureProvider.family<SpaceRelationsOverview, String>((ref, spaceId) async {
      final relatedSpaces = await ref.watch(
        spaceRelationsProvider(spaceId).future,
      );
      if (relatedSpaces == null) throw SpaceNotFound;
      bool hasMore = false;
      final List<String> knownSubspaces = [];
      final List<String> knownChats = [];
      final List<String> suggested = [];
      for (final related in relatedSpaces.children()) {
        final roomId = related.roomId().toString();
        if (related.suggested()) suggested.add(roomId);

        final room = ref.watch(maybeRoomProvider(roomId)).valueOrNull;
        if (room == null || !room.isJoined()) {
          // we don’t know this room or are not in it
          hasMore = true;
          continue;
        }

        if (related.targetType() == 'ChatRoom') {
          // we know this as a chat room
          knownChats.add(roomId);
        } else {
          // this must be some space.
          knownSubspaces.add(roomId);
        }
      }

      Space? mainParent;
      final mainSpace = relatedSpaces.mainParent();
      if (mainSpace != null) {
        if (mainSpace.targetType() != 'ChatRoom') {
          final roomId = mainSpace.roomId().toString();
          try {
            final space = await ref.watch(spaceProvider(roomId).future);
            if (space.isJoined()) mainParent = space;
          } catch (e, s) {
            _log.severe('Loading main Parent of $spaceId failed', e, s);
          }
        }
      }

      List<Space> parents = [];
      for (final related in relatedSpaces.otherParents()) {
        if (related.targetType() != 'ChatRoom') {
          final roomId = related.roomId().toString();
          try {
            final space = await ref.watch(spaceProvider(roomId).future);
            if (space.isJoined()) parents.add(space);
          } catch (e, s) {
            _log.severe('Loading other Parents of $spaceId failed', e, s);
          }
        }
      }
      return SpaceRelationsOverview(
        parents: parents,
        knownChats: knownChats,
        knownSubspaces: knownSubspaces,
        otherRelations: [],
        mainParent: mainParent,
        hasMore: hasMore,
        suggestedIds: suggested,
      );
    });

final acterAppSettingsProvider = FutureProvider.autoDispose
    .family<ActerAppSettings?, String>((ref, spaceId) async {
      final space = await ref.watch(maybeSpaceProvider(spaceId).future);
      if (space == null) return null;
      if (!await space.isActerSpace()) return null;
      return await space.appSettings();
    });

/// Whether there were any rooms in the accepted invites
final hasSpaceRedeemedInInviteCodeProvider = StateProvider.autoDispose<bool>(
  (ref) => false,
);

final hasRecommendedSpaceJoinedProvider = StateProvider.autoDispose<bool>(
  (ref) => false,
);
