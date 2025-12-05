import 'package:equatable/equatable.dart';
import 'package:flutter/material.dart';
import 'package:flutter_quill/flutter_quill.dart';
import 'package:mechanix_notes/src/features/editor/models/toolbar_models.dart';

class EditorBlocState extends Equatable {
  final bool isUndo;
  final bool isRedo;
  final bool toolbarToggle;
  final ToolbarEnum selectedToolbar;
  final Document? document;
  final bool isLoading;

  const EditorBlocState({
    this.isUndo = false,
    this.isRedo = false,
    this.toolbarToggle = false,
    this.document,
    this.isLoading = false,
    required this.selectedToolbar,
  });

  EditorBlocState copyWith({
    bool? isUndo,
    bool? isRedo,
    bool? toolbarToggle,
    LayerLink? linkLayer,
    LayerLink? optionsLayer,
    ToolbarEnum? selectedToolbar,
    Document? document,
    bool? isLoading,
  }) {
    return EditorBlocState(
      isUndo: isUndo ?? this.isUndo,
      isRedo: isRedo ?? this.isRedo,
      toolbarToggle: toolbarToggle ?? this.toolbarToggle,
      selectedToolbar: selectedToolbar ?? this.selectedToolbar,
      isLoading: isLoading ?? this.isLoading,
      document: document,
    );
  }

  @override
  List<Object?> get props => [
    isUndo,
    isRedo,
    toolbarToggle,
    selectedToolbar,
    isLoading,
    document,
  ];
}
