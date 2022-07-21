export type CommentBlock = {
    type: "comment";
    title?: string;
    text: string;
    media_type: string;
};

export type DataBlock = {
    type: "data";
    data: string | ArrayBuffer;
    title?: string;
    file_name?: string;
    media_type: string;
    display?: DisplaySuggestion;
    highlights?: Highlight[];
};

export type SpoilerBlock = {
    type: "spoiler";
    title: string;
    data: string;
};

export type Block =
    | CommentBlock
    | DataBlock
    | SpoilerBlock;

export type LinkMetadata = {
    type: "link";
    title: string;
    url: string;
};

export type DocsMetadata = {
    type: "docs";
    url: string;
};

export type ValueMetadata = {
    type: "value";
    value: any;
};

export type ColorMetadata = {
    type: "value";
    color: string;
};

export type Metadata =
    | LinkMetadata
    | DocsMetadata
    | ValueMetadata
    | ColorMetadata;

export type DisplaySuggestion = {
    binary?: "viewer" | "base64" | "hex";
    image?: "smooth" | "pixelated";
};

export type Highlight = {
    kind: "value";
    range: [number, number];
};

export type Output = {
    title: string;
    blocks: Block[];
    metadata: Metadata[];
    mode: "primary" | "attachment";
};

export type ResultOk = {
    type: "ok";
    outputs: Output[];
};

export type ResultFail = {
    type: "fail";
    outputs: Output[];
};

export type ResultNoCoammnd = {
    type: "no_command";
};

export type Result =
    | ResultOk
    | ResultFail
    | ResultNoCoammnd;