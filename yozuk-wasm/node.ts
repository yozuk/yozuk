import { exec, random_suggests, push_stream } from './wasm-node/yozuk_wasm'
import { YozukBase, I18n } from './yozuk'

export class Yozuk extends YozukBase {
    exec_impl(command: string, i18n: string): Promise<string> {
        return Promise.resolve(exec(command, i18n));
    }

    push_stream_impl(stream: Uint8Array): Promise<void> {
        return Promise.resolve(push_stream(stream));
    }

    random_suggests_impl(amount: number): Promise<string> {
        return Promise.resolve(random_suggests(amount));
    }

    i18n(): I18n {
        return {
            timezone: Intl.DateTimeFormat().resolvedOptions().timeZone
        };
    }
}
