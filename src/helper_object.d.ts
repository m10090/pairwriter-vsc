type PairwriterCmdObj = {
  readFile: (path: string) => Uint8Array;
  editBuf: (path: string, pos: number, del: number, text: string) => undefined;
  readFileTree: () => {
    files: string[];
    emptyDirs: string[];
  };
  updateBuf: (path: string, text: string) => undefined;
  fileChange: () => Promise<string>;
  sendRpc: (JsObject: RPC) => undefined;
};

type RPC =
  | {
      CreateDirectory: {
        path: string;
      };
    }
  | {
      DeleteDirectory: {
        path: string;
      };
    }
  | {
      MoveDirectory: {
        path: string;
        new_path: string;
      };
    }
  // file system operations
  | {
      CreateFile: {
        path: string;
      };
    }
  | {
      DeleteFile: {
        path: String;
      };
    }
  | {
      MoveFile: {
        path: String;
        new_path: String;
      };
    }
  | {
      ReqSaveFile: {
        path: String;
      };
    }
  | {
      Undo: {
        path: string;
      };
    }
  | {
      Redo: {
        path: string;
      };
    };
