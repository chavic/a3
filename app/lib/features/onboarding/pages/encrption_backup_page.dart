import 'package:acter/features/backups/providers/backup_manager_provider.dart';
import 'package:acter/features/encryption_backup_feature/widgets/encryption_backup_widget.dart';
import 'package:acter/features/onboarding/types.dart';
import 'package:acter/features/onboarding/widgets/remindme_key_dialog.dart';
import 'package:acter/l10n/generated/l10n.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:phosphor_flutter/phosphor_flutter.dart';

class EncryptionBackupPage extends ConsumerStatefulWidget {
  final CallNextPage? callNextPage;

  const EncryptionBackupPage({super.key, required this.callNextPage});

  @override
  ConsumerState<EncryptionBackupPage> createState() =>
      _EncryptionBackupPageState();
}

class _EncryptionBackupPageState extends ConsumerState<EncryptionBackupPage> {
  final isEnableNextButton = ValueNotifier<bool>(false);

  @override
  Widget build(BuildContext context) {
    final lang = L10n.of(context);
    final primaryColor = Theme.of(context).colorScheme.primary;
    return Scaffold(
      appBar: AppBar(),
      body: SafeArea(
        child: SingleChildScrollView(
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 32),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const SizedBox(height: 24),
                Icon(PhosphorIcons.lockKey(), size: 80, color: primaryColor),
                const SizedBox(height: 24),
                _buildHeader(context, lang),
                const SizedBox(height: 16),
                _buildDescription(context, lang),
                const SizedBox(height: 32),
                _buildEncryptionKey(context),
                const SizedBox(height: 32),
                _buildNavigationButtons(context, lang),
                const SizedBox(height: 30),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildHeader(BuildContext context, L10n lang) {
    return Text(
      lang.encryptionKeyBackupTitle,
      style: Theme.of(context).textTheme.titleLarge,
      textAlign: TextAlign.center,
    );
  }

  Widget _buildDescription(BuildContext context, L10n lang) {
    return Text(
      lang.encryptionKeyBackupDescription,
      style: Theme.of(context).textTheme.labelMedium?.copyWith(fontSize: 14),
      textAlign: TextAlign.center,
    );
  }

  Widget _buildEncryptionKey(BuildContext context) {
    final encKey = ref.watch(enableEncrptionBackUpProvider);
    return encKey.when(
      data: (data) {
        return Column(
          children: [
            _buildEncryptionKeyContent(context, data),
            const SizedBox(height: 32),
            PasswordManagerBackupWidget(
              encryptionKey: data,
              onButtonPressed: () => isEnableNextButton.value = true,
            ),
          ],
        );
      },
      error:
          (error, stack) => _buildEncryptionKeyError(context, error.toString()),
      loading: () => const Center(child: LinearProgressIndicator()),
    );
  }

  Widget _buildEncryptionKeyContent(BuildContext context, String encKey) {
    final style = Theme.of(
      context,
    ).textTheme.bodyMedium?.copyWith(fontSize: 16, letterSpacing: 1.2);

    return Container(
      padding: const EdgeInsets.symmetric(vertical: 16, horizontal: 24),
      decoration: BoxDecoration(
        color: Theme.of(context).colorScheme.surfaceContainerLow,
        borderRadius: BorderRadius.circular(12),
      ),
      child: SelectableText(encKey, style: style, textAlign: TextAlign.center),
    );
  }

  Widget _buildEncryptionKeyError(BuildContext context, String error) {
    final errorColor = Theme.of(context).colorScheme.error;
    final style = Theme.of(
      context,
    ).textTheme.bodyMedium?.copyWith(color: errorColor);
    return Center(
      child: Column(
        children: [
          Text(error, style: style),
          const SizedBox(height: 16),
          OutlinedButton(
            style: OutlinedButton.styleFrom(
              foregroundColor: errorColor,
              side: BorderSide(color: errorColor),
            ),
            onPressed: () => ref.invalidate(enableEncrptionBackUpProvider),
            child: Text(L10n.of(context).retry),
          ),
        ],
      ),
    );
  }

  Widget _buildNavigationButtons(BuildContext context, L10n lang) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        _buildSaveKeySecurelyButton(context, lang),
        const SizedBox(height: 16),
        _buildRemindMeButton(context, lang),
        const SizedBox(height: 16),
      ],
    );
  }

  Widget _buildSaveKeySecurelyButton(BuildContext context, L10n lang) {
    return ValueListenableBuilder<bool>(
      valueListenable: isEnableNextButton,
      builder: (context, isEnabled, _) {
        return ElevatedButton(
          onPressed:
              isEnabled
                  ? () => showDialog(
                    context: context,
                    builder:
                        (BuildContext context) => RemindMeAboutKeyDialog(
                          callNextPage: widget.callNextPage,
                        ),
                  )
                  : null,
          child: Text(
            lang.savedKeySecurely,
            style: const TextStyle(fontSize: 16),
          ),
        );
      },
    );
  }

  Widget _buildRemindMeButton(BuildContext context, L10n lang) {
    return OutlinedButton(
      onPressed: () => widget.callNextPage?.call(),
      child: Text(L10n.of(context).remindMeLater),
    );
  }
}
