import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/constants.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/styles/custom_styles.dart';
import 'package:mechanix_settings/src/commons/styles/text.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:nm/nm.dart';

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
      Navigator.pop(context);
    }

    return BlocProvider(
      create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
      child: BlocListener<ConnectNetworkBloc, ConnectNetworkState>(
        listener: (context, state) {
          if (state.error != null && state.error!.isNotEmpty) {
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(
                content: Text(
                  'Authentication failed: ${state.error}',
                  style: const TextStyle(color: Colors.red),
                ),
              ),
            );
          }
        },
        child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
          builder: (context, state) {
            return Scaffold(
              appBar: CustomAppBar(
                title: "Network",
                leftIcon: Image.asset(Images.back),
                leftIconOnTap: () => Navigator.pop(context),
                customChild: TextButton(
                  onPressed: () => backNavigation(context),
                  style: ButtonStyle(
                    backgroundColor:
                        const WidgetStatePropertyAll<Color>(Color(0xFF3B3B3B)),
                    padding: const WidgetStatePropertyAll<EdgeInsets>(
                        EdgeInsets.symmetric(horizontal: 8)),
                    shape: WidgetStatePropertyAll<RoundedRectangleBorder>(
                      RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(8),
                        side: const BorderSide(),
                      ),
                    ),
                  ),
                  child: Text(
                    "Cancel",
                    style: baseHeaderStyle,
                  ),
                ),
              ),
              body: ContainerWidget(
                child: Form(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        "Enter password to connect to ‘${utf8.decode(accessPoint.ssid)}’",
                        style: baseHeaderStyle.copyWith(fontSize: 20),
                      ),
                      const SizedBox(height: 20),
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Expanded(
                            child: TextFormField(
                              obscureText: state.obscurePassword,
                              style: inputFieldTextStyle,
                              textInputAction: TextInputAction.done,
                              decoration: InputDecoration(
                                focusedBorder: OutlineInputBorder(
                                  borderRadius: BorderRadius.circular(8),
                                  borderSide: const BorderSide(
                                    color: Color(0xFF444444),
                                    width: 2,
                                  ),
                                ),
                                enabledBorder: OutlineInputBorder(
                                  borderRadius: BorderRadius.circular(8),
                                  borderSide: const BorderSide(
                                    color: Color(0xFF444444),
                                    width: 2,
                                  ),
                                ),
                                isDense: true,
                                contentPadding: const EdgeInsets.symmetric(
                                  horizontal: 12,
                                  vertical: 10,
                                ),
                                suffixIcon: IconButton(
                                  icon: Icon(
                                    state.obscurePassword
                                        ? Icons.visibility_off
                                        : Icons.visibility,
                                  ),
                                  onPressed: () {
                                    context
                                        .read<ConnectNetworkBloc>()
                                        .add(TogglePasswordVisibility());
                                  },
                                ),
                              ),
                              onChanged: (value) {
                                context
                                    .read<ConnectNetworkBloc>()
                                    .add(PasswordChanged(value));
                              },
                              onFieldSubmitted: (_) {
                                if (state.password.isNotEmpty) {
                                  context
                                      .read<ConnectNetworkBloc>()
                                      .add(ConnectToNetwork(accessPoint));
                                  backNavigation(context);
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
                    ],
                  ),
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
