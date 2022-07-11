type CommentBlock = {
    type: "comment";
    title?: string;
    text: string;
    media_type: string;
};

type DataBlock = {
    type: "data";
    data: string | ArrayBuffer;
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

type LinkMetadata = {
    type: "link";
    title: string;
    url: string;
};

type DocsMetadata = {
    type: "docs";
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
    | LinkMetadata
    | DocsMetadata
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

export type Result =
    | ResultOk
    | ResultFail
    | ResultNoCoammnd;