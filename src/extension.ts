// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below

import * as vscode from "vscode";
import { pairwriterStartServer } from "./server";
import fs from "fs";
import path from "path";
// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export function activate(context: vscode.ExtensionContext) {
  // Use the console to output diagnostic information (console.log) and errors (console.error)
  // This line of code will only be executed once when your extension is activate

  fs.mkdir(context.logUri.fsPath, { recursive: true }, (err) => {
    if (err) {
      vscode.window.showErrorMessage(err.message);
    }
  });

  process.env.LOGFILE = path.join(context.logUri.fsPath, "pairwriter.log");
  // The command has been defined in the package.json file
  // Now provide the implementation of the command with registerCommand
  // The commandId parameter must match the command field in package.json
  const disposable = vscode.commands.registerCommand(
    "pairwriter.startserver",
    pairwriterStartServer,
  );

  context.subscriptions.push(disposable);
}

// This method is called when your extension is deactivated
export function deactivate() {}
