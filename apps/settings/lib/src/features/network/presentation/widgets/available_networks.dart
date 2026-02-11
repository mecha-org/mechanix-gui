import 'dart:convert';

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
import 'package:mechanix_settings/src/features/network/presentation/add_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/connect_secure_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_strength_icon.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class AvailableNetworks extends StatefulWidget {
  const AvailableNetworks({super.key, required this.scrollToTop});

  final Function scrollToTop;

  @override
  State<AvailableNetworks> createState() => _AvailableNetworksState();
}

class _AvailableNetworksState extends State<AvailableNetworks> {
  List<SectionListItems> _buildWifiListItems(
    BuildContext context,
    List<AccessPoints> accessPoints,
  ) {
    final wifi = accessPoints.map((ap) {
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
    wifi.add(
      SectionListItems.leadingIcon(
        title: 'Add Wireless',
        titleTextStyle:
            context.textTheme.labelMedium?.copyWith(color: context.primary),
        defaultTrailingIcon: false,
        onTap: () {
          final connectNetworkBloc = context.read<ConnectNetworkBloc>();
          final wirelessSettingsBloc = context.read<WirelessSettingsBloc>();

          MechanixBottomSheet.show(
            context,
            child: MultiBlocProvider(
              providers: [
                BlocProvider.value(value: connectNetworkBloc),
                BlocProvider.value(value: wirelessSettingsBloc),
              ],
              child: const AddNetwork(),
            ),
          );
        },
        iconPath: Images.wirelessAdd,
        isActive: true,
      ),
    );
    return wifi;
  }

  void _onNetworkTap(AccessPoints item, BuildContext context) {
    if (item.nmAccessPoint.wpaFlags.isEmpty &&
        item.nmAccessPoint.rsnFlags.isEmpty) {
      context
          .read<ConnectNetworkBloc>()
          .add(ConnectToNetwork(item.nmAccessPoint));
    } else {
      final connectNetworkBloc = context.read<ConnectNetworkBloc>();
      final wirelessSettingsBloc = context.read<WirelessSettingsBloc>();

      MechanixBottomSheet.show(
        context,
        child: MultiBlocProvider(
          providers: [
            BlocProvider.value(value: connectNetworkBloc),
            BlocProvider.value(value: wirelessSettingsBloc),
          ],
          child: ConnectSecureNetwork(
              accessPoint: item.nmAccessPoint, scrollToTop: widget.scrollToTop),
        ),
      );
    }
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
    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
      builder: (context, state) {
        return MechanixSectionList(
          physics: const BouncingScrollPhysics(),
          title: 'Available networks',
          theme: const MechanixSectionListThemeData(
            widgetPadding: EdgeInsets.zero,
          ),
          sectionListItems: _buildWifiListItems(
            context,
            state.availableOtherNetworks,
          ),
        );
      },
    );
  }
}
