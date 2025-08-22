import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter_code_editor/flutter_code_editor.dart';
import 'package:flutter_highlight/flutter_highlight.dart';
import 'package:flutter_highlight/themes/monokai.dart';
import 'package:highlight/highlight_core.dart';
import 'package:highlight/languages/dart.dart';
import 'package:highlight/languages/json.dart';
import 'package:highlight/languages/sql.dart';
import 'package:highlight/languages/yaml.dart';
import 'package:highlight/languages/python.dart';
import 'package:highlight/languages/java.dart';
import 'package:highlight/languages/cpp.dart';
import 'package:highlight/languages/ruby.dart';
import 'package:highlight/languages/xml.dart';
import 'package:highlight/languages/rust.dart';
import 'package:highlight/languages/javascript.dart';
import 'package:path/path.dart' as p;

class CodeEditorPage extends StatefulWidget {
  final String filePath;
  const CodeEditorPage({super.key, required this.filePath});

  @override
  State<CodeEditorPage> createState() => _CodeEditorPageState();
}

class _CodeEditorPageState extends State<CodeEditorPage> {
  bool _initialized = false;
  bool _isEditing = false;

  late String _code;
  late CodeController _codeController;

  @override
  void initState() {
    super.initState();
    _initEditor();
  }

  Future<void> _initEditor() async {
    final code = await File(widget.filePath).readAsString();
    final ext = p.extension(widget.filePath).replaceAll('.', '');

    final language = _getLanguage(ext);
    _code = code;

    _codeController = CodeController(
      text: code,
      language: language,
    );

    setState(() => _initialized = true);
  }

  Mode _getLanguage(String ext) {
    switch (ext) {
      case 'dart':
        return dart;
      case 'json':
        return json;
      case 'yaml':
      case 'yml':
        return yaml;
      case 'py':
        return python;
      case 'java':
        return java;
      case 'c':
      case 'cpp':
        return cpp;
      case 'js':
        return javascript;
      case 'sql':
        return sql;
      // case 'ini':
      // case 'toml':
      // return ini; // NOTE: Syntax highlighting modes for 'ini' and 'toml' are currently not supported. Using 'dart' as a fallback for these
      case 'rb':
        return ruby;
      case 'xml':
        return xml;
      case 'rs':
        return rust;
      default:
        return dart; // fallback
    }
  }

  String _getLanguageName(String ext) {
    switch (ext) {
      case 'dart':
      case 'json':
      case 'yaml':
      case 'py':
      case 'java':
      case 'c':
      case 'cpp':
      case 'js':
      case 'sql':
      // case 'ini':
      // case 'toml': // NOTE: Fallback to 'dart' for unsupported languages
      case 'rb':
      case 'xml':
      case 'rs':
      case 'yml':
        return ext;
      default:
        return 'dart'; // fallback
    }
  }

  Future<void> _save() async {
    final updated = _codeController.text;
    await File(widget.filePath).writeAsString(updated);

    setState(() {
      _code = updated;
      _isEditing = false;
    });

    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('Saved successfully')),
    );
  }

  @override
  Widget build(BuildContext context) {
    if (!_initialized) {
      return const Scaffold(
        body: Center(child: CircularProgressIndicator()),
      );
    }

    return Scaffold(
        appBar: AppBar(
          title: Text(p.basename(widget.filePath)),
          actions: [
            if (_isEditing) ...[
              IconButton(icon: const Icon(Icons.save), onPressed: _save),
              IconButton(
                icon: const Icon(Icons.cancel),
                onPressed: () => setState(() => _isEditing = false),
              ),
            ] else
              IconButton(
                icon: const Icon(Icons.edit),
                onPressed: () => setState(() => _isEditing = true),
              ),
          ],
        ),
        body: Padding(
          padding: const EdgeInsets.all(12),
          child: _isEditing
              ? SingleChildScrollView(
                  child: CodeTheme(
                    data: CodeThemeData(styles: monokaiTheme),
                    child: CodeField(
                      controller: _codeController,
                      textStyle: const TextStyle(fontFamily: 'monospace'),
                    ),
                  ),
                )
              : InteractiveViewer(
                  constrained: false,
                  child: HighlightView(
                    _code,
                    // language: languageName,
                    language: _getLanguageName(
                        p.extension(widget.filePath).replaceAll('.', '')),
                    theme: monokaiTheme,
                    padding: const EdgeInsets.all(12),
                    textStyle: const TextStyle(fontFamily: 'monospace'),
                  ),
                ),
        ));
  }
}
