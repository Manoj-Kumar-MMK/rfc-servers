import 'dart:io';
import 'dart:convert' show utf8;

const port = 4000;
var outputFile = File('output.txt');

Future<void> handleRequests(HttpServer server) async {
  await for (HttpRequest request in server) {
    try {
      String body = await utf8.decodeStream(request);
      String response_content = 'Body length : ${body.length}\nBody : $body';
      String log_content =
          '\n<-------\nRequest-headers :\n${request.headers}\n\n' +
              response_content +
              '\n------->\n';
      outputFile.writeAsStringSync(log_content, mode: FileMode.append);
      request.response.write(response_content);
      await request.response.close();
    } on Error catch (e) {
      print(e);
    }
  }
}

Future<void> main() async {
  final server = await createServer();
  print('Server started at port ${server.port}');
  await handleRequests(server);
}

Future<HttpServer> createServer() async {
  final address = InternetAddress.loopbackIPv4;
  const port = 4000;
  return await HttpServer.bind(address, port);
}
