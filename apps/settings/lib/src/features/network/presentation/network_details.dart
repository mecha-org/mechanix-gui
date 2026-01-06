import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:mechanix_settings/src/features/network/presentation/configure_dns.dart';
import 'package:mechanix_settings/src/features/network/presentation/configure_proxy.dart';
import 'package:mechanix_settings/src/features/network/presentation/connect_secure_network.dart';
import 'package:mechanix_settings/src/features/network/presentation/ipv4_address.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/connect_network.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/bottom_bar/bottom_bar_button_type.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class NetworkDetails extends StatefulWidget {
  const NetworkDetails({super.key});

  @override
  State<NetworkDetails> createState() => _NetworkDetailsState();
}

class _NetworkDetailsState extends State<NetworkDetails> {
  void onForgetPressed(String ssid) {
    context.read<WirelessSettingsBloc>().add(ForgetNetwork(ssid));
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
      builder: (context, state) {
        return Scaffold(
          body: SingleChildScrollView(
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  CustomTitle(
                    title: state.selectedNMAccessPoint != null
                        ? utf8.decode(state.selectedNMAccessPoint!.ssid)
                        : '',
                  ),
                  const Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [],
                  ),

                  // const SizedBox(height: 20),
                  // CustomLabelValue(
                  //   title: "Network SSID",
                  //   value: ssid,
                  // ),
                  // CustomLabelValue(
                  //   title: "Network Type",
                  //   value: ssid,
                  // ),
                  // CustomLabelValue(
                  //   title: "Passphrase",
                  //   value: accessPoint.rsnFlags.isNotEmpty ? 'WPA/WPA2' : 'None',
                  // ),
                  // CustomLabelValue(
                  //   title: "Frequency",
                  //   value: '${accessPoint.frequency} MHz',
                  // ),

                  MechanixSectionList(
                    physics: const BouncingScrollPhysics(),
                    title: 'About The Network',
                    theme: MechanixSectionListThemeData(
                        widgetPadding: Spacing.only(top: 8, bottom: 40)),
                    sectionListItems: [
                      SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Private Wifi Address',
                          trailing: CustomTrailingText(
                            title: state.selectedNMAccessPoint?.hwAddress ?? '',
                          )),
                    ],
                  ),

                  if (state.selectedAccessPoint != null &&
                      state.selectedNMAccessPoint != null)
                    MechanixSectionList(
                      sectionListItems: [
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Signal Strength',
                          trailing: CustomTrailingText(
                                  title:
                                      '${state.selectedAccessPoint?.nmAccessPoint.strength} %')
                              .padRight(8),
                        ),
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Supported Frequency',
                          trailing: CustomTrailingText(
                                  title:
                                      '${state.selectedAccessPoint?.nmAccessPoint.frequency} MHz')
                              .padRight(8),
                        ),
                      ],
                    ),

