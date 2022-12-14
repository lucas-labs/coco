#!/usr/bin/env node

import { render } from 'ink';
import React from 'react';
import { getConfig } from './common/config/coco.config';
import { repoPath } from './common/git/commands/repoPath';
import { listStaged } from './common/git/commands/list-staged';
import { i18n, LoadDictonary } from './common/i18n/i18n';
import { CocoApp } from './views/CocoApp';
import c from 'chalk';

run();

async function run() {
    const currentRepo = await repoPath('.')
    await LoadDictonary();

    // check if we are inside a git repo
    if (currentRepo) {
        const config = getConfig(currentRepo);

        // check repo status
        const staged = await listStaged('.');

        if (staged.length > 0) {
			// enters alternate screen buffer
            process.stdout.write(`\x1b[?1049h`);

            const { unmount, waitUntilExit } = render(<CocoApp cfg={config} />, {
                exitOnCtrlC: false,
                patchConsole: false,
            });

            process.on('exit', () => kill(unmount));

            waitUntilExit().then(() => {
                kill(unmount);
            });
        } else {
            // nothing to commit
            console.log(
                `${c.bold.red(
                    i18n(`Nothing to commit! Stage your changes first ('git add .')`)
                )}\n`
            );
        }
    } else {
        console.log(`${c.bold.red(i18n('Not a git repository'))}\n`);
    }
}

function kill(unmount: (error?: number | Error | null | undefined) => void) {
    unmount();

	// exits alternate screen buffer
    process.stdout.write('\x1b[?1049l');
}
