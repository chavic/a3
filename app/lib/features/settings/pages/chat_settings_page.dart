import 'package:acter/common/extensions/acter_build_context.dart';
import 'package:acter/config/constants.dart';
import 'package:acter/router/routes.dart';
import 'package:acter/common/widgets/with_sidebar.dart';
import 'package:acter/features/settings/pages/settings_page.dart';
import 'package:acter/features/settings/widgets/auto_download_tile.dart';
import 'package:acter/features/settings/widgets/typing_notice_tile.dart';
import 'package:flutter/material.dart';
import 'package:acter/l10n/generated/l10n.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:settings_ui/settings_ui.dart';

class ChatSettingsPage extends ConsumerWidget {
  const ChatSettingsPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final lang = L10n.of(context);
    return WithSidebar(
      sidebar: const SettingsPage(),
      child: Scaffold(
        appBar: AppBar(
          title: Text(lang.chat),
          automaticallyImplyLeading: !context.isLargeScreen,
        ),
        body: SettingsList(
          sections: [
            SettingsSection(
              tiles: [
                CustomSettingsTile(child: AutoDownloadTile()),
                CustomSettingsTile(child: TypingNoticeTile()),
                SettingsTile.switchTile(
                  title: Text(lang.chatSettingsReadReceipts),
                  description: Text(lang.chatSettingsReadReceiptsExplainer),
                  enabled: false,
                  initialValue: false,
                  onToggle: (newVal) {},
                ),
                if (includeShowCases)
                  CustomSettingsTile(
                    child: Card(
                      margin: EdgeInsets.zero,
                      child: ListTile(
                        onTap:
                            () =>
                                context.pushNamed(Routes.chatListShowcase.name),
                        title: Text(lang.chatUIShowcase),
                        subtitle: Text(
                          lang.chatUIShowcaseDes,
                          style: Theme.of(context).textTheme.labelMedium,
                        ),
                        trailing: Icon(Icons.arrow_forward_ios),
                      ),
                    ),
                  ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}
