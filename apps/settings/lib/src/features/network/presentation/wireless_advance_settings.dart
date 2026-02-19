import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_settings.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class WirelessAdvanceSettings extends StatelessWidget {
  const WirelessAdvanceSettings({super.key});

  void onTap(BuildContext context) {
    final bloc = context.read<WirelessSettingsBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: const NetworkSettings(),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return MechanixSectionList(title: 'Advanced settings', sectionListItems: [
      SectionListItems(
        title: 'Manage wireless',
        onTap: () => onTap(context),
      ),
    ]);
  }
}
