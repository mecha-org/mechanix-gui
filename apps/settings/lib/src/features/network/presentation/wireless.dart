import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/presentation/connect_secure_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless_advance_settings.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:widgets/widgets/switch/mechanix_switch.dart';
import 'package:widgets/widgets/switch/mechanix_switch_theme.dart';

class WirelessSettings extends StatefulWidget {
  const WirelessSettings({super.key});

  @override
  State<WirelessSettings> createState() => _WirelessSettingsState();
}

class _WirelessSettingsState extends State<WirelessSettings> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
        builder: (context, state) {
      return Scaffold(
          appBar: PreferredSize(
              preferredSize: const Size.fromHeight(52),
              child: const MechanixNavigationBar(title: "Network")
                  .padHorizontal(12)),
          body: SingleChildScrollView(
            padding: const EdgeInsets.all(16.0),
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  MechanixSimpleList(
                      physics: const BouncingScrollPhysics(),
                      listItems: [
                        SimpleListItems(
                          title: 'Wireless',
                          trailing: MechanixSwitch(
                            activeText: 'OFF',
                            inactiveText: 'ON',
                            style: const MechanixSwitchStyle(
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
                            title: utf8.decode(
                                state.connectedNetwork!.nmAccessPoint.ssid),
                            leading: SizedBox(
                                height: 24,
                                width: 24,
                                // iconColor: context.colorScheme.primary,
                                child: getWirelessStrengthIcon(
                                    strength: state.connectedNetwork
                                            ?.nmAccessPoint.strength ??
                                        0,
                                    isActive: true)),
                            trailing: Row(
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                Text(state.wifiState ?? '',
                                    style: const TextStyle(
                                        color: Colors.white, fontSize: 12)),
                                IconButton(
                                  onPressed: () => onInfoTap(
                                      state.connectedNetwork!, context),
                                  icon: const SizedBox(
                                      height: 24,
                                      width: 24,
                                      child: IconWidget(
                                        iconPath: Images.settings,
                                      )),
                                ),
                              ],
                            ),
                          )
                      ]),
                  // wired device
                  if (state.wiredDevice?.enabled != null &&
                      state.wiredDevice!.enabled)
                    MechanixSectionList(
                      physics: const BouncingScrollPhysics(),
                      title: 'Wired',
                      sectionListItems: [
                        SectionListItems(
                          title: state.wiredDevice!.enabled
                              ? "Connected ${state.wiredDevice?.speed} Mb/s"
                              : "Cable unplugged",
                          backgroundColor: Colors.transparent,
                          defaultTrailingIcon: false,
                          trailing: MechanixSwitch(
                            activeText: 'OFF',
                            inactiveText: 'ON',
                            style: const MechanixSwitchStyle(
                              inactiveThumbColor: Color(0xFF989898),
                              inactiveTrackColor: Color(0xFF252525),
                            ),
                            value: state.wiredDevice?.enabled ?? false,
                            onChanged: (val) => {
                              // todo
                            },
                          ),
                        ),
                      ],
                    ),

                  if (state.wifiOn &&
                      !state.availableSavedNetworksLoading &&
                      state.availableSavedNetworks.isEmpty)
                    MechanixSectionList(
                      physics: const BouncingScrollPhysics(),
                      title: 'My Networks',
                      sectionListItems: [
                        SectionListItems(
                          title: '',
                          backgroundColor: Colors.transparent,
                          defaultTrailingIcon: false,
                          leading: const CustomLoader(),
                        ),
                      ],
                    ),
                  if (state.wifiOn && state.availableSavedNetworks.isNotEmpty)
                    MechanixSectionList(
                        physics: const BouncingScrollPhysics(),
                        title: 'My Networks',
                        sectionListItems: getWifiList(
                            context, state.availableSavedNetworks, false)),

                  if (state.wifiOn &&
                      !state.availableOtherNetworksLoading &&
                      state.availableOtherNetworks.isEmpty)
                    MechanixSectionList(
                      physics: const BouncingScrollPhysics(),
                      title: 'Available Networks',
                      sectionListItems: [
                        SectionListItems(
                          title: '',
                          backgroundColor: Colors.transparent,
                          defaultTrailingIcon: false,
                          leading: CustomLoader(),
                        ),
                      ],
                    ),
                  // Padding(
                  //   padding: EdgeInsets.only(right: 25),
                  //   child: CustomLoader(),
                  // )
                  if (state.wifiOn && state.availableOtherNetworks.isNotEmpty)
                    MechanixSectionList.lazy(
                        physics: const BouncingScrollPhysics(),
                        title: 'Available Networks',
                        listBoxConstraints:
                            const BoxConstraints(maxHeight: 280),
                        theme: const MechanixSectionListThemeData(
                          widgetPadding: EdgeInsets.zero,
                        ),
                        sectionListItems: getWifiList(
                            context, state.availableOtherNetworks, true)),
                  const SizedBox(height: 16),
                  const WirelessAdvanceSettings()
                ],
              ),
            ).padTop(8),
          ));
    });
  }
}

void onNetworkTap(AccessPoints item, BuildContext context) {
  context.read<WirelessSettingsBloc>().add(SelectNetwork(item));
  context
      .read<WirelessSettingsBloc>()
      .add(SelectNetworkPoint(item.nmAccessPoint));

  if (item.isSaved && !item.isActive) {
    context
        .read<WirelessSettingsBloc>()
        .add(ConnectSavedNetwork('', item.nmAccessPoint));
  } else if (!item.isSecure && !item.isActive) {
    context
        .read<ConnectNetworkBloc>()
        .add(ConnectToNetwork(item.nmAccessPoint));
  } else {
    final bloc = context.read<ConnectNetworkBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: ConnectSecureNetwork(accessPoint: item.nmAccessPoint),
        ),
      ),
    );
  }
}

void onInfoTap(AccessPoints item, BuildContext context) {
  final bloc = context.read<WirelessSettingsBloc>();

  bloc.add(SelectNetwork(item));
  bloc.add(SelectNetworkPoint(item.nmAccessPoint));

  Navigator.push(
    context,
    MaterialPageRoute(
      builder: (context) => BlocProvider.value(
        value: bloc, // reuse the existing bloc
        child: const NetworkDetails(),
      ),
    ),
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
    BuildContext context, List<AccessPoints> accessPoints, bool showAddOption) {
  final wifi = accessPoints.map((ap) {
    return SectionListItems(
      title: utf8.decode(ap.nmAccessPoint.ssid),
      onTap: () => onNetworkTap(ap, context),
      leading: SizedBox(
        height: 24,
        width: 24,
        child: getWirelessStrengthIcon(strength: ap.nmAccessPoint.strength),
      ),
      defaultTrailingIcon: false,
      trailing: IconButton(
        onPressed: () => onInfoTap(ap, context),
        icon: const SizedBox(
          height: 24,
          width: 24,
          child: IconWidget(iconPath: Images.settings),
        ),
      ),
    );
  }).toList();

  if (showAddOption) {
    wifi.add(
      SectionListItems(
        title: 'Add Wireless',
        defaultTrailingIcon: false,
        onTap: () => Navigator.pushNamed(
            context, AppRoutes.wirelessConnectHiddenNetwork),
        leading: const IconWidget(iconPath: Images.wirelessAdd),
      ),
    );
  }

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
