import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
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
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';
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
        body: SingleChildScrollView(
          physics: const BouncingScrollPhysics(),
          child: ContainerWidget(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const CustomTitle(title: "Network"),
                MechanixSimpleList(
                  physics: const BouncingScrollPhysics(),
                  isDividerRequired: false,
                  listItems: [
                    SimpleListItems(
                      title: 'Wireless',
                      titleTextStyle: TextStyle(fontWeight: FontWeight.w700),
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
                        // onTap: () =>
                        //     onInfoTap(state.connectedNetwork!, context),
                        title: utf8
                            .decode(state.connectedNetwork!.nmAccessPoint.ssid),
                        titleTextStyle: context.textTheme.labelMedium
                            ?.copyWith(color: context.primary),
                        leading: IconWidget(
                          iconPath: Images.wifi,
                          iconColor: context.primary,
                        ),
                        trailing: Row(
                          children: [
                            IconWidget(
                              iconPath: Images.circularCheckIcon,
                              iconColor: context.primary,
                              isActive: true,
                              iconHeight: 19,
                              iconWidth: 19,
                              activeIconColor: context.primary,
                            ).padRight(8),
                            IconButton(
                              onPressed: () =>
                                  onInfoTap(state.connectedNetwork!, context),
                              icon: Row(
                                children: [
                                  IconWidget(
                                    iconPath: Images.settings,
                                    iconColor: context.onSurface,
                                  )
                                ],
                              ),
                            ),
                          ],
                        ),
                      ),
                  ],
                ),
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
                        leading: const CustomLoader(),
                      ),
                    ],
                  ),
                if (state.wifiOn && state.availableOtherNetworks.isNotEmpty)
                  MechanixSectionList(
                    physics: const BouncingScrollPhysics(),
                    title: 'Available Networks',
                    theme: const MechanixSectionListThemeData(
                      widgetPadding: EdgeInsets.zero,
                    ),
                    sectionListItems: getWifiList(
                        context, state.availableOtherNetworks, true),
                  ),
                const WirelessAdvanceSettings().padTop(36)
              ],
            ),
          ).padTop(8),
        ),
        bottomNavigationBar: MechanixBottomBar(
          leadingWidget: [context.backButton],
        ),
      );
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
      // onTap: () => onNetworkTap(ap, context),
      leading: const IconWidget(iconPath: Images.wifi),
      defaultTrailingIcon: false,
      trailing: IconButton(
        onPressed: () => onInfoTap(ap, context),
        icon: SizedBox(
          height: 24,
          width: 24,
          child: IconWidget(
            iconPath: Images.settings,
            iconColor: context.onSurfaceVariant,
          ),
        ),
      ),
    );
  }).toList();

  if (showAddOption) {
    wifi.add(
      SectionListItems.leadingIcon(
        title: 'Add Wireless',
        titleTextStyle:
            context.textTheme.labelMedium?.copyWith(color: context.primary),
        defaultTrailingIcon: false,
        onTap: () => Navigator.pushNamed(
            context, AppRoutes.wirelessConnectHiddenNetwork),
        // leading: const IconWidget(iconPath: Images.wirelessAdd),
        iconPath: Images.wirelessAdd,
        isActive: true,
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
