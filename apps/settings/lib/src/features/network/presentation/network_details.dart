import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_text_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_toggle.dart';
import 'package:mechanix_settings/src/commons/styles/color.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/models/access_points.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';

class NetworkDetails extends StatefulWidget {
  const NetworkDetails({super.key});

  @override
  State<NetworkDetails> createState() => _NetworkDetailsState();
}

class _NetworkDetailsState extends State<NetworkDetails> {
  void _backNavigation() {
    Navigator.pop(context);
  }

  void _showDeleteDialog(String ssid) {
    final dialog = AlertDialog(
      backgroundColor: const Color.fromARGB(255, 54, 54, 54),
      title: const Text('Forget network'),
      content: const Text(
        'This device will no longer connect to this network automatically. You may need to enter the password next time.',
        style: TextStyle(fontSize: 18),
      ),
      actions: <Widget>[
        CustomTextButton(
          label: 'Cancel',
          onPressed: () {
            Navigator.of(context).pop();
          },
        ),
        CustomTextButton(
          label: 'Forget',
          onPressed: () {
            context.read<WirelessSettingsBloc>().add(
                  ForgetNetwork(ssid),
                );
            Navigator.of(context).pop();
            Navigator.pop(context);
          },
          textColor: selectColor,
        ),
      ],
    );

    showDialog(
      context: context,
      barrierDismissible: false,
      builder: (context) => dialog,
    );
  }