                  if (state.selectedAccessPoint != null &&
                          state.selectedNMAccessPoint != null &&
                          state.selectedAccessPoint!.isActive ||
                      state.selectedAccessPoint!.isSaved)
                    MechanixSectionList(
                      title: 'IPV4 Address',
                      physics: const BouncingScrollPhysics(),
                      sectionListItems: [
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Configure IP',
                          onTap: () {
                            final bloc = context.read<WirelessSettingsBloc>();

                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (context) => BlocProvider.value(
                                  value: bloc,
                                  child: const Ipv4AddressWidget(),
                                ),
                              ),
                            );
                          },
                          trailing:
                              const CustomTrailingText(title: 'Automatic'),
                        ),
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'IP Address',
                          trailing: CustomTrailingText(
                            title:
                                '${state.selectedAccessPoint != null ? state.selectedAccessPoint?.ip4Config?.addressData.first['address'] : ''}',
                          ),
                        ),
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Subnet Mask',
                          trailing: CustomTrailingText(
                            title:
                                '${state.selectedAccessPoint != null ? state.selectedAccessPoint?.ip4Config?.addressData.first['prefix'] : ''}',
                          ),
                        ),
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Router',
                          trailing: CustomTrailingText(
                            title:
                                '${state.selectedAccessPoint != null ? state.selectedAccessPoint?.ip4Config?.routeData.first['dest'] : ''}',
                          ),
                        ),
                      ],
                    ),

                  if (state.selectedAccessPoint != null &&
                      state.selectedNMAccessPoint != null &&
                      state.selectedAccessPoint!.isActive)
                    MechanixSectionList(
                      title: 'IPV6 Address',
                      physics: const BouncingScrollPhysics(),
                      sectionListItems: [
                        SectionListItems(
                          title: 'IP Address',
                          trailing: CustomTrailingText(
                                  title:
                                      '${state.selectedAccessPoint != null ? state.selectedAccessPoint?.ip6Config?.addressData.length : ''} Addresses')
                              .padRight(8),
                        ),
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Router',
                          trailing: CustomTrailingText(
                                  title:
                                      '${state.selectedAccessPoint != null ? state.selectedAccessPoint?.ip6Config?.routeData.first['dest'] : ''}')
                              .padRight(8),
                        ),
                      ],
                    ),
                  if (state.selectedAccessPoint != null &&
                      state.selectedNMAccessPoint != null &&
                      state.selectedAccessPoint!.isActive)
                    MechanixSectionList(
                      title: 'DNS',
                      // physics: const BouncingScrollPhysics(),

                      sectionListItems: [
                        SectionListItems(
                          onTap: () {
                            final bloc = context.read<WirelessSettingsBloc>();

                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (context) => BlocProvider.value(
                                  value: bloc,
                                  child: const ConfigureDnsWidget(),
                                ),
                              ),
                            );
                          },
                          title: 'Configure DNS',
                          trailing: const CustomTrailingText(title: 'Automatic')
                              .padRight(8),
                        ),
                      ],
                    ),

                  if (state.selectedAccessPoint != null &&
                      state.selectedNMAccessPoint != null &&
                      state.selectedAccessPoint!.isActive)
                    MechanixSectionList(
                      title: 'HTTP Proxy',
                      // physics: const BouncingScrollPhysics(),

                      sectionListItems: [
                        SectionListItems(
                          onTap: () {
                            final bloc = context.read<WirelessSettingsBloc>();

                            Navigator.push(
                              context,
                              MaterialPageRoute(
                                builder: (context) => BlocProvider.value(
                                  value: bloc,
                                  child: const ConfigureProxyWidget(),
                                ),
                              ),
                            );
                          },
                          title: 'Configure Proxy',
                          trailing: const CustomTrailingText(title: 'Off')
                              .padRight(8),
                        ),
                      ],
                    ),

                  const SizedBox(
                    height: 20,
                  ),
                  // CustomLabelValue(
                  //   title: "IP Address",
                  //   value: accessPoint.,
                  // ),
                  // CustomLabelValue(
                  //   title: "Subnet Mask",
                  //   value: '${accessPoint.frequency} MHz',
                  // ),
                  // CustomLabelValue(
                  //   title: "Gateway",
                  //   value: '${accessPoint.frequency} MHz',
                  // ),

                  // LabelValueListRow(
                  //     title: 'Frequency', value: '${accessPoint.frequency} MHz'),
                  // LabelValueListRow(
                  //     title: 'Hw Address', value: accessPoint.hwAddress),
                  // LabelValueListRow(
                  //   title: 'Security',
                  //   value: accessPoint.rsnFlags.isNotEmpty ? 'WPA/WPA2' : 'None',
                  // ),
                ],
              ).padTop(8),
            ),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
            centerWidget: [
              BottomBarButton.widget(
                widget: TextButton.icon(
                  onPressed: () {
                    onNetworkTap(context, state.selectedAccessPoint);
                    Navigator.push(
                      context,
                      MaterialPageRoute(
                          builder: (context) => ConnectNetwork(
                              accessPoint: state.selectedNMAccessPoint)),
                    );
                  },
                  icon: IconWidget(
                    iconPath: Images.addRoundedSquare,
                    iconHeight: 15,
                    iconWidth: 15,
                    boxWidth: 20,
                    boxHeight: 20,
                    iconColor: Theme.of(context)
                        .textButtonTheme
                        .style
                        ?.iconColor
                        ?.resolve({}),
                  ),
                  label: const Text("Join Network"),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

Future<void> onNetworkTap(
    BuildContext context, AccessPoints? selectedAccessPoint) async {
  if (selectedAccessPoint != null && selectedAccessPoint.isSaved) {
    context
        .read<WirelessSettingsBloc>()
        .add(ConnectSavedNetwork('', selectedAccessPoint.nmAccessPoint));
    Navigator.pop(context);
  } else {
    final bloc = context.read<ConnectNetworkBloc>();

    Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: ConnectSecureNetwork(
              accessPoint: selectedAccessPoint?.nmAccessPoint),
        ),
      ),
    );
  }
}
