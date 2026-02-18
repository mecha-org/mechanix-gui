import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/security_protocols.dart';
import 'package:mechanix_settings/src/features/network/presentation/connect_secure_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_strength_icon.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class SavedNetworks extends StatefulWidget {
  const SavedNetworks({super.key, required this.scrollToTop});

  final Function scrollToTop;

  @override
  State<SavedNetworks> createState() => _SavedNetworksState();
}

class _SavedNetworksState extends State<SavedNetworks> {
  List<SectionListItems> _buildWifiListItems(
    BuildContext context,
    List<AccessPoints> accessPoints,
  ) {
    return accessPoints.map((ap) {
      final ssid = utf8.decode(ap.nmAccessPoint.ssid);

      return SectionListItems(
        title: ssid,
        onTap: () => _onNetworkTap(ap, context),
        leading: getWirelessStrengthIcon(
          strength: ap.nmAccessPoint.strength,
          isSecure: ap.isSecure,
          isActive: ap.isActive,
        ),
        defaultTrailingIcon: false,
        trailing: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            IconButton(
              onPressed: () => _onInfoTap(ap, context),
              icon: SizedBox(
                height: 24,
                width: 24,
                child: IconWidget(
                  iconPath: Images.settings,
                  iconColor: context.onSurfaceVariant,
                ),
              ),
            ),
          ],
        ),
      );
    }).toList();
  }

  void _onNetworkTap(AccessPoints item, BuildContext context) {
    final wirelessSettingsBloc = context.read<WirelessSettingsBloc>();

    wirelessSettingsBloc.add(SelectNetwork(item));
    wirelessSettingsBloc.add(SelectNetworkPoint(item.nmAccessPoint));

    if (item.isSaved && !item.isActive) {
      wirelessSettingsBloc.add(ConnectSavedNetwork('', item.nmAccessPoint));
    } else if (!item.isSecure && !item.isActive) {
      context
          .read<ConnectNetworkBloc>()
          .add(ConnectToNetwork(item.nmAccessPoint));
    } else {
      _showSecureNetworkDialog(context, item);
    }
  }

  void _showSecureNetworkDialog(BuildContext context, AccessPoints? item) {
    final connectNetworkBloc = context.read<ConnectNetworkBloc>();
    final wirelessSettingsBloc = context.read<WirelessSettingsBloc>();

    MechanixBottomSheet.show(
      context,
      withCloseButton: true,
      child: MultiBlocProvider(
        providers: [
          BlocProvider.value(value: connectNetworkBloc),
          BlocProvider.value(value: wirelessSettingsBloc),
        ],
        child: ConnectSecureNetwork(
          accessPoint: item?.nmAccessPoint,
          scrollToTop: widget.scrollToTop,
        ),
      ),
    );
  }

  void _onInfoTap(AccessPoints item, BuildContext context) {
    final connectNetworkBloc = context.read<ConnectNetworkBloc>();
    final wirelessSettingsBloc = context.read<WirelessSettingsBloc>();

    wirelessSettingsBloc.add(SelectNetwork(item));
    wirelessSettingsBloc.add(SelectNetworkPoint(item.nmAccessPoint));

    final flag = getWirelessProtocol(item.nmAccessPoint.rsnFlags);
    wirelessSettingsBloc.add(SelectedWirelessProtocol(flag));

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => MultiBlocProvider(
          providers: [
            BlocProvider.value(value: connectNetworkBloc),
            BlocProvider.value(value: wirelessSettingsBloc),
          ],
          child: NetworkDetails(scrollToTop: widget.scrollToTop),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return BlocListener<WirelessSettingsBloc, WirelessSettingsState>(
      listenWhen: (previous, current) =>
          previous.deviceState != current.deviceState &&
          current.deviceState == NetworkManagerDeviceState.failed,
      listener: (context, state) {
        final fromSavedNetwork = state.availableSavedNetworks.any((network) =>
            listEquals(network.nmAccessPoint.ssid,
                state.selectedAccessPoint?.nmAccessPoint.ssid));

        if (fromSavedNetwork &&
            state.deviceState == NetworkManagerDeviceState.failed) {
          _showSecureNetworkDialog(context, state.selectedAccessPoint);
        }
      },
      child: BlocSelector<WirelessSettingsBloc, WirelessSettingsState,
          ({List<AccessPoints> list})>(
        selector: (state) => (list: state.availableSavedNetworks,),
        builder: (context, data) {
          return MechanixSectionList(
            physics: const BouncingScrollPhysics(),
            title: 'My networks',
            theme: const MechanixSectionListThemeData(
              widgetPadding: EdgeInsets.zero,
            ),
            sectionListItems: _buildWifiListItems(
              context,
              data.list,
            ),
          ).padTop(36);
        },
      ),
    );
  }
}
