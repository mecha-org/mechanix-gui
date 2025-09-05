import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless_advance_settings.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class WirelessSettings extends StatelessWidget {
  const WirelessSettings({super.key});

  void _backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
        builder: (context, state) {
      return Scaffold(
          appBar: CustomAppBar(
            title: "Network",
            leftIcon: Image.asset(Images.back),
            leftIconOnTap: () => _backNavigation(context),
          ),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  MechanixSimpleList(isDividerRequired: true, listItems: [
                    SimpleListItems(
                      title: 'Wireless',
                      trailing: CustomToggle(
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
                            child: IconWidget(
                              iconPath: Images.wifiConnected,
                              isActive: true,
                            )),
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
                  // const SizedBox(
                  //   height: 20,
                  // ),
                  // if (state.wifiOn && state.connectedNetwork != null)
                  //   MechanixSectionList(
                  //     sectionListItems: [
                  //       SectionListItems(
                  //         onTap: () =>
                  //             onInfoTap(state.connectedNetwork!, context),
                  //         title: utf8.decode(
                  //             state.connectedNetwork!.nmAccessPoint.ssid),
                  //         leading: SizedBox(
                  //             height: 24,
                  //             width: 24,
                  //             child: IconWidget(
                  //               iconPath: Images.wifiConnected,
                  //               isActive: true,
                  //             )),
                  //         trailing: IconButton(
                  //             onPressed: () => onNetworkTap(
                  //                 state.connectedNetwork!, context),
                  //             icon: SizedBox(
                  //                 height: 24,
                  //                 width: 24,
                  //                 child: IconWidget(
                  //                   iconPath: Images.settings,
                  //                 ))),
                  //       )
                  //     ],
                  //   ),
                  // if (state.wifiOn)
                  //   Padding(
                  //     padding: EdgeInsets.only(
                  //       bottom: 5,
                  //       top: 15,
                  //     ),
                  //     child: Row(
                  //       mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  //       children: [
                  //         Text('Available Networks', style: baseHeaderStyle),
                  //         // TextButton(
                  //         //     style: buttonStyle,
                  //         //     onPressed: () => _refreshWifi(context),
                  //         //     child: Text("Refresh")),
                  //       ],
                  //     ),
                  //   ),
                  if (state.wifiOn && !state.loading && state.networks.isEmpty)
                    // const Center(child: Text("No networks found")),
                    MechanixSectionList(
                        title: 'Available Networks',
                        sectionListItems: [
                          SectionListItems(
                              title: 'No networks found',
                              backgroundColor: Colors.transparent,
                              defaultTrailing: false),
                        ]),
                  if (state.wifiOn && state.networks.isNotEmpty)
                    MechanixSectionList(
                        title: 'Available Networks',
                        sectionListItems: getWifiList(context, state.networks)),
                  // Container(
                  //     child: ListView.builder(
                  //   shrinkWrap: true,
                  //   physics: NeverScrollableScrollPhysics(),
                  //   itemCount: state.networks.length,
                  //   itemBuilder: (context, index) {
                  //     return MechanixSimpleList(
                  //         listItems: getWifiList(context, state.networks));
                  //   },
                  // )),
                  // const SizedBox(
                  //   height: 10,
                  // ),
                  // if (state.wifiOn)
                  //   Row(
                  //     mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  //     children: [
                  //       const Text("Other networks",
                  //           style: secondaryHeaderStyle),
                  //       TextButton(
                  //         onPressed: () => _backNavigation(context),
                  //         style: buttonStyle,
                  //         child: const Text(
                  //           "Add a network",
                  //           style: baseHeaderStyle,
                  //         ),
                  //       )
                  //     ],
                  //   ),
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

void _refreshWifi(BuildContext context) {
  context.read<WirelessSettingsBloc>().add(LoadNetworks());
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
        child: IconWidget(iconPath: Images.wifiConnected),
      ),
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
        onTap: () => Navigator.pushNamed(
            context, AppRoutes.wirelessConnectUnknownNetwork),
        leading: IconWidget(iconPath: Images.wirelessAdd),
        trailing: IconWidget(
          iconWidth: 10,
          iconHeight: 17,
          iconPath: Images.rightIconArrow,
        )),
  );

  return wifi;
}
