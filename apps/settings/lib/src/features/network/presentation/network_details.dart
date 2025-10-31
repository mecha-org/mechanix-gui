import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_text_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/commons/styles/color.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

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
        void showDeleteDialog(String networkName) {
          showDialog(
            context: context,
            builder: (dialogContext) {
              return AlertDialog(
                backgroundColor: const Color.fromARGB(255, 54, 54, 54),
                title: const Text('Forget network'),
                content: Column(
                  mainAxisSize: MainAxisSize.min,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                        'You might need to enter password to reconnect to this network again.',
                        style: const TextStyle(fontSize: 18)),
                  ],
                ),
                actions: [
                  CustomTextButton(
                    label: 'Cancel',
                    onPressed: () => Navigator.pop(dialogContext),
                  ),
                  CustomTextButton(
                    label: 'Forget',
                    onPressed: () {
                      Navigator.pop(dialogContext); // pop dialog first
                      context
                          .read<WirelessSettingsBloc>()
                          .add(ForgetNetwork(networkName));
                      Navigator.pop(context);
                    },
                    textColor: dangerColor,
                  ),
                ],
              );
            },
          );
        }

        return Scaffold(
          appBar: PreferredSize(
              preferredSize: const Size.fromHeight(52),
              child: MechanixNavigationBar(
                      title: state.selectedNMAccessPoint != null
                          ? utf8.decode(state.selectedNMAccessPoint!.ssid)
                          : '',
                      actionWidgets: (state.selectedAccessPoint!.isActive ||
                              state.selectedAccessPoint!.isSaved)
                          ? [
                              IconButton(
                                onPressed: () => {
                                  showDeleteDialog(utf8.decode(
                                      state.selectedNMAccessPoint!.ssid)),
                                },
                                style: ButtonStyle(
                                  iconColor:
                                      WidgetStateProperty.all(Colors.white),
                                  backgroundColor: WidgetStateProperty.all(
                                      Color(0xFFB71C1C)),
                                  shape: WidgetStateProperty.all(
                                    RoundedRectangleBorder(
                                      borderRadius: BorderRadius.circular(8.0),
                                    ),
                                  ),
                                  minimumSize:
                                      WidgetStateProperty.all(Size(32, 32)),
                                  fixedSize:
                                      WidgetStateProperty.all(Size(32, 32)),
                                  padding:
                                      WidgetStateProperty.all(EdgeInsets.zero),
                                ),
                                icon: Center(
                                  child: CustomIcon(
                                    icon: Image.asset(Images.trash),
                                    width: 15,
                                    height: 17,
                                  ),
                                ),
                              ).padRight(16)
                            ]
                          : [
                              TextButton.icon(
                                onPressed: () => onNetworkTap(
                                    context, state.selectedAccessPoint),
                                style: ButtonStyle(
                                  backgroundColor:
                                      WidgetStatePropertyAll<Color>(
                                          Color(0xFF044DDF)),
                                  shape: WidgetStatePropertyAll<
                                      RoundedRectangleBorder>(
                                    RoundedRectangleBorder(
                                      borderRadius: BorderRadius.circular(8),
                                      side: BorderSide(),
                                    ),
                                  ),
                                ),
                                icon: IconWidget(
                                  iconColor: Colors.white,
                                  iconPath: Images.addRoundedSquare,
                                  iconHeight: 20,
                                  iconWidth: 20,
                                ),
                                label: Text(
                                  "Join Network",
                                  style: context.textTheme.labelMedium
                                      ?.copyWith(color: Colors.white),
                                ),
                              ).padRight(16)
                            ])
                  .padHorizontal(12)),
          body: SingleChildScrollView(
            padding: const EdgeInsets.all(16.0),
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  Row(
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
    print('Connecting to saved network... Redirect to back');
    Navigator.pop(context);
  } else {
    Navigator.pushNamed(
      context,
      AppRoutes.wirelessConnectSecureNetwork,
      arguments: {'accessPoint': selectedAccessPoint?.nmAccessPoint},
    );
  }
}
