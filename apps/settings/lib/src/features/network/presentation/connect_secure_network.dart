import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/app_route.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_protocols.dart';
import 'package:nm/nm.dart';
import 'package:widgets/mechanix.dart';

class ConnectSecureNetwork extends StatelessWidget {
  const ConnectSecureNetwork({super.key});

  @override
  Widget build(BuildContext context) {
    final args =
        ModalRoute.of(context)!.settings.arguments as Map<String, dynamic>;
    final NetworkManagerAccessPoint accessPoint =
        args['accessPoint'] as NetworkManagerAccessPoint;

    final wifiRepository = context.read<WifiRepository>();

    void backNavigation(BuildContext context) {
      Navigator.pop(context);
    }

    return BlocProvider(
      create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
      child: BlocListener<ConnectNetworkBloc, ConnectNetworkState>(
        listenWhen: (context, state) {
          return state.deviceState == NetworkManagerDeviceState.activated;
        },
        listener: (context, state) =>
            Navigator.pushNamed(context, AppRoutes.wireless),
        child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
          builder: (context, state) {
            return Scaffold(
              appBar: MechanixNavigationBar(
                title: "Join ${utf8.decode(accessPoint.ssid)}",
              ),
              body: ContainerWidget(
                child: Form(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Expanded(
                            child: MechanixTextInput.password(
                              label: 'Wireless Credentials',
                              isFormField: true,
                              hintText: 'Enter Password',
                              onChanged: (value) {
                                context
                                    .read<ConnectNetworkBloc>()
                                    .add(PasswordChanged(value));
                              },
                              onFieldSubmitted: (_) {
                                if (state.password.isNotEmpty) {
                                  print('connection enter pressed');
                                  context
                                      .read<ConnectNetworkBloc>()
                                      .add(ConnectToNetwork(accessPoint));
                                }
                              },
                              validator: (value) {
                                if (value == null || value.isEmpty) {
                                  return 'Please enter a password';
                                }
                                return null;
                              },
                            ),
                          ),
                        ],
                      ),
                      WirelessProtocols()
                    ],
                  ).padTop(8),
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
