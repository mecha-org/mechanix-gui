import 'dart:io';
import 'package:flutter/material.dart';
import 'package:excel/excel.dart';
import 'package:mechanix_files/src/commons/customWidgets/custom_container.dart';
import 'package:path/path.dart' as p;

/// A widget that displays an Excel file from the device's local path.
class ExcelViewer extends StatefulWidget {
  final String filePath;

  const ExcelViewer({super.key, required this.filePath});

  @override
  State<ExcelViewer> createState() => _ExcelViewerState();
}

class _ExcelViewerState extends State<ExcelViewer> {
  Excel? excel; // The decoded Excel file
  List<List<String>> table = []; // Table data for the selected sheet
  List<String> sheetNames = []; // All sheet names in the Excel file
  String? selectedSheet; // Currently selected sheet name
  bool _isLoading = true; // Flag to indicate loading state

  /// Loads the Excel file from the file system and extracts sheet names
  Future<void> _loadExcelFromAssets() async {
    setState(() {
      _isLoading = true;
    });

    try {
      final file = File(widget.filePath);

      if (!await file.exists()) {
        throw Exception("File does not exist at path: ${widget.filePath}");
      }

      final bytes = await file.readAsBytes();

      if (bytes.isEmpty) {
        throw Exception("File is empty.");
      }

      // Decode the Excel file from bytes
      final decoded = Excel.decodeBytes(bytes);

      if (decoded.tables.isEmpty) {
        throw Exception('Excel file has no sheets.');
      }

      setState(() {
        excel = decoded;
        sheetNames = decoded.tables.keys.toList();
        selectedSheet = sheetNames.first;
      });

      // Load data from the first sheet by default
      await _loadSheetData(selectedSheet!);
    } catch (e) {
      // If any error occurs, stop loading
      setState(() {
        _isLoading = false;
      });
    }
  }

  /// Loads the content of a given sheet into the table
  Future<void> _loadSheetData(String sheetName) async {
    final sheet = excel?.tables[sheetName];
    if (sheet == null || sheet.rows.isEmpty) {
      setState(() {
        table = [];
        _isLoading = false;
      });
      return;
    }

    // Convert Excel cells to list of strings
    final rows = sheet.rows
        .map((row) => row.map((cell) => cell?.value.toString() ?? '').toList())
        .toList();

    // Artificial delay to simulate loading state
    await Future.delayed(const Duration(milliseconds: 100));

    setState(() {
      table = rows;
      _isLoading = false;
    });
  }

  @override
  void initState() {
    super.initState();
    _loadExcelFromAssets(); // Load Excel file on widget initialization
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(p.basename(widget.filePath)), // Show filename in title
        actions: [
          // Sheet selector dropdown (if not loading and sheets are available)
          if (!_isLoading && sheetNames.isNotEmpty)
            DropdownButton<String>(
              value: selectedSheet,
              onChanged: (value) async {
                if (value != null) {
                  setState(() {
                    selectedSheet = value;
                    _isLoading = true;
                  });
                  await _loadSheetData(value); // Load data for selected sheet
                }
              },
              items: sheetNames
                  .map((name) => DropdownMenuItem(
                        value: name,
                        child: Text(name),
                      ))
                  .toList(),
            ),
        ],
      ),
      body: _isLoading
          // Show loading indicator while reading file or sheet
          ? const Center(child: CircularProgressIndicator())
          // Show message if no data found in selected sheet
          : table.isEmpty
              ? const Center(child: Text('No data available in this sheet.'))
              // Render Excel table using DataTable inside scroll views
              : ContainerWidget(
                  child: InteractiveViewer(
                    constrained: false,
                    child: SingleChildScrollView(
                      scrollDirection: Axis.horizontal,
                      child: SingleChildScrollView(
                        child: DataTable(
                          // Create columns based on number of cells in first row
                          columns: [
                            for (int i = 0;
                                i < (table.isNotEmpty ? table.first.length : 0);
                                i++)
                              DataColumn(label: Text('Col ${i + 1}')),
                          ],
                          // Create rows dynamically from table data
                          rows: [
                            for (var row in table)
                              DataRow(
                                cells: row
                                    .map((cell) => DataCell(Text(cell)))
                                    .toList(),
                              ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ),
    );
  }
}
