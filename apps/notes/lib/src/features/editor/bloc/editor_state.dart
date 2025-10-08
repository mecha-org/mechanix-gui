import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:mechanix_notes/src/features/home/models/toolbar_model.dart';

class EditorBlocState extends Equatable {
  final bool isUndo;
  final bool isRedo;
  final bool isPinned;
  final bool toolbarToggle;
  final ToolbarEnum selectedToolbar;

  const EditorBlocState({
    this.isUndo = false,
    this.isRedo = false,
    this.isPinned = false,
    this.toolbarToggle = true,
    required this.selectedToolbar,
  });

  EditorBlocState copyWith({
    bool? isUndo,
    bool? isRedo,
    bool? isPinned,
    bool? toolbarToggle,
    LayerLink? linkLayer,
    LayerLink? optionsLayer,
    ToolbarEnum? selectedToolbar,
  }) {
    return EditorBlocState(
      isUndo: isUndo ?? this.isUndo,
      isRedo: isRedo ?? this.isRedo,
      isPinned: isPinned ?? this.isPinned,
      toolbarToggle: toolbarToggle ?? this.toolbarToggle,
      selectedToolbar: selectedToolbar ?? this.selectedToolbar,
    );
  }

  @override
  List<Object?> get props => [
    isUndo,
    isRedo,
    isPinned,
    toolbarToggle,
    selectedToolbar,
  ];
}
