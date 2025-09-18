import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_trailing_text.dart';
import 'package:mechanix_settings/src/features/about/bloc/about_bloc.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/listItems/simple_list_items_type.dart';

class About extends StatefulWidget {
  const About({super.key});

  @override
  State<About> createState() => _AboutState();
}

class _AboutState extends State<About> {
  @override
  Widget build(BuildContext context) {
    return BlocBuilder<AboutBloc, AboutState>(
      builder: (context, state) {
        return Scaffold(
          appBar: MechanixNavigationBar(title: 'About this Comet'),
          body: ContainerWidget(
              child: Column(
            children: [
              MechanixSimpleList(
                listItems: [
                  SimpleListItems(
                    title: 'Device Name',
                    trailing: CustomTrailingText(title: state.hostname),
                  ),
                ],
              ),
              MechanixSimpleList(
                listItems: [
                  SimpleListItems(
                    title: 'Model',
                    trailing: CustomTrailingText(title: state.hardwareModel),
                  ),
                  SimpleListItems(
                    title: 'Firmware Version',
                    trailing: CustomTrailingText(title: state.firmwareVersion),
                  ),
                  SimpleListItems(
                    title: 'Kernel Version',
                    trailing: CustomTrailingText(title: state.kernelRelease),
                  ),
                  SimpleListItems(
                    title: 'OS Name',
                    trailing:
                        CustomTrailingText(title: state.operatingSystemName),
                  ),
                ],
              )
            ],
          ).padTop(8)),
        );
      },
    );
  }
}
