import * as vscode from "vscode";


const startServer:(port: number, currentworkingdir: string) => PairwriterCmdObj = (()=> {
  return require("@pairwriter-helper").startServer;
}) ();
export async function pairwriterStartServer() {
  const port = await vscode.window.showInputBox({});
  const username = await vscode.window.showInputBox({});
  process.env.SERVER_USERNAME = username;

  // The code you place here will be executed every time your command is executed
  if (!port) {
    vscode.window.showErrorMessage("Please provide a port number");
    return;
  }

  const portNumber = +port;

  if (!vscode.workspace.workspaceFolders) {
    return await vscode.window.showErrorMessage("No workspace folder is open");
  }
  // Get the path of the first workspace folder
  const folderPath = vscode.workspace.workspaceFolders[0].uri.fsPath;
  const serverFunctions: PairwriterCmdObj = startServer(portNumber, folderPath);
  openDocumentHock(serverFunctions);
  editDocumentHock(serverFunctions);
  outsideEditHock(serverFunctions);
}
/// this is hock that will be called when the server is started
function openDocumentHock(serverFunctions: PairwriterCmdObj) {
  vscode.workspace.onDidOpenTextDocument((document) => {
    // get the relative path
    if (document.uri.scheme === "file") {
      const relativePath = "./" + vscode.workspace.asRelativePath(document.uri);
      const textArray: Uint8Array = serverFunctions.readFile(relativePath);
      try {
        const text = Buffer.from(textArray).toString("utf-8");
        vscode.window.showTextDocument(document).then((editor) => {
          editor.edit((editBuilder) => {
            const fullRange = new vscode.Range(
              document.positionAt(0), // Start of the document
              document.positionAt(document.getText().length), // End of the document
            );

            editBuilder.replace(fullRange, text);
          });
        });
      } catch (error) {
        vscode.window.showErrorMessage(
          "Error opening file :\nmost likely the file is binary",
        );
      }
    }
  });
}
function editDocumentHock(serverFunctions: PairwriterCmdObj) {
  vscode.workspace.onDidChangeTextDocument((event) => {
    const document = event.document;
    if (document.uri.scheme === "file") {
      const relativePath = "./" + vscode.workspace.asRelativePath(document.uri);
      const text = document.getText();
      serverFunctions.updateBuf(relativePath, text);
    }
  });
}
function outsideEditHock(serverFunctions: PairwriterCmdObj) {
  const loop = () => {
    const promis = serverFunctions.fileChange();
    promis.then(async (path) => {
      const absolutePath =
        vscode.workspace.workspaceFolders![0].uri.fsPath + path.slice(1);
      const uri = vscode.Uri.file(absolutePath);
      const document = await vscode.workspace.openTextDocument(uri);
      const editor = await vscode.window.showTextDocument(document);
      editor.edit((editBuilder) => {
        const fullRange = new vscode.Range(
          document.positionAt(0), // Start of the document
          document.positionAt(document.getText().length), // End of the document
        );
        const text = Buffer.from(serverFunctions.readFile(path)).toString(
          "utf8",
        );
        editBuilder.replace(fullRange, text);
      });
      setTimeout(loop, 0);
    });
  };
  setTimeout(loop, 0);
}
