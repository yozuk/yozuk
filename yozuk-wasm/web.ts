import init, { exec, random_suggestions, push_stream, push_suggestions_stream, clear_suggestions_stream, suggestions } from './wasm-web/yozuk_wasm'
import { YozukBase, I18n } from './yozuk'

let initialized: boolean = false;

async function init_once() {
    if (!initialized) {
        await init()
        initialized = true;
    }
}

export class Yozuk extends YozukBase {
    protected exec_impl(command: string, i18n: string): Promise<string> {
        return init_once().then(() => exec(command, i18n));
    }

    protected push_stream_impl(stream: Uint8Array): Promise<void> {
        return init_once().then(() => push_stream(stream));
    }

    protected random_suggestions_impl(amount: number): Promise<string> {
        return init_once().then(() => random_suggestions(amount));
    }

    protected push_suggestions_stream_impl(stream: Uint8Array): Promise<void> {
        return init_once().then(() => push_suggestions_stream(stream));
    }

    protected clear_suggestions_stream_impl(): Promise<void> {
        return init_once().then(() => clear_suggestions_stream());
    }

    protected suggestions_impl(command: string, amount: number): Promise<string> {
        return init_once().then(() => suggestions(command, amount));
    }

    i18n(): I18n {
        return {
            locale: navigator.language,
            timezone: Intl.DateTimeFormat().resolvedOptions().timeZone
        };
    }
}
