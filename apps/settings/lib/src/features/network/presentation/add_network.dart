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

class AddNetwork extends StatelessWidget {
  const AddNetwork({super.key});

  @override
  Widget build(BuildContext context) {
    final formKey = GlobalKey<FormState>();
    final wifiRepository = context.read<WifiRepository>();

    // void backNavigation(BuildContext context) {
    //   Navigator.pop(context);
    // }

    void onAddButtonPressed(BuildContext context, ConnectNetworkState state) {
      if (formKey.currentState!.validate()) {
        context.read<ConnectNetworkBloc>().add(
              ConnectToHiddenNetwork(
                state.username,
                state.password,
              ),
            );
      }
    }

    return BlocProvider(
        create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
        child: BlocListener<ConnectNetworkBloc, ConnectNetworkState>(
          listener: (context, state) {
            ScaffoldMessenger.of(context).clearSnackBars();

            final bool hasError =
                state.deviceState == NetworkManagerDeviceState.needAuth ||
                    (state.error != null && state.error!.isNotEmpty);

            final bool isAuthenticating =
                state.deviceState == NetworkManagerDeviceState.ipCheck ||
                    state.deviceState == NetworkManagerDeviceState.config;

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
            } else if (isAuthenticating) {
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
            } else {
              // // TODO: check connected network is same as active connection added,
              // // then navigate to previous screen
              ScaffoldMessenger.of(context).showSnackBar(
                SnackBar(
                  content: const Text(
                    "Connected",
                    style: TextStyle(color: Colors.white),
                  ),
                  duration: const Duration(seconds: 2),
                  backgroundColor: Colors.grey[800],
                ),
              );
              // backNavigation(context);
            }
          },
          child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
            builder: (context, state) {
              return Scaffold(
                appBar: PreferredSize(
                    preferredSize: const Size.fromHeight(52),
                    child: MechanixNavigationBar(
                        title: "New Wireless",
                        actionWidgets: [
                          IconButton(
                            icon: Image.asset(Images.submit,
                                width: 20, height: 20),
                            onPressed: state.password.isNotEmpty &&
                                    state.password.length >= 8
                                ? () {
                                    onAddButtonPressed(context, state);
                                  }
                                : null,
                          ),
                        ]).padHorizontal(12)),
                body: SingleChildScrollView(
                  physics: const BouncingScrollPhysics(),
                  child: ContainerWidget(
                    child: Form(
                      key: formKey,
                      child: Column(
                        children: [
                          MechanixTextInput.textInput(
                            hintText: 'Name',
                            isFormField: true,
                            onChanged: (value) {
                              context
                                  .read<ConnectNetworkBloc>()
                                  .add(UsernameChanged(value));
                            },
                          ).padBottom(8),
                          MechanixTextInput.password(
                            hintText: 'Enter Password',
                            isFormField: true,
                            onChanged: (value) {
                              context
                                  .read<ConnectNetworkBloc>()
                                  .add(PasswordChanged(value));
                            },
                            onFieldSubmitted: (_) {
                              if (state.password.isNotEmpty &&
                                  state.password.length >= 8) {
                                onAddButtonPressed(context, state);
                              }
                            },
                          ),
                          // // NOTE: Not in use currently
                          // WirelessProtocols()
                        ],
                      ),
                    ),
                  ).padTop(8),
                ),
              );
            },
          ),
        ));
  }
}