  @override
  Widget build(BuildContext context) {
    final args =
        ModalRoute.of(context)!.settings.arguments as Map<String, AccessPoints>;
    final networkDetails = args["networkDetails"] as AccessPoints;

    final accessPoint = networkDetails.nmAccessPoint;
    final ssid = utf8.decode(accessPoint.ssid);
    return BlocProvider(
        create: (context) => WirelessSettingsBloc(
              wifiRepository: context.read<WifiRepository>(),
            ),
        child: Scaffold(
          appBar: CustomAppBar(
              title: ssid,
              leftIcon: Image.asset(Images.back),
              leftIconOnTap: _backNavigation,
              customChild: networkDetails.isActive
                  ? TextButton.icon(
                      onPressed: () => _showDeleteDialog(ssid),
                      style: ButtonStyle(
                        backgroundColor: const WidgetStatePropertyAll<Color>(
                            Color(0xFF3B3B3B)),
                        padding: const WidgetStatePropertyAll<EdgeInsets>(
                            EdgeInsets.symmetric(horizontal: 8)),
                        shape: WidgetStatePropertyAll<RoundedRectangleBorder>(
                          RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(8),
                            side: const BorderSide(),
                          ),
                        ),
                      ),
                      icon: CustomIcon(
                        icon: Image.asset(Images.delete),
                        height: 20,
                        width: 20,
                      ),
                      label: Text(
                        "Forget Network",
                        style: baseHeaderStyle,
                      ),
                    )
                  : null),
          body: SingleChildScrollView(
            child: ContainerWidget(
              child: Column(
                children: [
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    mainAxisAlignment: MainAxisAlignment.spaceBetween,
                    children: [
                      Text(
                        ssid,
                        style: baseHeaderStyle.copyWith(fontSize: 20),
                      ),
                      if (!networkDetails.isActive)
                        TextButton.icon(
                          onPressed: () => {
                            onNetworkTap(
                                context, networkDetails.isSaved, networkDetails)
                          },
                          style: ButtonStyle(
                            backgroundColor: WidgetStatePropertyAll<Color>(
                                Color(0xFF044DDF)),
                            shape:
                                WidgetStatePropertyAll<RoundedRectangleBorder>(
                              RoundedRectangleBorder(
                                borderRadius: BorderRadius.circular(8),
                                side: BorderSide(),
                              ),
                            ),
                          ),
                          icon: const Icon(
                            Icons.add,
                            color: Color(0xFFF0F0F0),
                          ),
                          label: const Text(
                            "Join Network",
                            style: baseHeaderStyle,
                          ),
                        )
                    ],
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
                    title: 'About The Network',
                    sectionListItems: [
                      SectionListItems(
                        title: 'Private Wifi Address',
                        trailing: Row(
                          children: [
                            Text(
                              'Fixed',
                              style: context.textTheme.labelLarge,
                            ).padRight(8),
                            IconWidget(
                              iconWidth: 9,
                              iconHeight: 18,
                              iconPath: Images.rightIconArrow,
                            ),
                          ],
                        ),
                      ),
                      SectionListItems(
                          title: 'Private Wifi Address',
                          trailing: Text(networkDetails.nmAccessPoint.hwAddress,
                              style: context.textTheme.labelLarge)),
                    ],
                  ),

                  if (networkDetails.isActive)
                    MechanixSimpleList(listItems: [
                      SimpleListItems(
                          title: 'Limit IP address tracking',
                          trailing:
                              CustomToggle(value: true, onChanged: (v) {}))
                    ]),

                  MechanixSectionList(
                    title: 'IPV4 Address',
                    sectionListItems: [
                      SectionListItems(
                        title: 'Configure IP',
                        onTap: () {
                          Navigator.pushNamed(
                            context,
                            AppRoutes.ipv4Address,
                          );
                        },
                        trailing: Row(
                          children: [
                            Text('Automatic',
                                    style: context.textTheme.labelLarge)
                                .padRight(8),
                            IconWidget(
                              iconWidth: 9,
                              iconHeight: 18,
                              iconPath: Images.rightIconArrow,
                            ),
                          ],
                        ),
                      ),
                      if (networkDetails.isActive)
                        SectionListItems(
                          title: 'IP Address',
                          trailing: Text(networkDetails.nmAccessPoint.hwAddress,
                              style: context.textTheme.labelLarge),
                        ),
                      if (networkDetails.isActive)
                        SectionListItems(
                          title: 'Subnet Mask',
                          trailing: Text(networkDetails.nmAccessPoint.hwAddress,
                              style: context.textTheme.labelLarge),
                        ),
                      if (networkDetails.isActive)
                        SectionListItems(
                          title: 'Router',
                          trailing: Text(networkDetails.nmAccessPoint.hwAddress,
                              style: context.textTheme.labelLarge),
                        ),
                    ],
                  ),

                  if (networkDetails.isActive)
                    MechanixSectionList(
                      title: 'IPV6 Address',
                      sectionListItems: [
                        SectionListItems(
                          title: 'IP Address',
                          trailing: Row(
                            children: [
                              Text('2 Addresses',
                                      style: context.textTheme.labelLarge)
                                  .padRight(8),
                              IconWidget(
                                iconWidth: 9,
                                iconHeight: 18,
                                iconPath: Images.rightIconArrow,
                              ),
                            ],
                          ),
                        ),
                        SectionListItems(
                          title: 'Router',
                          trailing: Text('Automatic',
                                  style: context.textTheme.labelLarge)
                              .padRight(8),
                        ),
                      ],
                    ),

                  MechanixSectionList(
                    title: 'DNS',
                    sectionListItems: [
                      SectionListItems(
                        title: 'Configure DNS',
                        onTap: () {
                          Navigator.pushNamed(
                            context,
                            AppRoutes.configureDNS,
                          );
                        },
                        trailing: Row(
                          children: [
                            Text('Automatic',
                                    style: context.textTheme.labelLarge)
                                .padRight(8),
                            IconWidget(
                              iconWidth: 9,
                              iconHeight: 18,
                              iconPath: Images.rightIconArrow,
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),

                  MechanixSectionList(
                    title: 'HTTP Proxy',
                    sectionListItems: [
                      SectionListItems(
                        title: 'Configure Proxy',
                        onTap: () {
                          Navigator.pushNamed(
                            context,
                            AppRoutes.configureProxy,
                            // arguments: {'networkDetails': item},
                          );
                        },
                        trailing: Row(
                          children: [
                            Text('OFF', style: context.textTheme.labelLarge)
                                .padRight(8),
                            IconWidget(
                              iconWidth: 9,
                              iconHeight: 18,
                              iconPath: Images.rightIconArrow,
                            ),
                          ],
                        ),
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
              ),
            ),
          ),
        ));
  }
}

Future<void> onNetworkTap(
    BuildContext context, bool isSaved, AccessPoints accessPoint) async {
  if (isSaved) {
    context
        .read<WirelessSettingsBloc>()
        .add(ConnectSavedNetwork('', accessPoint.nmAccessPoint));
    Navigator.pop(context);
  } else {
    Navigator.pushNamed(
      context,
      AppRoutes.wirelessConnectSecureNetwork,
      arguments: {'accessPoint': accessPoint.nmAccessPoint},
    );
  }
}
