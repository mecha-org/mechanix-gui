import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
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

    return BlocProvider(
      create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
      child: BlocListener<ConnectNetworkBloc, ConnectNetworkState>(
        listenWhen: (previous, current) {
          return previous.deviceState != current.deviceState ||
                 previous.error != current.error;
        },
        listener: (context, state) {
          ScaffoldMessenger.of(context).clearSnackBars();

          final bool hasError = 
              state.deviceState == NetworkManagerDeviceState.needAuth ||
              (state.error != null && state.error!.isNotEmpty);

          if (hasError) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(
                  state.error ?? "Connection failed",
                  style: const TextStyle(color: Colors.red),
                ),
                duration: const Duration(seconds: 2),
                backgroundColor: Colors.grey[800],
              ),
            );
            return;
          }

          final bool isAuthenticating = 
              state.deviceState == NetworkManagerDeviceState.ipCheck ||
              state.deviceState == NetworkManagerDeviceState.config;

          if (isAuthenticating) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: const Text(
                  "Authenticating...",
                  style: TextStyle(color: Colors.white),
                ),
                duration: const Duration(seconds: 2),
                backgroundColor: Colors.grey[800],
              ),
            );
          }
        },
        child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
          builder: (context, state) {
            return Scaffold(
              appBar: PreferredSize(
                  preferredSize: const Size.fromHeight(52),
                  child: MechanixNavigationBar(
                      title: "Join ${utf8.decode(accessPoint.ssid)}",
                      actionWidgets: [
                        IconButton(
                          icon: Image.asset(
                            Images.submit,
                            width: 20,
                            height: 20,
                          ),
                          onPressed: state.password.isNotEmpty &&
                                  state.password.length >= 8
                              ? () {
                                  context
                                      .read<ConnectNetworkBloc>()
                                      .add(ConnectToNetwork(accessPoint));
                                }
                              : null,
                        ),
                      ]).padHorizontal(12)),
              body: ContainerWidget(
                child: Form(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Expanded(
                            child: Padding(
                              padding: const EdgeInsets.all(16.0),
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
                                  if (state.password.isNotEmpty &&
                                      state.password.length >= 8) {
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
                          ),
                        ],
                      ),
                       // // NOTE: Not in use currently
                      // WirelessProtocols()
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