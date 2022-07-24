import { decode } from 'base64-arraybuffer';
import { Highlight, Result } from './output'

export abstract class YozukBase {
    protected abstract exec_impl(command: string, i18n: string): Promise<string>;
    protected abstract push_stream_impl(stream: Uint8Array): Promise<void>;
    protected abstract random_suggestions_impl(amount: number): Promise<string>;
    protected abstract push_suggestions_stream_impl(stream: Uint8Array): Promise<void>;
    protected abstract clear_suggestions_stream_impl(): Promise<void>;
    protected abstract suggestions_impl(command: string, amount: number): Promise<string>;
    protected abstract i18n(): I18n;

    async exec(command: string, streams: Uint8Array[] = []): Promise<Result> {
        for (const stream of streams) {
            await this.push_stream_impl(stream);
        }
        const result: Result = JSON.parse(await this.exec_impl(command, JSON.stringify(this.i18n())));
        const textDecoder = new TextDecoder('utf-8', { fatal: true });
        if (result.type == "ok" || result.type == "fail") {
            for (const output of result.outputs) {
                for (const block of output.blocks) {
                    if (block.type == "data") {
                        if (typeof block.data == "string") {
                            const decoded = decode(block.data);
                            try {
                                block.data = textDecoder.decode(decoded);
                                if (block.highlights) {
                                    let newHighlights: Highlight[] = [];
                                    let byteStart = 0;
                                    let charStart = 0;
                                    for (const highlight of block.highlights) {
                                        charStart += textDecoder.decode(decoded.slice(byteStart, highlight.range.start)).length;
                                        let charEnd = charStart + textDecoder.decode(decoded.slice(highlight.range.start, highlight.range.end)).length;
                                        newHighlights.push({ ...highlight, range: { start: charStart, end: charEnd } });
                                        charStart = charEnd;
                                        byteStart = highlight.range.end;
                                    }
                                    block.highlights = newHighlights;
                                }
                            } catch {
                                block.data = decoded;
                            }
                        }
                    }
                }
            }
        }
        return result;
    }

    async random_suggestions(amount: number = 5): Promise<String[]> {
        return JSON.parse(await this.random_suggestions_impl(amount));
    }

    async set_suggestions_streams(streams: Uint8Array[]): Promise<void> {
        await this.clear_suggestions_stream_impl();
        for (const stream of streams) {
            await this.push_suggestions_stream_impl(stream);
        }
    }

    async suggestions(command: string, amount: number = 5): Promise<String[]> {
        return JSON.parse(await this.suggestions_impl(command, amount));
    }
}

export type I18n = {
    locale?: string;
    timezone?: string;
};
