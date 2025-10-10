import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkBloc.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkEvent.dart';
import 'package:mechanix_settings/src/features/network/blocs/connectNetworkState.dart';
import 'package:mechanix_settings/src/features/network/data/wifi_repository.dart';
import 'package:mechanix_settings/src/features/network/presentation/widgets/wireless_protocols.dart';
import 'package:widgets/mechanix.dart';

class AddNetwork extends StatelessWidget {
  const AddNetwork({super.key});

  @override
  Widget build(BuildContext context) {
    final formKey = GlobalKey<FormState>();
    final nameController = TextEditingController();
    final passwordController = TextEditingController();

    // Get WifiRepository from ancestor provider
    final wifiRepository = context.read<WifiRepository>();

    void backNavigation(BuildContext context) {
      Navigator.pop(context);
    }

    void onAddButtonPressed(BuildContext context, ConnectNetworkState state) {
      if (formKey.currentState!.validate()) {
        context.read<ConnectNetworkBloc>().add(
              PasswordChanged(passwordController.text),
            );
        context.read<ConnectNetworkBloc>().add(
              ConnectToUnknownNetwork(
                nameController.text,
                passwordController.text,
              ),
            );

        if (state.error != '') {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(
                content: Text(
              'Authentication failed!',
              style: TextStyle(color: Colors.red),
            )),
          );
        } else {
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('connecting to network...')),
          );
          backNavigation(context);
        }
      }
    }

    return BlocProvider(
      create: (_) => ConnectNetworkBloc(wifiRepository: wifiRepository),
      child: BlocBuilder<ConnectNetworkBloc, ConnectNetworkState>(
        builder: (context, state) {
          return Scaffold(
            appBar: MechanixNavigationBar(
              title: "New Wireless",
            ),
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
                          if (state.password.isNotEmpty) {
                            onAddButtonPressed(context, state);
                            backNavigation(context);
                          }
                        },
                      ),
                      WirelessProtocols()
                    ],
                  ),
                ),
              ).padTop(8),
            ),
          );
        },
      ),
    );
  }
}
