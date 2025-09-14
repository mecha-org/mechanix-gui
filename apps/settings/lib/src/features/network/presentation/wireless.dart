import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless_advance_settings.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

class WirelessSettings extends StatelessWidget {
  const WirelessSettings({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
        builder: (context, state) {
      return Scaffold(
          appBar: MechanixNavigationBar(title: "Network"),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  MechanixSimpleList(isDividerRequired: true, listItems: [
                    SimpleListItems(
                      title: 'Wireless',
                      trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        style: MechanixSwitchStyle(
                          inactiveThumbColor: Color(0xFF989898),
                          inactiveTrackColor: Color(0xFF252525),
                        ),
                        value: state.wifiOn,
                        onChanged: (val) => context
                            .read<WirelessSettingsBloc>()
                            .add(ToggleWifi(val)),
                      ),
                    ),
                    if (state.wifiOn && state.connectedNetwork != null)
                      SimpleListItems(
                        onTap: () =>
                            onInfoTap(state.connectedNetwork!, context),
                        title: utf8
                            .decode(state.connectedNetwork!.nmAccessPoint.ssid),
                        leading: SizedBox(
                            height: 24,
                            width: 24,
                            // iconColor: context.colorScheme.primary,
                            child: getWirelessStrengthIcon(
                                strength: state.connectedNetwork?.nmAccessPoint
                                        .strength ??
                                    0,
                                isActive: true)),
                        trailing: IconButton(
                            onPressed: () =>
                                onNetworkTap(state.connectedNetwork!, context),
                            icon: SizedBox(
                                height: 24,
                                width: 24,
                                child: IconWidget(
                                  iconPath: Images.settings,
                                ))),
                      )
                  ]),
                  if (state.wifiOn && !state.loading && state.networks.isEmpty)
                    // const Center(child: Text("No networks found")),
                    MechanixSectionList(
                        title: 'Available Networks',
                        sectionListItems: [
                          SectionListItems(
                            title: 'No networks found',
                            backgroundColor: Colors.transparent,
                            defaultTrailingIcon: false,
                          ),
                        ]),
                  if (state.wifiOn && state.networks.isNotEmpty)
                    MechanixSectionList(
                        title: 'Available Networks',
                        sectionListItems: getWifiList(context, state.networks)),
                  const WirelessAdvanceSettings()
                ],
              ),
            ),
          ));
    });
  }
}

void onNetworkTap(AccessPoints item, BuildContext context) {
  context.read<WirelessSettingsBloc>().add(SelectNetwork(item));
  if (item.isSaved && !item.isActive) {
    context
        .read<WirelessSettingsBloc>()
        .add(ConnectSavedNetwork('', item.nmAccessPoint));
  } else if (!item.isActive) {
    Navigator.pushNamed(
      context,
      AppRoutes.wirelessConnectSecureNetwork,
      arguments: {'accessPoint': item.nmAccessPoint},
    );
  }
}

void onInfoTap(AccessPoints item, BuildContext context) {
  Navigator.pushNamed(
    context,
    AppRoutes.wirelessNetworkDetails,
    arguments: {'networkDetails': item},
  );
}

String getNetworkIcon(String security, int? signalStrength) {
  // Set base icon based on WPA
  String icon = security.contains('WPA')
      ? Images.securedWirelessStrong
      : Images.wirelessStrong;

  // Update based on signal strength
  if (signalStrength != null) {
    if (signalStrength < 30) {
      icon = icon.replaceAll('strong', 'low');
    } else if (signalStrength < 70) {
      icon = icon.replaceAll('strong', 'weak');
    }
  }

  return icon;
}

List<SectionListItems> getWifiList(
    BuildContext context, List<AccessPoints> state) {
  final wifi = state.map((s) {
    return SectionListItems(
      title: utf8.decode(s.nmAccessPoint.ssid),
      onTap: () => onInfoTap(s, context),
      leading: SizedBox(
        height: 24,
        width: 24,
        child: getWirelessStrengthIcon(strength: s.nmAccessPoint.strength),
      ),
      defaultTrailingIcon: false,
      trailing: IconButton(
        onPressed: () => onNetworkTap(s, context),
        icon: SizedBox(
          height: 24,
          width: 24,
          child: IconWidget(iconPath: Images.settings),
        ),
      ),
    );
  }).toList();

  wifi.add(
    SectionListItems(
      title: 'Add Wireless',
      defaultTrailingIcon: false,
      onTap: () =>
          Navigator.pushNamed(context, AppRoutes.wirelessConnectUnknownNetwork),
      leading: IconWidget(iconPath: Images.wirelessAdd),
    ),
  );

  return wifi;
}

IconWidget getWirelessStrengthIcon(
    {required int strength, bool isActive = false}) {
  if (strength > 75) {
    return IconWidget(isActive: isActive, iconPath: Images.wifiHigh);
  } else if (strength > 50) {
    return IconWidget(isActive: isActive, iconPath: Images.wifiMedium);
  } else if (strength > 25) {
    return IconWidget(isActive: isActive, iconPath: Images.wifiLow);
  } else {
    return IconWidget(isActive: isActive, iconPath: Images.wifiNone);
  }
}
