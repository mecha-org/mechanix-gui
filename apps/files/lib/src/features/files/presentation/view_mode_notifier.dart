import 'package:flutter/material.dart';

class ViewModeNotifier extends ValueNotifier<bool> {
  // false = list view, true = grid view
  ViewModeNotifier() : super(false);
}

// Global singleton
final viewModeNotifier = ViewModeNotifier();
