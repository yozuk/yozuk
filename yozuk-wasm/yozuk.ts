import { decode } from 'base64-arraybuffer';
import { Result } from './output'

export abstract class YozukBase {
    protected abstract exec_impl(command: string, i18n: string): Promise<string>;
    protected abstract push_stream_impl(stream: Uint8Array): Promise<void>;
    protected abstract random_suggests_impl(amount: number): Promise<string>;
    protected abstract push_suggests_stream_impl(stream: Uint8Array): Promise<void>;
    protected abstract clear_suggests_stream_impl(): Promise<void>;
    protected abstract suggests_impl(amount: number, command: string): Promise<string>;
    protected abstract i18n(): I18n;

    async exec(command: string, streams: Uint8Array[] = []): Promise<Result> {
        for (const stream of streams) {
            await this.push_stream_impl(stream);
        }
        const result = JSON.parse(await this.exec_impl(command, JSON.stringify(this.i18n())));
        const textDecoder = new TextDecoder('utf-8', { fatal: true });
        if (result.outputs) {
            result.outputs.forEach((output: any) => {
                output.blocks.forEach((block: any) => {
                    const { data } = block;
                    if (data) {
                        const decoded = decode(data);
                        try {
                            block.data = textDecoder.decode(decoded);
                        } catch {
                            block.data = decoded;
                        }
                    }
                });
            });
        }
        return result;
    }

    async random_suggests(amount: number = 5): Promise<String[]> {
        return JSON.parse(await this.random_suggests_impl(amount));
    }

    async set_suggests_streams(streams: Uint8Array[]): Promise<void> {
        await this.clear_suggests_stream_impl();
        for (const stream of streams) {
            await this.push_suggests_stream_impl(stream);
        }
    }

    async suggests(command: string, amount: number = 5): Promise<String[]> {
        return JSON.parse(await this.suggests_impl(amount, command));
    }
}

export type I18n = {
    locale?: string;
    timezone?: string;
};
