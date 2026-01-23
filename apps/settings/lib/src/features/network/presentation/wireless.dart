import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_loader.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/models/security_protocols.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/available_networks.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/saved_networks.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_strength_icon.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless_advance_settings.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/bottom_bar/mechanix_bottom_bar_theme.dart';
import 'package:widgets/widgets/list_items/simple_list_items_type.dart';
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
                      titleTextStyle:
                          const TextStyle(fontWeight: FontWeight.w700),
                      onTap: () {
                        context
                            .read<WirelessSettingsBloc>()
                            .add(ToggleWifi(!state.wifiOn));
                      },
                      trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        style: MechanixSwitchStyle(
                          activeTrackColor: context.secondaryContainer,
                          inactiveTrackColor: context.secondaryContainer,
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
                        leading: getWirelessStrengthIcon(
                          strength:
                              state.connectedNetwork!.nmAccessPoint.strength,
                          isSecure: state.connectedNetwork!.isSecure,
                          isActive: state.connectedNetwork!.isActive,
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
                              icon: IconWidget(
                                iconPath: Images.settings,
                                iconColor: context.onSurface,
                              ),
                            ),
                          ],
                        ),
                      ),
                  ],
                ),
                if (state.wifiOn &&
                    !state.availableSavedNetworksLoading &&
                    state.availableOtherNetworks.isEmpty)
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
                  const SavedNetworks(),
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
                  const AvailableNetworks(),
                const WirelessAdvanceSettings().padTop(36)
              ],
            ),
          ).padTop(8),
        ),
        bottomNavigationBar: MechanixBottomBar(
          leadingWidget: [context.backButton],
          anchorWidget: [
            BottomBarButton(
              onPressed: () {
                context.read<WirelessSettingsBloc>().add(RefreshWifiList());
              },
              iconTheme: const MechanixBottomBarIconThemeData(
                hoverColor: Colors.redAccent,
                buttonSize: Size(44, 44),
                iconBoxSize: Size(28, 28),
                iconSize: Size(21.88, 21.45),
                buttonMargin: EdgeInsets.only(right: 12),
              ),
              iconPath: Images.arrowCounterClockWise,
            )
          ],
        ),
      );
    });
  }
}

void onInfoTap(AccessPoints item, BuildContext context) {
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
        child: const NetworkDetails(),
      ),
    ),
  );
}
