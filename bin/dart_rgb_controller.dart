import 'dart:convert';
import 'dart:io';

import 'package:dart_rgb_controller/HIDInterface.dart';


void main(List<String> arguments) async {
  var hidInterface = HIDInterface(debugMessages: false);
  await hidInterface.connect();
  hidInterface.send([255, 70, 255]);
  await Future.delayed(Duration(milliseconds: 1)); // don't kill hid_send before sending
  hidInterface.diconnect();
}
