import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_bloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/wireless_settings_state.dart';
import 'package:mechanix_settings/src/features/network/models/saved_networks.dart';
import 'package:mechanix_settings/src/features/network/presentation/configure_dns.dart';
import 'package:mechanix_settings/src/features/network/presentation/configure_proxy.dart';
import 'package:mechanix_settings/src/features/network/presentation/ipv4_address.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class SavedNetworkDetails extends StatefulWidget {
  const SavedNetworkDetails({this.network, super.key});
  final SavedWirelessNetwork? network;

  @override
  State<SavedNetworkDetails> createState() => _SavedNetworkDetailsState();
}

class _SavedNetworkDetailsState extends State<SavedNetworkDetails> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<WirelessSettingsBloc, WirelessSettingsState>(
      builder: (context, state) {
        final network = widget.network!;
        final ssid = network.ssid;

        return Scaffold(
          body: SingleChildScrollView(
            padding: const EdgeInsets.all(16.0),
            physics: const BouncingScrollPhysics(),
            child: ContainerWidget(
              child: Column(
                children: [
                  CustomTitle(
                    title: ssid ?? '',
                  ),
                  MechanixSectionList(
                    physics: const BouncingScrollPhysics(),
                    title: 'About The Network',
                    theme: MechanixSectionListThemeData(
                        widgetPadding: Spacing.only(top: 8, bottom: 40)),
                    sectionListItems: [
                      SectionListItems(
                          defaultTrailingIcon: false,
                          title: 'MAC Address',
                          trailing: SizedBox(
                            width: 250,
                            child: CustomTrailingText(
                              title: network.macAddress ?? "",
                              titleStyle: const TextStyle(
                                overflow: TextOverflow.ellipsis,
                              ),
                            ),
                          ).padRight(8)),
                    ],
                  ),
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
                        trailing: const CustomTrailingText(title: 'Automatic'),
                      ),
                    ],
                  ),
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
                        trailing:
                            const CustomTrailingText(title: 'Off').padRight(8),
                      ),
                    ],
                  ),
                ],
              ).padTop(8),
            ),
          ),
          bottomNavigationBar: MechanixBottomBar(
            leadingWidget: [context.backButton],
          ),
        );
      },
    );
  }
}
