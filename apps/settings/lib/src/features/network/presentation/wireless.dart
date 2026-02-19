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
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:mechanix_settings/src/features/network/presentation/network_details.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/available_networks.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/saved_networks.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_strength_icon.dart';
import 'package:mechanix_settings/src/features/network/presentation/wireless_advance_settings.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/list_items/mechanix_simple_list_theme.dart';
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
  final ScrollController _scrollController = ScrollController();

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  void scrollToTop() {
    if (_scrollController.hasClients) {
      _scrollController.animateTo(
        0,
        duration: const Duration(milliseconds: 300),
        curve: Curves.easeOut,
      );
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
          child: NetworkDetails(scrollToTop: scrollToTop),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SingleChildScrollView(
        controller: _scrollController,
        physics: const BouncingScrollPhysics(),
        child: ContainerWidget(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const CustomTitle(title: "Network"),
              BlocSelector<
                  WirelessSettingsBloc,
                  WirelessSettingsState,
                  ({
                    bool wifiOn,
                    NetworkManagerDeviceState? deviceState,
                    AccessPoints? connectedNetwork,
                    ActivatingNetwork? activatingNetwork
                  })>(
                selector: (state) => (
                  wifiOn: state.wifiOn,
                  deviceState: state.deviceState,
                  connectedNetwork: state.connectedNetwork,
                  activatingNetwork: state.activatingNetwork,
                ),
                builder: (context, data) {
                  final List<SimpleListItems> items = [];

                  items.add(
                    SimpleListItems(
                      title: 'Wireless',
                      titleTextStyle:
                          const TextStyle(fontWeight: FontWeight.w700),
                      onTap: () {
                        context
                            .read<WirelessSettingsBloc>()
                            .add(ToggleWifi(!data.wifiOn));
                      },
                      trailing: MechanixSwitch(
                        activeText: 'OFF',
                        inactiveText: 'ON',
                        style: MechanixSwitchStyle(
                          activeTrackColor: context.secondaryContainer,
                          inactiveTrackColor: context.secondaryContainer,
                        ),
                        value: data.wifiOn,
                        onChanged: (val) {
                          context
                              .read<WirelessSettingsBloc>()
                              .add(ToggleWifi(val));
                        },
                      ),
                    ),
                  );

                  if (data.activatingNetwork != null &&
                      data.activatingNetwork!.ssid.isNotEmpty &&
                      !data.activatingNetwork!.isActivate) {
                    final network = data.activatingNetwork?.accessPoint;

                    if (network != null) {
                      items.add(
                        SimpleListItems(
                          title: utf8.decode(network.nmAccessPoint.ssid),
                          titleTextStyle: context.textTheme.labelMedium,
                          leading: getWirelessStrengthIcon(
                            strength: network.nmAccessPoint.strength,
                            isSecure: network.isSecure,
                            isActive: network.isActive,
                          ),
                          trailing: Row(
                            children: [
                              const CustomLoader(),
                              IconButton(
                                onPressed: () => onInfoTap(network, context),
                                icon: IconWidget(
                                  iconPath: Images.settings,
                                  iconColor: context.onSurfaceVariant,
                                ),
                              ).padLeft(8),
                            ],
                          ),
                        ),
                      );
                    }
                  } else if (data.wifiOn &&
                      data.connectedNetwork != null &&
                      data.deviceState == NetworkManagerDeviceState.activated &&
                      data.connectedNetwork!.isActive) {
                    final network = data.connectedNetwork!;

                    items.add(
                      SimpleListItems(
                        title: utf8.decode(
                          network.nmAccessPoint.ssid,
                        ),
                        titleTextStyle: context.textTheme.labelMedium
                            ?.copyWith(color: context.primary),
                        leading: getWirelessStrengthIcon(
                          strength: network.nmAccessPoint.strength,
                          isSecure: network.isSecure,
                          isActive: network.isActive,
                        ),
                        trailing: Row(
                          children: [
                            Container(
                              margin: const EdgeInsets.only(right: 8),
                              child: Image.asset(
                                Images.circularCheckIcon,
                                width: 19,
                                height: 19,
                              ),
                            ),
                            IconButton(
                              onPressed: () => onInfoTap(network, context),
                              icon: IconWidget(
                                iconPath: Images.settings,
                                iconColor: context.onSurface,
                              ),
                            ),
                          ],
                        ),
                      ),
                    );
                  }

                  return MechanixSimpleList(
                    physics: const BouncingScrollPhysics(),
                    isDividerRequired: false,
                    theme: const MechanixSimpleListThemeData(
                      widgetMargin: EdgeInsets.zero,
                    ),
                    listItems: items,
                  );
                },
              ),
              BlocSelector<WirelessSettingsBloc, WirelessSettingsState,
                  ({bool wifiOn, bool loading, List<AccessPoints> list})>(
                selector: (state) => (
                  wifiOn: state.wifiOn,
                  loading: state.availableOtherNetworksLoading,
                  list: state.availableSavedNetworks,
                ),
                builder: (context, data) {
                  return AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      switchInCurve: Curves.easeOut,
                      switchOutCurve: Curves.easeIn,
                      transitionBuilder: (child, animation) {
                        return FadeTransition(
                          opacity: animation,
                          child: SizeTransition(
                            sizeFactor: animation,
                            axisAlignment: -1,
                            child: child,
                          ),
                        );
                      },
                      child: data.wifiOn && data.list.isNotEmpty
                          ? SavedNetworks(scrollToTop: scrollToTop)
                          : null);
                },
              ),
              BlocSelector<WirelessSettingsBloc, WirelessSettingsState,
                  ({bool wifiOn, bool loading, List<AccessPoints> list})>(
                selector: (state) => (
                  wifiOn: state.wifiOn,
                  loading: state.availableOtherNetworksLoading,
                  list: state.availableOtherNetworks,
                ),
                builder: (context, data) {
                  return AnimatedSwitcher(
                      duration: const Duration(milliseconds: 300),
                      switchInCurve: Curves.easeOut,
                      switchOutCurve: Curves.easeIn,
                      transitionBuilder: (child, animation) {
                        return FadeTransition(
                          opacity: animation,
                          child: SizeTransition(
                            sizeFactor: animation,
                            axisAlignment: -1,
                            child: child,
                          ),
                        );
                      },
                      child: (data.wifiOn && data.loading)
                          ? MechanixSectionList(
                              title: 'Available networks',
                              theme: const MechanixSectionListThemeData(
                                widgetPadding: EdgeInsets.zero,
                              ),
                              sectionListItems: [
                                SectionListItems(
                                  title: '',
                                  backgroundColor: Colors.transparent,
                                  defaultTrailingIcon: false,
                                  leading: const CustomLoader(),
                                ),
                              ],
                            ).padTop(36)
                          : data.list.isNotEmpty
                              ? AvailableNetworks(scrollToTop: scrollToTop)
                              : null);
                },
              ),
              const WirelessAdvanceSettings().padTop(36),
            ],
          ),
        ).padTop(8),
      ),
      bottomNavigationBar: MechanixBottomBar(
        leadingWidget: [context.backButton],
        anchorWidget: [
          BottomBarButton.widget(
            widget: IconButton(
              onPressed: () {
                context.read<WirelessSettingsBloc>().add(RefreshWifiList());
              },
              icon: const IconWidget(
                iconPath: Images.arrowCounterClockWise,
                boxWidth: 48,
                boxHeight: 48,
                iconHeight: 21,
                iconWidth: 21,
              ),
            ).padRight(8),
          ),
        ],
      ),
    );
  }
}
