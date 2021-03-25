import 'dart:async';
import 'dart:io';

class HIDInterface {
  int productID;
  int vendorID;
  int usage;
  int usagePage;
  Process childProcess;
  bool isReady = false;
  bool debugMessages;

  Future connect() async {
    childProcess = await Process.start('hid_send/target/release/hid_send', [productID.toString(), vendorID.toString(), usage.toString(), usagePage.toString()]);
    var isHidSendReady = Completer();

    childProcess.stdout.listen((l) {
      var message = String.fromCharCodes(l);
      if(debugMessages) print('got message from child process: $message');
      if(message.contains('ok')) {
        isReady = true;
        isHidSendReady.complete();
        if(debugMessages) print('Child process is ready!');
      }
      if(message.contains('Error:')) {
        print('Got error from child process: $message');
      }
    });
    return isHidSendReady.future;
  }

  void send(List<int> data) {
    for(var i in data) {
      childProcess.stdin.write(i.toString() + ' ');
    }
    childProcess.stdin.write('\n');
  }

  void diconnect() {
    childProcess.stdin.write('stop\n');
  }

  HIDInterface({this.debugMessages = false, this.productID = 0xEF8D, this.vendorID = 0x4B50, this.usagePage = 0xFF60, this.usage = 0x61});
}