type CommentBlock = {
    type: "comment";
    title?: string;
    text: string;
    media_type: string;
};

type DataBlock = {
    type: "data";
    data: Uint8Array;
    title?: string;
    file_name?: string;
    media_type: string;
};

type Block =
    | CommentBlock
    | DataBlock;

export type Output = {
    title: string;
    blocks: Block[];
    mode: "primary" | "attachment";
};

type ResultOk = {
    type: "ok";
    outputs: Output[];
};

type ResultFail = {
    type: "fail";
    outputs: Output[];
};

export type Result =
    | ResultOk
    | ResultFail;