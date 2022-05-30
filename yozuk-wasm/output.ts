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
    display?: DisplaySuggestion;
};

type SpoilerBlock = {
    type: "spoiler";
    title: string;
    data: string;
};

type Block =
    | CommentBlock
    | DataBlock
    | SpoilerBlock;

type DocsMetadata = {
    type: "docs";
    url: string;
};

type ShareMetadata = {
    type: "share";
    url: string;
};

type ValueMetadata = {
    type: "value";
    value: any;
};

type ColorMetadata = {
    type: "value";
    color: string;
};

type Metadata =
    | DocsMetadata
    | ShareMetadata
    | ValueMetadata
    | ColorMetadata;

type DisplaySuggestion = {
    binary?: "viewer" | "base64" | "hex";
    iamge?: "smooth" | "pixelated";
};

export type Output = {
    title: string;
    blocks: Block[];
    metadata: Metadata[];
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

type ResultNoCoammnd = {
    type: "no_command";
};

type ResultError = {
    type: "error";
    message: string;
};

export type Result =
    | ResultOk
    | ResultFail
    | ResultNoCoammnd
    | ResultError;