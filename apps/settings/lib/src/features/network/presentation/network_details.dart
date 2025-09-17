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
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
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

  void onForgetPressed(String ssid) {
    context.read<WirelessSettingsBloc>().add(ForgetNetwork(ssid));
    Navigator.pop(context);
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
          appBar: MechanixNavigationBar(
              title: ssid,
              actionWidgets: networkDetails.isActive
                  ? [
                      IconButton(
                        onPressed: () => onForgetPressed(ssid),
                        style: ButtonStyle(
                          iconColor: WidgetStateProperty.all(Colors.white),
                          backgroundColor:
                              WidgetStateProperty.all(Color(0xFFB71C1C)),
                          shape: WidgetStateProperty.all(
                            RoundedRectangleBorder(
                              borderRadius: BorderRadius.circular(8.0),
                            ),
                          ),
                          minimumSize: WidgetStateProperty.all(Size(32, 32)),
                          fixedSize: WidgetStateProperty.all(Size(32, 32)),
                          padding: WidgetStateProperty.all(EdgeInsets.zero),
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
                  : !networkDetails.isActive
                      ? [
                          TextButton.icon(
                            onPressed: () => {
                              onNetworkTap(context, networkDetails.isSaved,
                                  networkDetails)
                            },
                            style: ButtonStyle(
                              backgroundColor: WidgetStatePropertyAll<Color>(
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
                        ]
                      : null),
          body: SingleChildScrollView(
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
                    title: 'About The Network',
                    theme: MechanixSectionListThemeData(
                        widgetPadding: Spacing.only(top: 8, bottom: 40)),
                    sectionListItems: [
                      SectionListItems(
                        title: 'Private Wifi Address',
                        trailing:
                            CustomTrailingText(title: 'Fixed').padRight(8),
                      ),
                      SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Private Wifi Address',
                          trailing: CustomTrailingText(
                            title: networkDetails.nmAccessPoint.hwAddress,
                          )),
                    ],
                  ),

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
                        trailing:
                            CustomTrailingText(title: 'Automatic').padRight(8),
                      ),
                      if (networkDetails.isActive)
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'IP Address',
                          trailing: CustomTrailingText(
                            title:
                                '${networkDetails.ip4Config?.addressData.first['address']}',
                          ),
                        ),
                      if (networkDetails.isActive)
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Subnet Mask',
                          trailing: CustomTrailingText(
                            title:
                                '${networkDetails.ip4Config?.addressData.first['prefix']}',
                          ),
                        ),
                      if (networkDetails.isActive)
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Router',
                          trailing: CustomTrailingText(
                            title:
                                '${networkDetails.ip4Config?.routeData.first['dest']}',
                          ),
                        ),
                    ],
                  ),

                  if (networkDetails.isActive)
                    MechanixSectionList(
                      title: 'IPV6 Address',
                      sectionListItems: [
                        SectionListItems(
                          title: 'IP Address',
                          trailing: CustomTrailingText(
                                  title:
                                      '${networkDetails.ip6Config?.addressData.length} Addresses')
                              .padRight(8),
                        ),
                        SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'Router',
                          trailing: CustomTrailingText(
                                  title:
                                      '${networkDetails.ip6Config?.routeData.first['dest']}')
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
