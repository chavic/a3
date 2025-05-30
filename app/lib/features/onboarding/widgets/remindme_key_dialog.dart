import 'package:acter/common/toolkit/buttons/primary_action_button.dart';
import 'package:acter/features/activities/actions/destroy_enc_key_action.dart';
import 'package:acter/features/onboarding/types.dart';
import 'package:flutter/material.dart';
import 'package:acter/l10n/generated/l10n.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class RemindMeAboutKeyDialog extends ConsumerStatefulWidget {
   final CallNextPage? callNextPage;
  const RemindMeAboutKeyDialog({super.key, required this.callNextPage});

  @override
  ConsumerState<RemindMeAboutKeyDialog> createState() =>
      RemindMeAboutKeyDialogState();
}

class RemindMeAboutKeyDialogState
    extends ConsumerState<RemindMeAboutKeyDialog> {
  @override
  Widget build(BuildContext context) {
    final lang = L10n.of(context);
    return AlertDialog(
      title: Text(lang.remindMeLater, textAlign: TextAlign.center),
      content: Container(
        constraints: const BoxConstraints(maxWidth: 500),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisAlignment: MainAxisAlignment.center,
          mainAxisSize: MainAxisSize.min,
          children: [
            Text(
              lang.remindMeAboutKeyLaterDescription,
              textAlign: TextAlign.center,
            ),
          ],
        ),
      ),
      actionsAlignment: MainAxisAlignment.spaceEvenly,
      actions: <Widget>[
        Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisAlignment: MainAxisAlignment.center,
          mainAxisSize: MainAxisSize.min,
          children: [
            ActerPrimaryActionButton(
              onPressed: () => destroyEncKey(context, ref, widget.callNextPage),
              child: Text(lang.remindMeAboutKeyLater),
            ),
            const SizedBox(height: 10),
            OutlinedButton(
              onPressed: () => Navigator.pop(context),
              child: Text(lang.cancel),
            ),
          ],
        ),
      ],
    );
  }
}
