import 'package:flutter/material.dart';
import 'package:mechanix_settings/src/commons/customWidgets/custom_container.dart';

class LabelValueListRow extends StatelessWidget {
  final String title; // details' key/label
  final String? value;

  const LabelValueListRow({
    super.key,
    required this.title,
    this.value = '',
  });

  @override
  @override
  Widget build(BuildContext context) {
    return FixedHeightRow(
      child: ListTile(
        title: Text(title,
            style: const TextStyle(
              color: Colors.white,
              fontSize: 24,
            )),
        trailing: Text(
          value!,
          style: const TextStyle(
              color: Color.fromARGB(197, 255, 255, 255), fontSize: 20),
        ),
      ),
    );
  }
}
