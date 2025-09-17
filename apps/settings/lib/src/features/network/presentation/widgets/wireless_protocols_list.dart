import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/select/select_type.dart';

class WirelessProtocolsList extends StatelessWidget {
  const WirelessProtocolsList(
      {super.key, this.selectedValue, required this.onChanged});

  final WirelessProtocol? selectedValue;

  final void Function(SelectOption) onChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        MechanixSimpleList(listItems: [
          SimpleListItems(
              title: 'Private Wireless Address',
              trailing: CustomToggle(value: false, onChanged: (v) {}))
        ]),
        MechanixSelect(
          value: selectedValue,
          onChanged: onChanged,
          options: wirelessProtocolOptions,
        ),
      ],
    );
  }
}
