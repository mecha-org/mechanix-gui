import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_icon.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_event.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/sectionList/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/sectionList/section_list_items_type.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_text_button.dart';
import 'package:mechanix_settings/src/commons/styles/color.dart';

class SavedNetworkDetails extends StatefulWidget {
  const SavedNetworkDetails({super.key});

  @override
  State<SavedNetworkDetails> createState() => _SavedNetworkDetailsState();
}

class _SavedNetworkDetailsState extends State<SavedNetworkDetails> {
  void onForgetPressed(String ssid) async {
    context.read<WirelessSettingsBloc>().add(ForgetNetwork(ssid));
    print('Forgetting network...routing back');
    Navigator.pop(context);
  }

  @override
  Widget build(BuildContext context) {
    final args =
        ModalRoute.of(context)!.settings.arguments as Map<String, dynamic>;
    final SavedWirelessNetwork network =
        args['network'] as SavedWirelessNetwork;

    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
      builder: (context, state) {
        final ssid = network.ssid;
        final security = network.security;

        // todo : add ?
         void showDeleteDialog(String networkName) {
          showDialog(
            context: context,
            builder: (dialogContext) {
              // This context does NOT have BlocProvider!
              return AlertDialog(
                backgroundColor: const Color.fromARGB(255, 54, 54, 54),
                title: const Text('Delete saved network'),
                content: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text('Network name: $networkName',
                        style: const TextStyle(fontSize: 18)),
                    Text('Security : None',
                        style: const TextStyle(fontSize: 18)),
                  ],
                ),
                actions: [
                  CustomTextButton(
                    label: 'Cancel',
                    onPressed: () => Navigator.pop(dialogContext),
                  ),
                  CustomTextButton(
                    label: 'Delete',
                    onPressed: () {
                      // Navigator.pop(dialogContext); // pop dialog first
                      // outerContext
                      //     .read<WirelessSettingsBloc>()
                      //     .add(DeleteSavedNetwork(networkName));
                      // Navigator.pop(context);
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
              child: MechanixNavigationBar(title: ssid ?? '', actionWidgets: [
                IconButton(
                  onPressed: () => onForgetPressed(ssid ?? ''),
                  style: ButtonStyle(
                    iconColor: WidgetStateProperty.all(Colors.white),
                    backgroundColor: WidgetStateProperty.all(Color(0xFFB71C1C)),
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
              ]).padHorizontal(12)),
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
                          title: 'Security',
                          trailing: CustomTrailingText(
                            title: security ?? '',
                          )),
                    ],
                  ),
                ],
              ).padTop(8),
            ),
          ),
        );
      },
    );
  }
}
