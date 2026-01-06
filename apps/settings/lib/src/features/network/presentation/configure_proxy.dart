import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/back_button.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_title.dart';
import 'package:mechanix_settings/src/features/network/models/types.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/select/select_type.dart';

class ConfigureProxyWidget extends StatefulWidget {
  const ConfigureProxyWidget({super.key});

  @override
  State<ConfigureProxyWidget> createState() => _ConfigureProxyWidgetState();
}

class _ConfigureProxyWidgetState extends State<ConfigureProxyWidget> {
  void backNavigation(BuildContext context) {
    Navigator.pop(context);
  }

  DnsProxyConfiguration? selectedProxyDNS;

  void _handleProxyDNSChange(SelectOption<DnsProxyConfiguration> option) {
    setState(() {
      selectedProxyDNS = option.value;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: ContainerWidget(
        child: SingleChildScrollView(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              const CustomTitle(title: "Configure Proxy"),
              MechanixSelect<DnsProxyConfiguration>(
                value: selectedProxyDNS,
                onChanged: _handleProxyDNSChange,
                options: dnsProxyOptions,
              ),
            ],
          ),
        ),
      ),
      bottomSheet: MechanixBottomBar(
        leadingWidget: [context.backButton],
      ),
    );
  }
}
