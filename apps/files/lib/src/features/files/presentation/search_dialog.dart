import 'package:flutter/material.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/commons/customWidgets/pressable_icon.dart';
import 'package:widgets/mechanix.dart';

class SearchOverlayController {
  OverlayEntry? _searchOverlayEntry;

  final FocusNode _focusNode = FocusNode();

  final ValueNotifier<bool> isTextInputOpened = ValueNotifier(false);

  SearchOverlayController() {
    _focusNode.addListener(focusChange);
  }

  bool get isVisible => _searchOverlayEntry != null;

  void show(
    BuildContext context, {
    required ValueNotifier<String> searchQuery,
    required VoidCallback onClear,
    required Function(String) onSearch,
  }) {
    final overlay = Overlay.of(context);

    hide();

    _searchOverlayEntry = OverlayEntry(
      builder: (context) {
        WidgetsBinding.instance.addPostFrameCallback((_) async {
          await Future.delayed(const Duration(milliseconds: 300));
          _focusNode.addListener(focusChange);
          if (!_focusNode.canRequestFocus) return;
          if (_searchOverlayEntry == null) return;
        });
        return Positioned(
          left: 0,
          right: 0,
          bottom: 0,
          child: Material(
            color: Colors.transparent,
            child: Container(
              color: context.secondaryContainer,
              child: MechanixTextInput.search(
                autofocus: true,
                focusNode: _focusNode,

                prefixIcon: IconWidget(
                  iconPath: Images.search,
                  iconColor: context.colorScheme.onSurface,
                  iconHeight: 24,
                  iconWidth: 24,
                ),

                anchorWidget: Padding(
                  padding: const EdgeInsets.only(left: 5),
                  child: DecoratedPressableIcon(
                    onTap: () {
                      onClear();
                      hide();
                    },
                    icon: const Icon(Icons.close),
                  ),
                ),

                hintText: "Search here",

                onChanged: (query) {
                  searchQuery.value = query;
                  onSearch(query);
                },

                onClear: onClear,
              ),
            ),
          ),
        );
      },
    );

    overlay.insert(_searchOverlayEntry!);

    WidgetsBinding.instance.addPostFrameCallback((_) {
      _focusNode.requestFocus();
    });
  }

  void hide() {
    _searchOverlayEntry?.remove();
    _searchOverlayEntry = null;
    _focusNode.unfocus();
  }

  void dispose() {
    hide();
    _focusNode.removeListener(focusChange);
    _focusNode.dispose();
    isTextInputOpened.dispose();
  }

  Future<void> focusChange() async {
    if (!_focusNode.hasFocus) {
      await Future.delayed(const Duration(milliseconds: 300));
    }

    isTextInputOpened.value = _focusNode.hasFocus;
  }
}
