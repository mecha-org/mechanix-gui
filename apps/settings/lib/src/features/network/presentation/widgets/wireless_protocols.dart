import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_protocols_list.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/select/select_type.dart';

class WirelessProtocols extends StatefulWidget {
  const WirelessProtocols({
    this.accessPoint,
    super.key,
    this.isDisabled = false,
  });

  final NetworkManagerAccessPoint? accessPoint;
  final bool isDisabled;

  @override
  State<WirelessProtocols> createState() => _WirelessProtocolsState();
}

class _WirelessProtocolsState extends State<WirelessProtocols> {
  WirelessProtocol? selectedValue;

  void onChange(SelectOption value) {
    setState(() {
      selectedValue = value.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
      builder: (context, connectState) {
        return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
          builder: (wirelessContext, wirelessState) {
            final label = wirelessProtocolOptions
                .firstWhereOrNull((element) =>
                    element.value == wirelessState.selectedAPWirelessProtocol)
                ?.label;

            return MechanixSimpleList(
              listItems: [
                SimpleListItems(
                    disabled: widget.isDisabled,
                    onTap: () => onTap(context, widget.accessPoint),
                    title: 'Security',
                    trailing: Row(
                      children: [
                        CustomTrailingText(
                          title: label ?? '',
                        ).padRight(8),
                      ],
                    ))
              ],
            );
          },
        );
      },
    );
  }
}

void onTap(BuildContext context, NetworkManagerAccessPoint? accessPoint) {
  final bloc = context.read<ConnectNetworkBloc>();

  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (context) => BlocProvider.value(
        value: bloc, // reuse the existing bloc
        child: WirelessProtocolsList(accessPoint: accessPoint),
      ),
    ),
  );
}
