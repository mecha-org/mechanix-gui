import 'dart:io';
import 'package:flutter/material.dart';
import 'package:csv/csv.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_app_bar.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:path/path.dart' as p;

/// A StatefulWidget to view and edit CSV files.
/// Supports row-wise deletion, addition, and inline editing.
class CsvViewer extends StatefulWidget {
  final String filePath;

  const CsvViewer({super.key, required this.filePath});

  @override
  State<CsvViewer> createState() => _CsvViewerState();
}

class _CsvViewerState extends State<CsvViewer> {
  // Holds CSV table data, each row includes a unique ID at index 0
  List<List<String>> table = [];

  // Used to track which cell is being edited for UI purposes
  String? _editingCellKey;

  @override
  void initState() {
    super.initState();
    _loadCsv();
  }

  /// Loads the CSV file, parses it, and adds a unique key to each row
  void _loadCsv() {
    final input = File(widget.filePath).readAsStringSync();
    final parsed = const CsvToListConverter().convert(input);
    setState(() {
      // Add a unique ID to each row to ensure stable row identity
      table = parsed
          .map((row) =>
              [UniqueKey().toString(), ...row.map((cell) => cell.toString())])
          .toList();
    });
  }

  /// Saves current table state (excluding the ID column) to the original CSV file
  void _saveCsv() {
    final filteredTable =
        table.map((row) => row.sublist(1)).toList(); // Remove the ID column
    final csvContent = const ListToCsvConverter().convert(filteredTable);
    File(widget.filePath).writeAsStringSync(csvContent);
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text("CSV saved")),
    );
  }

  /// Adds a new empty row with a unique ID
  void _addRow() {
    setState(() {
      final newRowIndex = table.length;
      table.add(
        [UniqueKey().toString(), ...List.filled(table.first.length - 1, '')],
      );
      _editingCellKey =
          '$newRowIndex-1'; // Focus on first editable column (col index 1)
    });
  }

  /// Deletes a row from the table and saves the file
  void _deleteRow(int rowIndex) {
    setState(() {
      table.removeAt(rowIndex);
    });
    _saveCsv();
  }

  /// Shows a confirmation dialog before deleting a row
  void _confirmDeleteRow(int rowIndex) async {
    final confirm = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Confirm Deletion'),
        content: const Text('Are you sure you want to delete this row?'),
        actions: [
          TextButton(
              onPressed: () => Navigator.pop(context, false),
              child: const Text('Cancel')),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text("Delete", style: TextStyle(color: Colors.red)),
          ),
        ],
      ),
    );
    if (confirm == true) {
      _deleteRow(rowIndex);
    }
  }

  /// Updates the value of a cell at [rowIndex], [columnIndex] and saves the change
  void _updateCell(int rowIndex, int columnIndex, String value) {
    setState(() {
      table[rowIndex][columnIndex] = value;
    });
    // _saveCsv(); // Save on every cell change
  }

  /// Determines whether the underline border should be shown in a cell
  bool _shouldShowUnderline(int rowIndex, int columnIndex) {
    return _editingCellKey == '$rowIndex-$columnIndex';
  }

  @override
  Widget build(BuildContext context) {
    if (table.isEmpty) {
      // Show loading indicator or empty state
      return const Center(child: CircularProgressIndicator());
    }

    return Scaffold(
      appBar: CustomAppBar(
        title: p.basename(widget.filePath), // Show file name
        leftIcon: const Icon(Icons.arrow_back),
        leftIconOnTap: () => Navigator.pop(context), // Back navigation
        rightIcon1: const Icon(Icons.add),
        rightIcon1OnTap: _addRow, // Add new row
        rightIcon2: const Icon(Icons.save),
        rightIcon2OnTap: _saveCsv, // Save file manually
      ),
      body: ContainerWidget(
        child: InteractiveViewer(
          constrained: false,
          child: DataTable(
            columns: [
              // Skip the first column (ID) in the table display
              for (int i = 1; i < table.first.length; i++)
                DataColumn(label: Text('Col $i')),
              const DataColumn(label: Text('Delete')), // Action column
            ],
            rows: [
              for (int rowIndex = 0; rowIndex < table.length; rowIndex++)
                DataRow(
                  key: ValueKey(table[rowIndex][0]), // Use unique ID as key
                  cells: [
                    for (int colIndex = 1;
                        colIndex < table[rowIndex].length;
                        colIndex++)
                      DataCell(
                        Focus(
                          onFocusChange: (hasFocus) {
                            // Clear editing highlight on focus loss
                            if (!hasFocus &&
                                _editingCellKey == '$rowIndex-$colIndex') {
                              setState(() {
                                _editingCellKey = null;
                              });
                            }
                          },
                          child: TextFormField(
                            initialValue: table[rowIndex][colIndex],
                            onTap: () {
                              // Set cell as being edited
                              setState(() {
                                _editingCellKey = '$rowIndex-$colIndex';
                              });
                            },
                            decoration: InputDecoration(
                              isDense: true,
                              contentPadding: const EdgeInsets.symmetric(
                                  vertical: 10, horizontal: 8),
                              border: _shouldShowUnderline(rowIndex, colIndex)
                                  ? const UnderlineInputBorder()
                                  : InputBorder.none,
                            ),
                            onChanged: (value) {
                              _updateCell(rowIndex, colIndex, value);
                            },
                          ),
                        ),
                      ),
                    DataCell(
                      IconButton(
                        icon: const Icon(Icons.delete, color: Colors.red),
                        onPressed: () => _confirmDeleteRow(rowIndex),
                      ),
                    ),
                  ],
                ),
            ],
          ),
        ),
      ),
    );
  }
}
